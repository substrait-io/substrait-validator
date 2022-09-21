// SPDX-License-Identifier: Apache-2.0

//! Module for parsing type derivations using ANTLR.

// TODO
#![allow(dead_code)]

mod substraittypelexer;
mod substraittypelistener;
mod substraittypeparser;

use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::extension::simple::module::DynScope;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::output::type_system::meta::Pattern;
use crate::parse::context;
use crate::util;
use antlr_rust::Parser;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::Arc;
use strum::IntoEnumIterator;
use substraittypeparser::*;

/// Enum for objects that are defined locally in analysis scope.
#[derive(Clone, Debug)]
enum PatternObject {
    /// A named binding.
    NamedBinding(String),

    /// A variant name of an enumeration defined as a parameter of a
    /// user-defined type class that is within scope (type has been
    /// used).
    EnumVariant(String),

    /// A type class.
    TypeClass(data::Class),
}

/// Context/state information used while analyzing type patterns and
/// derivations. The lifetime of the context should match the lifetime of
/// the evaluation context; for functions, for example, this means that the
/// same context must be used for all argument patterns, the intermediate
/// type derivation (if any), and the return type derivation.
pub struct AnalysisContext<'a> {
    /// The scope that we ultimately use to resolve names while analyzing.
    scope: Option<&'a dyn DynScope>,

    /// Names defined locally. This namespace can reference type classes,
    /// type parameter enumeration names, and named bindings. The keys are
    /// stored in lowercase for case insensitive matching.
    pattern_names: HashMap<String, PatternObject>,
}

impl<'a> Clone for AnalysisContext<'a> {
    fn clone(&self) -> Self {
        Self {
            scope: self.scope,
            pattern_names: self.pattern_names.clone(),
        }
    }
}

impl<'a> AnalysisContext<'a> {
    /// Makes a new analysis context from the given resolver, representing the
    /// scope in which the type patterns/derivations are analyzed.
    pub fn new(scope: Option<&'a dyn DynScope>) -> Self {
        // Declare built-in type classes.
        let mut pattern_names = HashMap::new();
        for simple in data::class::Simple::iter() {
            pattern_names.insert(
                simple.to_string().to_ascii_lowercase(),
                PatternObject::TypeClass(data::Class::Simple(simple)),
            );
        }
        for compound in data::class::Compound::iter() {
            pattern_names.insert(
                compound.to_string().to_ascii_lowercase(),
                PatternObject::TypeClass(data::Class::Compound(compound)),
            );
        }

        Self {
            scope,
            pattern_names,
        }
    }

    /// Resolve a local identifier path. This always succeeds, because if the
    /// name wasn't already defined, it will implicitly become a named binding.
    /// Resolving a path to a type class may also have side effects; if the
    /// type class is user-defined and has enumeration type parameters, those
    /// names will be implicitly defined as being enum variants.
    fn resolve_pattern<S, I>(&mut self, x: I, y: &mut context::Context) -> PatternObject
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let path = x.collect::<Vec<_>>();
        let name = path.iter().map(|x| x.as_ref()).join(".");
        let key = name.to_ascii_lowercase();

        // If this object was used before, return it as it was originally
        // implicitly declared.
        if let Some(object) = self.pattern_names.get(&key) {
            return object.clone();
        }

        // Try resolving as a user-definded type class.
        let object = if let Some(type_class) = self.scope.as_ref().and_then(|x| {
            x.resolve_type_class_from_ref(name.clone().into())
                .expect_not_ambiguous(y, |_, _| false)
                .as_opt_item()
        }) {
            // If the type class is known to have enum parameters, declare
            // references to the variant names so we can use them.
            if let Some(def) = &type_class.definition {
                for slot in def.parameter_slots.iter() {
                    if let meta::pattern::Value::Enum(Some(variants)) = &slot.pattern {
                        for variant in variants {
                            self.pattern_names.insert(
                                variant.to_ascii_lowercase(),
                                PatternObject::EnumVariant(variant.clone()),
                            );
                        }
                    }
                }
            }

            PatternObject::TypeClass(data::Class::UserDefined(type_class))
        } else {
            PatternObject::NamedBinding(name)
        };

        // Declare the new reference and return the referred object.
        self.pattern_names.insert(key, object.clone());
        object
    }

    /// Resolve a type variation identifier path.
    pub fn resolve_type_variation<S, I>(
        &mut self,
        x: I,
        y: &mut context::Context,
        class: &data::Class,
    ) -> extension::simple::type_variation::Reference
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let path = x.collect::<Vec<_>>();
        let name = path.iter().map(|x| x.as_ref()).join(".");

        self.scope
            .as_ref()
            .map(|x| {
                x.resolve_type_variation_from_ref(name.clone().into())
                    .filter_items(|x| &x.base == class)
                    .expect_one(
                        y,
                        |x, y| {
                            diagnostic!(
                                y,
                                Error,
                                LinkMissingTypeVariationNameAndClass,
                                "{x} exists, but is not a variation of {class} data types"
                            );
                            true
                        },
                        |_, _| false,
                    )
                    .as_item()
            })
            .unwrap_or_else(|| Arc::new(name.into()))
    }
}

/// Error listener that just collects error messages into a vector, such that
/// they can be obtained when parsing completes.
#[derive(Default, Clone)]
struct ErrorListener {
    messages: Rc<RefCell<Vec<String>>>,
}

impl<'a, T: antlr_rust::recognizer::Recognizer<'a>> antlr_rust::error_listener::ErrorListener<'a, T>
    for ErrorListener
{
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<<T>::TF as antlr_rust::token_factory::TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&antlr_rust::errors::ANTLRError>,
    ) {
        self.messages
            .borrow_mut()
            .push(format!("at {line}:{column}: {msg}"));
    }
}

impl ErrorListener {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_context(&self, y: &mut context::Context) {
        for message in self.messages.borrow_mut().drain(..) {
            diagnostic!(y, Error, TypeParseError, "{message}");
        }
    }
}

// Boilerplate code for connecting ANTLR to our diagnostic system and parsing
// a simple string slice with it.
macro_rules! antlr_parse {
    ($x:expr, $y:expr, $start:ident) => {{
        let lexer = substraittypelexer::SubstraitTypeLexer::new(antlr_rust::InputStream::new($x));
        let token_source = antlr_rust::common_token_stream::CommonTokenStream::new(lexer);
        let mut parser = SubstraitTypeParser::new(token_source);
        let listener = ErrorListener::new();
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(listener.clone()));
        let result = parser.$start();
        listener.to_context($y);
        result.map_err(|e| cause!(TypeParseError, "{e}"))
    }};
}

// Boilerplate code for converting the awkward left-recursion-avoidance rules
// for expressions into a normal expression tree.
macro_rules! antlr_reduce_left_recursion {
    (
        $x:expr, $y:expr, $z:expr, $x_typ:ty,
        $all_operands:ident, $next_analyzer:expr,
        $one_operator:ident, $operator_match:tt
    ) => {{
        fn left_recursive(
            x: &$x_typ,
            y: &mut context::Context,
            z: &mut AnalysisContext,
            start: usize,
        ) -> Result<meta::pattern::Value> {
            if start == 0 {
                // Only one operand remaining.
                Ok(antlr_hidden_child!(x, y, 0, $next_analyzer, z).unwrap_or_default())
            } else {
                // We're traversing the tree bottom-up, so start with the last
                // operation. The operations are evaluated left-to-right, so that's
                // the rightmost operation.
                let lhs = antlr_recurse!(x, y, lhs, left_recursive, z, start - 1)
                    .1
                    .unwrap_or_default();
                let rhs = antlr_child!(x, y, rhs, start, $next_analyzer, z)
                    .1
                    .unwrap_or_default();
                let arguments = vec![lhs, rhs];
                let function = x
                    .$one_operator(start - 1)
                    .map(|x| match x.as_ref() $operator_match)
                    .unwrap_or_default();
                check_function(y, z, &function, &arguments);
                Ok(meta::pattern::Value::Function(function, arguments))
            }
        }

        let total_operands = $x.$all_operands().len();
        left_recursive($x, $y, $z, total_operands - 1)
    }};
}

/// Helper function for check_function() to check normal arguments.
fn check_function_argument(
    y: &mut context::Context,
    _z: &mut AnalysisContext,
    function: &meta::Function,
    slot_index: usize,
    argument: &meta::pattern::Value,
    expected: &meta::Type,
) {
    if !argument.can_evaluate() {
        diagnostic!(
            y,
            Error,
            TypeDerivationInvalid,
            "{} argument of {function}() can never be evaluated",
            util::string::describe_nth((slot_index + 1) as u32)
        );
    } else if expected != &meta::Type::Unresolved {
        let arg_type = argument.determine_type();
        if arg_type != meta::Type::Unresolved && &arg_type != expected {
            diagnostic!(
                y,
                Error,
                TypeDerivationInvalid,
                "{} argument of {function}() must be of type {expected}, \
                but pattern always evaluates to {arg_type}",
                util::string::describe_nth((slot_index + 1) as u32)
            );
        }
    }
}

/// Helper function for check_function() to check normal functions against a
/// prototype.
fn check_function_fixed_prototype(
    y: &mut context::Context,
    z: &mut AnalysisContext,
    function: &meta::Function,
    arguments: &[meta::pattern::Value],
    expected: &[meta::Type],
) {
    if arguments.len() != expected.len() {
        diagnostic!(
            y,
            Error,
            TypeDerivationInvalid,
            "{function}() expects {} argument(s) but received {}",
            expected.len(),
            arguments.len()
        );
    } else {
        for (slot_index, (argument, expected)) in arguments.iter().zip(expected.iter()).enumerate()
        {
            check_function_argument(y, z, function, slot_index, argument, expected)
        }
    }
}

struct VariadicRange {
    pub min: usize,
    pub max: usize,
}

impl std::fmt::Display for VariadicRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.min, self.max) {
            (0, usize::MAX) => write!(f, "any number of arguments"),
            (1, usize::MAX) => write!(f, "one or more arguments"),
            (x, usize::MAX) => write!(f, "{x} or more arguments"),
            (0, 1) => write!(f, "zero or one argument"),
            (1, 1) => write!(f, "exactly one argument"),
            (x, y) => {
                if x == y {
                    write!(f, "exactly {x} arguments")
                } else {
                    write!(f, "{x} to {y} arguments")
                }
            }
        }
    }
}

impl VariadicRange {
    fn at_least(min: usize) -> Self {
        Self {
            min,
            max: usize::MAX,
        }
    }

    fn exactly(n: usize) -> Self {
        Self { min: n, max: n }
    }

    fn contains(&self, n: usize) -> bool {
        n >= self.min && n <= self.max
    }
}

/// Helper function for check_function() to check variadic functions against a
/// prototype.
fn check_function_variadic_prototype(
    y: &mut context::Context,
    z: &mut AnalysisContext,
    function: &meta::Function,
    arguments: &[meta::pattern::Value],
    expected: &meta::Type,
    validator_nargs: VariadicRange,
    substrait_nargs: VariadicRange,
) {
    if !validator_nargs.contains(arguments.len()) {
        diagnostic!(
            y,
            Error,
            TypeDerivationInvalid,
            "{function}() expects {validator_nargs} but received {}",
            arguments.len()
        );
    } else {
        if !substrait_nargs.contains(arguments.len()) {
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "{function}() officially only supports {substrait_nargs} but received {}",
                arguments.len()
            );
        }
        for (slot_index, argument) in arguments.iter().enumerate() {
            check_function_argument(y, z, function, slot_index, argument, expected)
        }
    }
}

/// Sanity-checks/lints a new function invocation.
fn check_function(
    y: &mut context::Context,
    z: &mut AnalysisContext,
    function: &meta::Function,
    arguments: &[meta::pattern::Value],
) {
    match function {
        meta::Function::Unresolved => (),
        meta::Function::Not => {
            check_function_fixed_prototype(y, z, function, arguments, &[meta::Type::Boolean]);
        }
        meta::Function::And | meta::Function::Or => {
            check_function_variadic_prototype(
                y,
                z,
                function,
                arguments,
                &meta::Type::Boolean,
                VariadicRange::at_least(0),
                VariadicRange::exactly(2),
            );
        }
        meta::Function::Negate => {
            check_function_fixed_prototype(y, z, function, arguments, &[meta::Type::Integer]);
        }
        meta::Function::Add | meta::Function::Multiply => {
            check_function_variadic_prototype(
                y,
                z,
                function,
                arguments,
                &meta::Type::Integer,
                VariadicRange::at_least(0),
                VariadicRange::exactly(2),
            );
        }
        meta::Function::Subtract | meta::Function::Divide => {
            check_function_fixed_prototype(
                y,
                z,
                function,
                arguments,
                &[meta::Type::Integer, meta::Type::Integer],
            );
        }
        meta::Function::Min | meta::Function::Max => {
            check_function_variadic_prototype(
                y,
                z,
                function,
                arguments,
                &meta::Type::Integer,
                VariadicRange::at_least(1),
                VariadicRange::exactly(2),
            );
        }
        meta::Function::Equal => {
            check_function_fixed_prototype(
                y,
                z,
                function,
                arguments,
                &[meta::Type::Unresolved, meta::Type::Unresolved],
            );
        }
        meta::Function::NotEqual => {
            check_function_fixed_prototype(
                y,
                z,
                function,
                arguments,
                &[meta::Type::Unresolved, meta::Type::Unresolved],
            );
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "{function}() is not officially supported"
            );
        }
        meta::Function::GreaterThan | meta::Function::LessThan => {
            check_function_fixed_prototype(
                y,
                z,
                function,
                arguments,
                &[meta::Type::Integer, meta::Type::Integer],
            );
        }
        meta::Function::LessEqual | meta::Function::GreaterEqual => {
            check_function_fixed_prototype(
                y,
                z,
                function,
                arguments,
                &[meta::Type::Integer, meta::Type::Integer],
            );
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "{function}() is not officially supported"
            );
        }
        meta::Function::Covers => {
            if arguments.len() != 2 {
                diagnostic!(
                    y,
                    Error,
                    TypeDerivationInvalid,
                    "{function}() expects two arguments but received {}",
                    arguments.len()
                );
            } else {
                if !arguments[0].can_evaluate() {
                    diagnostic!(
                        y,
                        Error,
                        TypeDerivationInvalid,
                        "first argument of covers() can never be evaluated"
                    );
                }
                match (arguments[0].determine_type(), arguments[1].determine_type()) {
                    (meta::Type::Unresolved, _) => (),
                    (_, meta::Type::Unresolved) => (),
                    (a, b) => {
                        if a != b {
                            diagnostic!(
                                y,
                                Info,
                                TypeDerivationInvalid,
                                "second argument of covers() can never match first; \
                                second matches only {b} while first returns {a}"
                            );
                        }
                    }
                }
            }
        }
        meta::Function::IfThenElse => {
            check_function_fixed_prototype(
                y,
                z,
                function,
                arguments,
                &[
                    meta::Type::Boolean,
                    meta::Type::Unresolved,
                    meta::Type::Unresolved,
                ],
            );
        }
    }
}

/// Analyzes a string literal.
fn analyze_string<S: AsRef<str>>(
    x: S,
    _y: &mut context::Context,
    _z: &mut AnalysisContext,
) -> Option<String> {
    let x = x.as_ref();
    if !x.starts_with('"') || !x.ends_with('"') || x.len() < 2 {
        None
    } else {
        Some(x[1..x.len() - 1].to_string())
    }
}

/// Analyzes an integer literal.
fn analyze_integer(
    x: &IntegerContextAll,
    y: &mut context::Context,
    _z: &mut AnalysisContext,
) -> Result<i64> {
    let value = if let Some(value) = x.Nonzero() {
        value.symbol.text.parse().unwrap_or(i128::MAX)
    } else {
        0i128
    };
    Ok(if x.Minus().is_some() {
        match i64::try_from(-value) {
            Ok(val) => val,
            Err(_) => {
                diagnostic!(
                    y,
                    Error,
                    TypeDerivationInvalid,
                    "integer literal is too small, minimum is {}",
                    i64::MIN
                );
                i64::MIN
            }
        }
    } else {
        match i64::try_from(value) {
            Ok(val) => val,
            Err(_) => {
                diagnostic!(
                    y,
                    Error,
                    TypeDerivationInvalid,
                    "integer literal is too large, maximum is {}",
                    i64::MAX
                );
                i64::MAX
            }
        }
    })
}

/// Analyzes and resolves an identifier path.
fn analyze_object_identifier(
    x: &IdentifierPathContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<PatternObject> {
    Ok(z.resolve_pattern(
        x.Identifier_all().iter().map(|x| x.symbol.text.to_string()),
        y,
    ))
}

/// Analyzes a pattern that can end up being either a data type pattern, a
/// binding, or an enum constant, depending on name resolution.
fn analyze_dtbc(
    x: &DatatypeBindingOrConstantContext,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    let object = antlr_hidden_child!(x, y, 0, analyze_object_identifier, z)
        .ok_or_else(|| cause!(TypeDerivationInvalid, "failed to resolve identifier"))?;
    match object {
        PatternObject::NamedBinding(name) => {
            if x.variation().is_some() {
                diagnostic!(
                    y,
                    Error,
                    TypeParseError,
                    "variation cannot be specified for bindings"
                );
            }
            if x.parameters().is_some() {
                diagnostic!(
                    y,
                    Error,
                    TypeParseError,
                    "parameters cannot be specified for bindings"
                );
            }
            let nullability = x.nullability().map(|nullability| {
                Arc::new(match nullability.as_ref() {
                    NullabilityContextAll::NullableContext(_) => {
                        meta::pattern::Value::Boolean(Some(true))
                    }
                    NullabilityContextAll::NonNullableContext(_) => {
                        meta::pattern::Value::Boolean(Some(false))
                    }
                    NullabilityContextAll::NullableIfContext(x) => {
                        let pattern = antlr_child!(x, y, nullability, 0, analyze_pattern, z)
                            .1
                            .unwrap_or_default();
                        let typ = pattern.determine_type();
                        if !matches!(typ, meta::Type::Boolean | meta::Type::Unresolved) {
                            diagnostic!(
                                y,
                                Error,
                                TypeDerivationInvalid,
                                "nullability pattern must be boolean, but {typ} was found"
                            );
                        }
                        pattern
                    }
                    NullabilityContextAll::Error(_) => meta::pattern::Value::Unresolved,
                })
            });
            if nullability.is_some() {
                diagnostic!(
                    y,
                    Error,
                    TypeDerivationNotSupported,
                    "bindings with nullability override are not officially supported"
                );
            }
            Ok(meta::pattern::Value::Binding(meta::pattern::Binding {
                name,
                inconsistent: false,
                nullability,
            }))
        }
        PatternObject::EnumVariant(name) => {
            if x.nullability().is_some() {
                diagnostic!(
                    y,
                    Error,
                    TypeParseError,
                    "nullability cannot be specified for enum literals"
                );
            }
            if x.variation().is_some() {
                diagnostic!(
                    y,
                    Error,
                    TypeParseError,
                    "variation cannot be specified for enum literals"
                );
            }
            if x.parameters().is_some() {
                diagnostic!(
                    y,
                    Error,
                    TypeParseError,
                    "parameters cannot be specified for enum literals"
                );
            }
            Ok(meta::pattern::Value::Enum(Some(vec![name])))
        }
        PatternObject::TypeClass(class) => {
            let nullable = if let Some(nullability) = x.nullability() {
                match nullability.as_ref() {
                    NullabilityContextAll::NullableContext(_) => {
                        meta::pattern::Value::Boolean(Some(true))
                    }
                    NullabilityContextAll::NonNullableContext(_) => {
                        diagnostic!(
                            y,
                            Error,
                            TypeDerivationNotSupported,
                            "explicitly-non-nullable type suffix is not officially supported"
                        );
                        meta::pattern::Value::Boolean(Some(false))
                    }
                    NullabilityContextAll::NullableIfContext(x) => {
                        let pattern = antlr_child!(x, y, nullability, 0, analyze_pattern, z)
                            .1
                            .unwrap_or_default();
                        let typ = pattern.determine_type();
                        if !matches!(typ, meta::Type::Boolean | meta::Type::Unresolved) {
                            diagnostic!(
                                y,
                                Error,
                                TypeDerivationInvalid,
                                "nullability pattern must be boolean, but {typ} was found"
                            );
                        }
                        diagnostic!(
                            y,
                            Error,
                            TypeDerivationNotSupported,
                            "nullability patterns are not officially supported"
                        );
                        pattern
                    }
                    NullabilityContextAll::Error(_) => meta::pattern::Value::Unresolved,
                }
            } else {
                meta::pattern::Value::Boolean(Some(false))
            };
            let variation = antlr_child!(x, y, variation, 0, analyze_type_variation, z, &class)
                .1
                .unwrap_or(meta::pattern::Variation::Compatible);
            let parameters = antlr_child!(x, y, parameters, 0, analyze_type_parameters, z).1;
            Ok(meta::pattern::Value::DataType(meta::pattern::DataType {
                class: Some(class),
                nullable: std::sync::Arc::new(nullable),
                variation,
                parameters,
            }))
        }
    }
}

/// Analyzes a type variation suffix.
fn analyze_type_variation_identifier(
    x: &IdentifierPathContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
    class: &data::Class,
) -> Result<extension::simple::type_variation::Reference> {
    Ok(z.resolve_type_variation(
        x.Identifier_all().iter().map(|x| x.symbol.text.to_string()),
        y,
        class,
    ))
}

/// Analyzes a type variation suffix.
fn analyze_type_variation(
    x: &VariationContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
    class: &data::Class,
) -> Result<meta::pattern::Variation> {
    Ok(if let Some(x) = x.variationBody() {
        match x.as_ref() {
            VariationBodyContextAll::VarAnyContext(_) => meta::pattern::Variation::Any,
            VariationBodyContextAll::VarSystemPreferredContext(_) => {
                meta::pattern::Variation::Exactly(data::Variation::SystemPreferred)
            }
            VariationBodyContextAll::VarUserDefinedContext(x) => {
                meta::pattern::Variation::Exactly(data::Variation::UserDefined(
                    antlr_child!(
                        x,
                        y,
                        variation,
                        0,
                        analyze_type_variation_identifier,
                        z,
                        class
                    )
                    .1
                    .unwrap_or_default(),
                ))
            }
            VariationBodyContextAll::Error(_) => meta::pattern::Variation::Any,
        }
    } else {
        meta::pattern::Variation::Any
    })
}

/// Analyzes a single type parameter.
fn analyze_type_parameter(
    x: &ParameterContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Parameter> {
    let name = x.identifierOrString().and_then(|name| {
        let name = match name.as_ref() {
            IdentifierOrStringContextAll::StrContext(x) => x
                .String()
                .and_then(|x| analyze_string(&x.symbol.text, y, z))
                .unwrap_or_default(),
            IdentifierOrStringContextAll::IdentContext(x) => x
                .Identifier()
                .map(|x| x.symbol.text.to_string())
                .unwrap_or_default(),
            IdentifierOrStringContextAll::Error(_) => String::from(""),
        };
        diagnostic!(
            y,
            Error,
            TypeDerivationNotSupported,
            "named parameters are not officially supported (nstruct is not a real type)"
        );
        if name.is_empty() {
            diagnostic!(
                y,
                Error,
                TypeInvalidFieldName,
                "parameter names (if specified) cannot be empty"
            );
            None
        } else {
            Some(name)
        }
    });

    let value = if let Some(value) = x.parameterValue() {
        match value.as_ref() {
            ParameterValueContextAll::SpecifiedContext(x) => Some(
                antlr_child!(x, y, pattern, 0, analyze_pattern, z)
                    .1
                    .unwrap_or_default(),
            ),
            ParameterValueContextAll::NullContext(_) => None,
            ParameterValueContextAll::Error(_) => Some(meta::pattern::Value::Unresolved),
        }
    } else {
        Some(meta::pattern::Value::Unresolved)
    };

    Ok(meta::pattern::Parameter { name, value })
}

/// Analyzes a type parameter pack.
fn analyze_type_parameters(
    x: &ParametersContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<Vec<meta::pattern::Parameter>> {
    Ok(antlr_children!(x, y, argument, analyze_type_parameter, z)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect())
}

/// Analyzes miscellaneous pattern types.
fn analyze_pattern_misc(
    x: &PatternMiscContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    match x {
        PatternMiscContextAll::ParenthesesContext(x) => {
            Ok(antlr_hidden_child!(x, y, 0, analyze_pattern, z).unwrap_or_default())
        }
        PatternMiscContextAll::IfThenElseContext(x) => {
            let condition = antlr_child!(x, y, condition, 0, analyze_pattern, z)
                .1
                .unwrap_or_default();
            let if_true = antlr_child!(x, y, if_true, 1, analyze_pattern, z)
                .1
                .unwrap_or_default();
            let if_false = antlr_child!(x, y, if_false, 2, analyze_pattern, z)
                .1
                .unwrap_or_default();
            let arguments = vec![condition, if_true, if_false];
            let function = meta::Function::IfThenElse;
            check_function(y, z, &function, &arguments);
            Ok(meta::pattern::Value::Function(function, arguments))
        }
        PatternMiscContextAll::UnaryNotContext(x) => {
            let expression = antlr_child!(x, y, expression, 0, analyze_pattern, z)
                .1
                .unwrap_or_default();
            let arguments = vec![expression];
            let function = meta::Function::Not;
            check_function(y, z, &function, &arguments);
            Ok(meta::pattern::Value::Function(function, arguments))
        }
        PatternMiscContextAll::UnaryNegateContext(x) => {
            let expression = antlr_child!(x, y, expression, 0, analyze_pattern, z)
                .1
                .unwrap_or_default();
            let arguments = vec![expression];
            let function = meta::Function::Negate;
            check_function(y, z, &function, &arguments);
            Ok(meta::pattern::Value::Function(function, arguments))
        }
        PatternMiscContextAll::AnyContext(_) => {
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the 'any' pattern is not officially supported"
            );
            Ok(meta::pattern::Value::Any)
        }
        PatternMiscContextAll::BoolAnyContext(_) => {
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the 'any boolean' pattern is not officially supported"
            );
            Ok(meta::pattern::Value::Boolean(None))
        }
        PatternMiscContextAll::BoolTrueContext(_) => Ok(meta::pattern::Value::Boolean(Some(true))),
        PatternMiscContextAll::BoolFalseContext(_) => {
            Ok(meta::pattern::Value::Boolean(Some(false)))
        }
        PatternMiscContextAll::IntAnyContext(_) => {
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the 'any integer' pattern is not officially supported"
            );
            Ok(meta::pattern::Value::Integer(i64::MIN, i64::MAX))
        }
        PatternMiscContextAll::IntRangeContext(x) => {
            let lower = antlr_hidden_child!(x, y, 0, analyze_integer, z).unwrap_or(i64::MIN);
            let upper = antlr_hidden_child!(x, y, 1, analyze_integer, z).unwrap_or(i64::MAX);
            if lower > upper {
                diagnostic!(
                    y,
                    Error,
                    TypeDerivationInvalid,
                    "lower bound of range is greater than upper bound"
                );
            }
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the integer range pattern is not officially supported"
            );
            Ok(meta::pattern::Value::Integer(lower, upper))
        }
        PatternMiscContextAll::IntAtMostContext(x) => {
            let upper = antlr_hidden_child!(x, y, 0, analyze_integer, z).unwrap_or(i64::MAX);
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the integer range pattern is not officially supported"
            );
            Ok(meta::pattern::Value::Integer(i64::MIN, upper))
        }
        PatternMiscContextAll::IntAtLeastContext(x) => {
            let lower = antlr_hidden_child!(x, y, 0, analyze_integer, z).unwrap_or(i64::MAX);
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the integer range pattern is not officially supported"
            );
            Ok(meta::pattern::Value::Integer(lower, i64::MAX))
        }
        PatternMiscContextAll::IntExactlyContext(x) => {
            let value = antlr_hidden_child!(x, y, 0, analyze_integer, z).unwrap_or(i64::MAX);
            Ok(meta::pattern::Value::Integer(value, value))
        }
        PatternMiscContextAll::EnumAnyContext(_) => {
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the 'any enumeration' pattern is not officially supported"
            );
            Ok(meta::pattern::Value::Enum(None))
        }
        PatternMiscContextAll::EnumSetContext(x) => {
            let names = x
                .Identifier_all()
                .iter()
                .map(|x| x.symbol.text.to_string())
                .collect::<Vec<_>>();
            let mut unique_names = HashSet::new();
            let mut repeated_names = HashSet::new();
            for name in names.iter() {
                if !unique_names.insert(name.to_ascii_uppercase()) {
                    repeated_names.insert(name.to_ascii_uppercase());
                }
            }
            if !repeated_names.is_empty() {
                diagnostic!(
                    y,
                    Error,
                    RedundantEnumVariant,
                    "enumeration variant names should be case-insensitively unique: {}",
                    repeated_names.iter().join(", ")
                );
            }
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the enum set pattern is not officially supported"
            );
            Ok(meta::pattern::Value::Enum(Some(names)))
        }
        PatternMiscContextAll::StrAnyContext(_) => {
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the 'any string' pattern is not officially supported"
            );
            Ok(meta::pattern::Value::String(None))
        }
        PatternMiscContextAll::StrExactlyContext(x) => {
            let s = x
                .String()
                .and_then(|x| analyze_string(&x.symbol.text, y, z));
            if s.is_none() {
                diagnostic!(y, Error, TypeParseError, "invalid string literal");
            }
            Ok(meta::pattern::Value::String(s))
        }
        PatternMiscContextAll::DtAnyContext(x) => {
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "the 'any data type' pattern is not officially supported"
            );
            let nullable = if let Some(nullability) = x.nullability() {
                match nullability.as_ref() {
                    NullabilityContextAll::NullableContext(_) => {
                        meta::pattern::Value::Boolean(Some(true))
                    }
                    NullabilityContextAll::NonNullableContext(_) => {
                        meta::pattern::Value::Boolean(Some(false))
                    }
                    NullabilityContextAll::NullableIfContext(x) => {
                        let pattern = antlr_child!(x, y, nullability, 0, analyze_pattern, z)
                            .1
                            .unwrap_or_default();
                        let typ = pattern.determine_type();
                        if !matches!(typ, meta::Type::Boolean | meta::Type::Unresolved) {
                            diagnostic!(
                                y,
                                Error,
                                TypeDerivationInvalid,
                                "nullability pattern must be boolean, but {typ} was found"
                            );
                        }
                        pattern
                    }
                    NullabilityContextAll::Error(_) => meta::pattern::Value::Unresolved,
                }
            } else {
                meta::pattern::Value::Boolean(Some(false))
            };
            Ok(meta::pattern::Value::DataType(meta::pattern::DataType {
                class: None,
                nullable: Arc::new(nullable),
                variation: meta::pattern::Variation::Compatible,
                parameters: None,
            }))
        }
        PatternMiscContextAll::FunctionContext(x) => {
            let function = x
                .Identifier()
                .and_then(|x| meta::Function::try_from(x.symbol.text.as_ref()).ok());
            if function.is_none() {
                diagnostic!(y, Error, TypeParseError, "unknown function");
            }
            let function = function.unwrap_or_default();
            let arguments = antlr_children!(x, y, argument, analyze_pattern, z)
                .1
                .into_iter()
                .map(|x| x.unwrap_or_default())
                .collect::<Vec<_>>();
            check_function(y, z, &function, &arguments);
            Ok(meta::pattern::Value::Function(function, arguments))
        }
        PatternMiscContextAll::DatatypeBindingOrConstantContext(x) => analyze_dtbc(x, y, z),
        PatternMiscContextAll::InconsistentContext(x) => {
            let name = x
                .Identifier()
                .map(|x| x.symbol.text.to_string())
                .unwrap_or_else(|| "!".to_string());

            // Check that the name actually maps to a binding.
            match z.resolve_pattern(std::iter::once(&name), y) {
                PatternObject::NamedBinding(_) => (),
                PatternObject::EnumVariant(x) => {
                    diagnostic!(
                        y,
                        Error,
                        TypeDerivationInvalid,
                        "{name} cannot be used as a binding; it maps to enum variant {x}"
                    );
                }
                PatternObject::TypeClass(x) => {
                    diagnostic!(
                        y,
                        Error,
                        TypeDerivationInvalid,
                        "{name} cannot be used as a binding; it maps to type class {x}"
                    );
                }
            }

            let nullability = x.nullability().map(|nullability| {
                Arc::new(match nullability.as_ref() {
                    NullabilityContextAll::NullableContext(_) => {
                        meta::pattern::Value::Boolean(Some(true))
                    }
                    NullabilityContextAll::NonNullableContext(_) => {
                        meta::pattern::Value::Boolean(Some(false))
                    }
                    NullabilityContextAll::NullableIfContext(x) => {
                        antlr_child!(x, y, nullability, 0, analyze_pattern, z)
                            .1
                            .unwrap_or_default()
                    }
                    NullabilityContextAll::Error(_) => meta::pattern::Value::Unresolved,
                })
            });
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "inconsistent bindings are not officially supported"
            );
            Ok(meta::pattern::Value::Binding(meta::pattern::Binding {
                name,
                inconsistent: true,
                nullability,
            }))
        }
        PatternMiscContextAll::Error(_) => Ok(meta::pattern::Value::Unresolved),
    }
}

/// Analyzes a set of zero or more a*b or a/b expressions.
fn analyze_pattern_mul_div(
    x: &PatternMulDivContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, z, PatternMulDivContextAll,
        patternMisc_all, analyze_pattern_misc, operatorMulDiv,
        {
            OperatorMulDivContextAll::MulContext(_) => meta::Function::Multiply,
            OperatorMulDivContextAll::DivContext(_) => meta::Function::Divide,
            OperatorMulDivContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more a+b or a-b expressions.
fn analyze_pattern_add_sub(
    x: &PatternAddSubContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, z, PatternAddSubContextAll,
        patternMulDiv_all, analyze_pattern_mul_div, operatorAddSub,
        {
            OperatorAddSubContextAll::AddContext(_) => meta::Function::Add,
            OperatorAddSubContextAll::SubContext(_) => meta::Function::Subtract,
            OperatorAddSubContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more integer inequality expressions.
fn analyze_pattern_ineq(
    x: &PatternIneqContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, z, PatternIneqContextAll,
        patternAddSub_all, analyze_pattern_add_sub, operatorIneq,
        {
            OperatorIneqContextAll::LtContext(_) => meta::Function::LessThan,
            OperatorIneqContextAll::LeContext(_) => meta::Function::LessEqual,
            OperatorIneqContextAll::GtContext(_) => meta::Function::GreaterThan,
            OperatorIneqContextAll::GeContext(_) => meta::Function::GreaterEqual,
            OperatorIneqContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more x==y or x!=y expressions.
fn analyze_pattern_eq_neq(
    x: &PatternEqNeqContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, z, PatternEqNeqContextAll,
        patternIneq_all, analyze_pattern_ineq, operatorEqNeq,
        {
            OperatorEqNeqContextAll::EqContext(_) => meta::Function::Equal,
            OperatorEqNeqContextAll::NeqContext(_) => meta::Function::NotEqual,
            OperatorEqNeqContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more x&&y expressions.
fn analyze_pattern_and(
    x: &PatternAndContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, z, PatternAndContextAll,
        patternEqNeq_all, analyze_pattern_eq_neq, operatorAnd,
        {
            OperatorAndContextAll::AndContext(_) => meta::Function::And,
            OperatorAndContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more x||y expressions.
fn analyze_pattern_or(
    x: &PatternOrContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, z, PatternOrContextAll,
        patternAnd_all, analyze_pattern_and, operatorOr,
        {
            OperatorOrContextAll::OrContext(_) => meta::Function::Or,
            OperatorOrContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more x||y expressions.
fn analyze_pattern_invalid_if_then_else(
    x: &PatternInvalidIfThenElseContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    match x {
        PatternInvalidIfThenElseContextAll::InvalidIfThenElseContext(x) => {
            diagnostic!(
                y,
                Error,
                TypeParseError,
                "'x ? y : z' ternary operator syntax is not supported; \
                use 'if x then y else z' instead"
            );
            let condition = antlr_child!(x, y, condition, 0, analyze_pattern_or, z)
                .1
                .unwrap_or_default();
            let if_true = antlr_child!(x, y, if_true, 1, analyze_pattern_or, z)
                .1
                .unwrap_or_default();
            let if_false = antlr_child!(x, y, if_false, 0, analyze_pattern, z)
                .1
                .unwrap_or_default();
            let arguments = vec![condition, if_true, if_false];
            let function = meta::Function::IfThenElse;
            check_function(y, z, &function, &arguments);
            Ok(meta::pattern::Value::Function(function, arguments))
        }
        PatternInvalidIfThenElseContextAll::ValidPatternContext(x) => {
            Ok(antlr_hidden_child!(x, y, 0, analyze_pattern_or, z).unwrap_or_default())
        }
        PatternInvalidIfThenElseContextAll::Error(_) => Ok(meta::pattern::Value::Unresolved),
    }
}

/// Analyzes a pattern parse tree node.
fn analyze_pattern(
    x: &PatternContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    Ok(antlr_hidden_child!(x, y, 0, analyze_pattern_invalid_if_then_else, z).unwrap_or_default())
}

/// Analyzes a statement parse tree node.
fn analyze_statement(
    x: &StatementContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::program::Statement> {
    let statement = match x {
        StatementContextAll::AssertContext(x) => {
            let rhs_expression = antlr_child!(x, y, rhs, 0, analyze_pattern, z)
                .1
                .unwrap_or_default();
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "assert statements are not officially supported"
            );
            meta::program::Statement {
                lhs_pattern: meta::pattern::Value::Boolean(Some(true)),
                rhs_expression,
            }
        }
        StatementContextAll::NormalContext(x) => {
            let rhs_expression = antlr_child!(x, y, rhs, 1, analyze_pattern, z)
                .1
                .unwrap_or_default();
            let lhs_pattern = antlr_child!(x, y, lhs, 0, analyze_pattern, z)
                .1
                .unwrap_or_default();
            let lhs_supported = match &lhs_pattern {
                meta::pattern::Value::Unresolved => true,
                meta::pattern::Value::Binding(binding) => {
                    !binding.inconsistent && binding.nullability.is_none()
                }
                _ => false,
            };
            if !lhs_supported {
                diagnostic!(
                    y, Error, TypeDerivationNotSupported,
                    "only normal bindings are officially supported at the LHS of an assignment statement"
                );
            }
            meta::program::Statement {
                lhs_pattern,
                rhs_expression,
            }
        }
        StatementContextAll::MatchContext(x) => {
            let rhs_expression = antlr_child!(x, y, rhs, 0, analyze_pattern, z)
                .1
                .unwrap_or_default();
            let lhs_pattern = antlr_child!(x, y, lhs, 1, analyze_pattern, z)
                .1
                .unwrap_or_default();
            diagnostic!(
                y,
                Error,
                TypeDerivationNotSupported,
                "assert-match statements are not officially supported"
            );
            meta::program::Statement {
                lhs_pattern,
                rhs_expression,
            }
        }
        StatementContextAll::Error(_) => return Ok(meta::program::Statement::default()),
    };
    if !statement.rhs_expression.can_evaluate() {
        diagnostic!(
            y,
            Error,
            TypeDerivationInvalid,
            "right-hand side of pattern assignment can never be evaluated"
        );
    }
    match (
        statement.lhs_pattern.determine_type(),
        statement.rhs_expression.determine_type(),
    ) {
        (meta::Type::Unresolved, _) => (),
        (_, meta::Type::Unresolved) => (),
        (lhs, rhs) => {
            if lhs != rhs {
                diagnostic!(
                    y,
                    Error,
                    TypeDerivationInvalid,
                    "pattern assignment can never match; lhs matches \
                    only {lhs} while rhs returns {rhs}"
                );
            }
        }
    }
    Ok(statement)
}

/// Analyzes a program parse tree node.
fn analyze_program(
    x: &ProgramContextAll,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::Program> {
    let statements = antlr_children!(x, y, statement, analyze_statement, z)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let expression = antlr_child!(x, y, pattern, 0, analyze_pattern, z)
        .1
        .unwrap_or_default();
    if !expression.can_evaluate() {
        diagnostic!(
            y,
            Error,
            TypeDerivationInvalid,
            "return pattern of program can never be evaluated"
        );
    }
    Ok(meta::Program {
        statements,
        expression,
    })
}

/// Parse a string as just the class part of a data type.
pub fn parse_class(
    x: &str,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<data::Class> {
    // Only accept type classes.
    if let PatternObject::TypeClass(class) = z.resolve_pattern(x.split('.'), y) {
        Ok(class)
    } else {
        Err(cause!(
            TypeResolutionError,
            "could not resolve {x} as a type class"
        ))
    }
}

/// Parse a string as a complete type.
pub fn parse_type(
    x: &str,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<data::Type> {
    let pattern = parse_pattern(x, y, z)?;
    let value = pattern.evaluate()?;
    value.get_data_type().ok_or_else(|| {
        cause!(
            TypeDerivationInvalid,
            "expected a data type, but received a pattern that evaluated to {value}"
        )
    })
}

/// Parse a string as a meta-pattern.
pub fn parse_pattern(
    x: &str,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    let x = antlr_parse!(x, y, startPattern)?;
    let result = antlr_child!(x.as_ref(), y, pattern, 0, analyze_pattern, z)
        .1
        .unwrap_or_default();
    Ok(result)
}

/// Parse a string as a meta-program.
pub fn parse_program(
    x: &str,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::Program> {
    let x = antlr_parse!(x, y, startProgram)?;
    let result = antlr_child!(x.as_ref(), y, program, 0, analyze_program, z)
        .1
        .unwrap_or_default();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::output::tree;

    #[test]
    fn test1() {
        // Boilerplate to get a parsing context (only used here as a sink for
        // diagnostics).
        let mut root = tree::Node::from(tree::NodeType::ProtoMessage("test"));
        let mut state = Default::default();
        let config = crate::Config::new();
        let mut context = context::Context::new("test", &mut root, &mut state, &config);
        let y = &mut context;

        // Boilerplate for analysis environment (mostly namespace/scope info,
        // which can be empty here).
        let mut analysis_context = AnalysisContext::new(None);
        let z = &mut analysis_context;

        // Parse argument patterns and return type program, taken from decimal
        // add because it's horrible.
        let x_slot = parse_pattern("decimal<P1,S1>", y, z).unwrap();
        let y_slot = parse_pattern("decimal<P2,S2>", y, z).unwrap();
        let return_program = parse_program(
            r#"init_scale = max(S1,S2)
            init_prec = init_scale + max(P1 - S1, P2 - S2) + 1
            min_scale = min(init_scale, 6)
            delta = init_prec - 38
            prec = min(init_prec, 38)
            scale_after_borrow = max(init_scale - delta, min_scale)
            scale = init_prec > 38 ? scale_after_borrow : init_scale
            DECIMAL<prec, scale>"#,
            y,
            z,
        )
        .unwrap();

        // Get some decimal types to use as arguments for testing; parse them
        // too (instead of using the constructors) for good measure.
        let x_binding = parse_type("decimal<20,10>", y, z).unwrap();
        let y_binding = parse_type("decimal<10,5>", y, z).unwrap();

        // Match the arguments and evaluate the return type.
        let mut eval_context = meta::Context::default();
        let c = &mut eval_context;
        assert!(x_slot
            .match_pattern_with_context(c, &x_binding.into())
            .unwrap());
        assert!(y_slot
            .match_pattern_with_context(c, &y_binding.into())
            .unwrap());
        let result = return_program.evaluate(c).unwrap();

        // Test the pretty-printer too because why not.
        assert_eq!(&result.to_string(), "DECIMAL<21, 10>");

        // Another batch of arguments.
        let x_binding = parse_type("decimal<38,38>", y, z).unwrap();
        let y_binding = parse_type("decimal<38,0>", y, z).unwrap();

        // Evaluate in new context.
        let mut eval_context = meta::Context::default();
        let c = &mut eval_context;
        assert!(x_slot
            .match_pattern_with_context(c, &x_binding.into())
            .unwrap());
        assert!(y_slot
            .match_pattern_with_context(c, &y_binding.into())
            .unwrap());
        let result = return_program.evaluate(c).unwrap();
        assert_eq!(&result.to_string(), "DECIMAL<38, 6>");
    }

    #[test]
    fn test2() {
        let mut node = tree::Node::from(tree::NodeType::ProtoMessage("test"));
        let mut state = Default::default();
        let config = crate::Config::new();
        let mut context = context::Context::new("test", &mut node, &mut state, &config);
        let mut analysis_context = AnalysisContext::new(None);

        let program = parse_program(r#"1 + 2 * 3 - 4 / 5"#, &mut context, &mut analysis_context)
            .unwrap_or_default();

        let mut eval_context = meta::Context::default();
        assert_eq!(
            program.evaluate(&mut eval_context),
            Ok(meta::Value::Integer(7))
        );
    }
}
