// SPDX-License-Identifier: Apache-2.0

//! Module for parsing type derivations using ANTLR.

// TODO
#![allow(dead_code)]

use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::extension::simple::module::DynScope;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::output::type_system::meta::Pattern;
use crate::parse::context;
use crate::util;
use antlr4rust::Parser;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use strum::IntoEnumIterator;
use substrait_antlr::substrait_type::substraittypeparser::*;

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

impl Clone for AnalysisContext<'_> {
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

impl<'a, T: antlr4rust::recognizer::Recognizer<'a>> antlr4rust::error_listener::ErrorListener<'a, T>
    for ErrorListener
{
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<<T>::TF as antlr4rust::token_factory::TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&antlr4rust::errors::ANTLRError>,
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
// a simple string slice with it. `$start` is the parser entry-rule method to
// invoke (`startRule` for expressions/derivations, `typeStatement` for plain
// types); it returns the corresponding root context on success.
macro_rules! antlr_parse {
    ($x:expr, $y:expr, $start:ident) => {{
        let lexer = substrait_antlr::substrait_type::SubstraitTypeLexer::new(
            antlr4rust::InputStream::new($x),
        );
        let token_source = antlr4rust::common_token_stream::CommonTokenStream::new(lexer);
        let mut parser = SubstraitTypeParser::new(token_source);
        let listener = ErrorListener::new();
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(listener.clone()));
        let result = parser.$start();
        listener.to_context($y);
        result.map_err(|e| cause!(TypeParseError, "{e}"))
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

/// Parses an integer literal token's text. The `Number` token already includes
/// an optional leading sign, so negatives arrive as a single token.
fn analyze_number(raw: &str, y: &mut context::Context) -> Option<i64> {
    match raw.parse::<i64>() {
        Ok(value) => Some(value),
        Err(_) => {
            diagnostic!(y, Error, TypeParseError, "invalid integer literal {raw}");
            None
        }
    }
}

/// Builds a data type pattern from a resolved class, nullability, and optional
/// parameter pack. Type variations are not expressible in the grammar, so the
/// variation is always left as the system-preferred-compatible default.
fn data_type_pattern(
    class: Option<data::Class>,
    nullable: bool,
    parameters: Option<Vec<meta::pattern::Parameter>>,
) -> meta::pattern::Value {
    meta::pattern::Value::DataType(meta::pattern::DataType {
        class,
        nullable: Arc::new(meta::pattern::Value::Boolean(Some(nullable))),
        variation: meta::pattern::Variation::Compatible,
        parameters,
    })
}

/// Resolves a bare identifier to a pattern value. Depending on name resolution
/// (see [`AnalysisContext::resolve_pattern`]) it becomes a (zero-parameter) data
/// type pattern, an enumeration constant, or a named binding. `nullable` is
/// `None` when no nullability suffix was given, `Some(true)` for a `?` suffix.
fn identifier_to_value(
    name: &str,
    nullable: Option<bool>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> meta::pattern::Value {
    match z.resolve_pattern(std::iter::once(name), y) {
        PatternObject::TypeClass(class) => {
            data_type_pattern(Some(class), nullable.unwrap_or(false), None)
        }
        PatternObject::EnumVariant(variant) => meta::pattern::Value::Enum(Some(vec![variant])),
        PatternObject::NamedBinding(binding_name) => {
            meta::pattern::Value::Binding(meta::pattern::Binding {
                name: binding_name,
                inconsistent: false,
                nullability: nullable
                    .map(|n| Arc::new(meta::pattern::Value::Boolean(Some(n)))),
            })
        }
    }
}

/// Maps the operator token of a left-recursive `expr` binary alternative to the
/// corresponding builtin meta-function.
fn binary_operator(x: &BinaryExprContext<'_>) -> meta::Function {
    if x.And().is_some() {
        meta::Function::And
    } else if x.Or().is_some() {
        meta::Function::Or
    } else if x.Plus().is_some() {
        meta::Function::Add
    } else if x.Minus().is_some() {
        meta::Function::Subtract
    } else if x.Asterisk().is_some() {
        meta::Function::Multiply
    } else if x.ForwardSlash().is_some() {
        meta::Function::Divide
    } else if x.Lte().is_some() {
        meta::Function::LessEqual
    } else if x.Gte().is_some() {
        meta::Function::GreaterEqual
    } else if x.Lt().is_some() {
        meta::Function::LessThan
    } else if x.Gt().is_some() {
        meta::Function::GreaterThan
    } else if x.Eq().is_some() {
        meta::Function::Equal
    } else if x.Ne().is_some() {
        meta::Function::NotEqual
    } else {
        meta::Function::Unresolved
    }
}

/// Analyzes an optional child `expr`, mapping a missing child to the unresolved
/// value (any diagnostics have already been emitted while parsing).
fn child_value(
    e: Option<Rc<ExprContextAll<'_>>>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> meta::pattern::Value {
    match e {
        Some(e) => analyze_expr(&e, y, z),
        None => meta::pattern::Value::Unresolved,
    }
}

/// Analyzes a single numeric parameter (the `<...>` arguments of a parameterized
/// type) into a pattern value.
fn numeric_parameter(
    x: &NumericParameterContextAll<'_>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> meta::pattern::Value {
    match x {
        NumericParameterContextAll::NumericLiteralContext(c) => c
            .Number()
            .and_then(|n| analyze_number(n.symbol.text.as_ref(), y))
            .map(|v| meta::pattern::Value::Integer(v, v))
            .unwrap_or_default(),
        NumericParameterContextAll::NumericParameterNameContext(c) => match c.Identifier() {
            Some(id) => identifier_to_value(id.symbol.text.as_ref(), None, y, z),
            None => meta::pattern::Value::Unresolved,
        },
        NumericParameterContextAll::NumericExpressionContext(c) => child_value(c.expr(), y, z),
        NumericParameterContextAll::Error(_) => meta::pattern::Value::Unresolved,
    }
}

/// Wraps a sequence of numeric parameters as unnamed pattern parameters.
fn numeric_parameters(
    xs: Vec<Rc<NumericParameterContextAll<'_>>>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Vec<meta::pattern::Parameter> {
    xs.iter()
        .map(|p| meta::pattern::Parameter {
            name: None,
            value: Some(numeric_parameter(p, y, z)),
        })
        .collect()
}

/// Wraps a sequence of `expr` parameters (used by `struct`, `list`, `map`, ...)
/// as unnamed pattern parameters.
fn expr_parameters(
    xs: Vec<Rc<ExprContextAll<'_>>>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Vec<meta::pattern::Parameter> {
    xs.iter()
        .map(|e| meta::pattern::Parameter {
            name: None,
            value: Some(analyze_expr(e, y, z)),
        })
        .collect()
}

/// Maps a scalar type alternative to its data type class.
fn analyze_scalar_type(x: &ScalarTypeContextAll<'_>) -> data::Class {
    use data::class::Simple;
    let simple = match x {
        ScalarTypeContextAll::BooleanContext(_) => Simple::Boolean,
        ScalarTypeContextAll::I8Context(_) => Simple::I8,
        ScalarTypeContextAll::I16Context(_) => Simple::I16,
        ScalarTypeContextAll::I32Context(_) => Simple::I32,
        ScalarTypeContextAll::I64Context(_) => Simple::I64,
        ScalarTypeContextAll::Fp32Context(_) => Simple::Fp32,
        ScalarTypeContextAll::Fp64Context(_) => Simple::Fp64,
        ScalarTypeContextAll::StringContext(_) => Simple::String,
        ScalarTypeContextAll::BinaryContext(_) => Simple::Binary,
        ScalarTypeContextAll::TimestampContext(_) => Simple::Timestamp,
        ScalarTypeContextAll::TimestampTzContext(_) => Simple::TimestampTz,
        ScalarTypeContextAll::DateContext(_) => Simple::Date,
        ScalarTypeContextAll::TimeContext(_) => Simple::Time,
        ScalarTypeContextAll::IntervalYearContext(_) => Simple::IntervalYear,
        ScalarTypeContextAll::UuidContext(_) => Simple::Uuid,
        ScalarTypeContextAll::Error(_) => return data::Class::Unresolved,
    };
    data::Class::Simple(simple)
}

/// Analyzes a parameterized type alternative into a data type pattern.
///
/// NOTE: the grammar (and the spec) include several classes the validator's
/// type system does not yet model (`func`, the `precision_*` temporal types and
/// `interval_compound`). These are mapped to an unresolved class with a
/// diagnostic for now; extending [`data::Class`] to cover them is tracked as a
/// follow-up to the substrait-antlr migration.
fn analyze_parameterized_type(
    x: &ParameterizedTypeContextAll<'_>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> meta::pattern::Value {
    use data::class::Compound;

    let unsupported = |y: &mut context::Context, what: &str| {
        diagnostic!(
            y,
            Error,
            TypeDerivationNotSupported,
            "the {what} type class is not yet supported by the validator"
        );
    };

    let (class, nullable, parameters) = match x {
        ParameterizedTypeContextAll::DecimalContext(c) => (
            data::Class::Compound(Compound::Decimal),
            c.QMark().is_some(),
            Some(numeric_parameters(c.numericParameter_all(), y, z)),
        ),
        ParameterizedTypeContextAll::VarCharContext(c) => (
            data::Class::Compound(Compound::VarChar),
            c.QMark().is_some(),
            Some(numeric_parameters(c.numericParameter().into_iter().collect(), y, z)),
        ),
        ParameterizedTypeContextAll::FixedCharContext(c) => (
            data::Class::Compound(Compound::FixedChar),
            c.QMark().is_some(),
            Some(numeric_parameters(c.numericParameter().into_iter().collect(), y, z)),
        ),
        ParameterizedTypeContextAll::FixedBinaryContext(c) => (
            data::Class::Compound(Compound::FixedBinary),
            c.QMark().is_some(),
            Some(numeric_parameters(c.numericParameter().into_iter().collect(), y, z)),
        ),
        ParameterizedTypeContextAll::StructContext(c) => (
            data::Class::Compound(Compound::Struct),
            c.QMark().is_some(),
            Some(expr_parameters(c.expr_all(), y, z)),
        ),
        ParameterizedTypeContextAll::NStructContext(c) => {
            // The i-th field name (`Identifier(i)`, filtered by token type)
            // pairs with the i-th field type expression.
            let values = c.expr_all();
            let parameters = values
                .iter()
                .enumerate()
                .map(|(i, value)| meta::pattern::Parameter {
                    name: c.Identifier(i).map(|name| name.symbol.text.to_string()),
                    value: Some(analyze_expr(value, y, z)),
                })
                .collect();
            (
                data::Class::Compound(Compound::NamedStruct),
                c.QMark().is_some(),
                Some(parameters),
            )
        }
        ParameterizedTypeContextAll::ListContext(c) => (
            data::Class::Compound(Compound::List),
            c.QMark().is_some(),
            Some(vec![meta::pattern::Parameter {
                name: None,
                value: Some(child_value(c.expr(), y, z)),
            }]),
        ),
        ParameterizedTypeContextAll::MapContext(c) => (
            data::Class::Compound(Compound::Map),
            c.QMark().is_some(),
            Some(expr_parameters(c.expr_all(), y, z)),
        ),
        ParameterizedTypeContextAll::UserDefinedContext(c) => {
            let nullable = c.QMark().is_some();
            let parameters = expr_parameters(c.expr_all(), y, z);
            let class = match c.Identifier() {
                Some(id) => {
                    let name = id.symbol.text.to_string();
                    match z.resolve_pattern(std::iter::once(name.as_str()), y) {
                        PatternObject::TypeClass(class) => class,
                        _ => data::Class::Unresolved,
                    }
                }
                None => data::Class::Unresolved,
            };
            (
                class,
                nullable,
                if parameters.is_empty() {
                    None
                } else {
                    Some(parameters)
                },
            )
        }
        ParameterizedTypeContextAll::FuncContext(c) => {
            unsupported(y, "func");
            (data::Class::Unresolved, c.QMark().is_some(), None)
        }
        ParameterizedTypeContextAll::PrecisionTimeContext(c) => {
            unsupported(y, "precision_time");
            (data::Class::Unresolved, c.QMark().is_some(), None)
        }
        ParameterizedTypeContextAll::PrecisionTimestampContext(c) => {
            unsupported(y, "precision_timestamp");
            (data::Class::Unresolved, c.QMark().is_some(), None)
        }
        ParameterizedTypeContextAll::PrecisionTimestampTZContext(c) => {
            unsupported(y, "precision_timestamp_tz");
            (data::Class::Unresolved, c.QMark().is_some(), None)
        }
        ParameterizedTypeContextAll::PrecisionIntervalDayContext(c) => {
            unsupported(y, "interval_day");
            (data::Class::Unresolved, c.QMark().is_some(), None)
        }
        ParameterizedTypeContextAll::PrecisionIntervalCompoundContext(c) => {
            unsupported(y, "interval_compound");
            (data::Class::Unresolved, c.QMark().is_some(), None)
        }
        ParameterizedTypeContextAll::Error(_) => (data::Class::Unresolved, false, None),
    };

    data_type_pattern(Some(class), nullable, parameters)
}

/// Analyzes an `anyType` (`any`/`anyN`). These behave like (wildcard) bindings:
/// they match any value and reproduce it on evaluation, which mirrors the
/// historical treatment of `anyN` identifiers as named bindings.
fn analyze_any_type(
    x: &AnyTypeContextAll<'_>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> meta::pattern::Value {
    let name = x
        .Any()
        .or_else(|| x.AnyVar())
        .map(|n| n.symbol.text.to_string())
        .unwrap_or_default();
    let nullable = x.QMark().is_some().then_some(true);
    identifier_to_value(&name, nullable, y, z)
}

/// Analyzes a `typeDef` (a scalar, parameterized, or `any` type) into a pattern.
fn analyze_type_def(
    x: &TypeDefContextAll<'_>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> meta::pattern::Value {
    if let Some(scalar) = x.scalarType() {
        data_type_pattern(Some(analyze_scalar_type(&scalar)), x.QMark().is_some(), None)
    } else if let Some(parameterized) = x.parameterizedType() {
        analyze_parameterized_type(&parameterized, y, z)
    } else if let Some(any) = x.anyType() {
        analyze_any_type(&any, y, z)
    } else {
        meta::pattern::Value::Unresolved
    }
}

/// Analyzes a `FunctionCall` (`identifier(args...)`) into a function pattern.
fn analyze_function_call(
    x: &FunctionCallContext<'_>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> meta::pattern::Value {
    let function = x
        .Identifier()
        .and_then(|id| meta::Function::try_from(id.symbol.text.as_ref()).ok());
    if function.is_none() {
        diagnostic!(y, Error, TypeParseError, "unknown function");
    }
    let function = function.unwrap_or_default();
    let arguments: Vec<_> = x
        .expr_all()
        .iter()
        .map(|e| analyze_expr(e, y, z))
        .collect();
    check_function(y, z, &function, &arguments);
    meta::pattern::Value::Function(function, arguments)
}

/// Analyzes a parsed `expr` node into a meta-pattern value.
fn analyze_expr(
    x: &ExprContextAll<'_>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> meta::pattern::Value {
    match x {
        ExprContextAll::ParenExpressionContext(x) => child_value(x.expr(), y, z),
        ExprContextAll::TypeLiteralContext(x) => match x.typeDef() {
            Some(td) => analyze_type_def(&td, y, z),
            None => meta::pattern::Value::Unresolved,
        },
        ExprContextAll::LiteralNumberContext(x) => x
            .Number()
            .and_then(|n| analyze_number(n.symbol.text.as_ref(), y))
            .map(|v| meta::pattern::Value::Integer(v, v))
            .unwrap_or_default(),
        ExprContextAll::ParameterNameContext(x) => {
            let nullable = x.QMark().is_some().then_some(true);
            match x.Identifier() {
                Some(id) => identifier_to_value(id.symbol.text.as_ref(), nullable, y, z),
                None => meta::pattern::Value::Unresolved,
            }
        }
        ExprContextAll::FunctionCallContext(x) => analyze_function_call(x, y, z),
        ExprContextAll::BinaryExprContext(x) => {
            let arguments = vec![child_value(x.expr(0), y, z), child_value(x.expr(1), y, z)];
            let function = binary_operator(x);
            check_function(y, z, &function, &arguments);
            meta::pattern::Value::Function(function, arguments)
        }
        ExprContextAll::IfExprContext(x) => {
            let arguments = vec![
                child_value(x.expr(0), y, z),
                child_value(x.expr(1), y, z),
                child_value(x.expr(2), y, z),
            ];
            let function = meta::Function::IfThenElse;
            check_function(y, z, &function, &arguments);
            meta::pattern::Value::Function(function, arguments)
        }
        ExprContextAll::TernaryContext(x) => {
            let arguments = vec![
                child_value(x.expr(0), y, z),
                child_value(x.expr(1), y, z),
                child_value(x.expr(2), y, z),
            ];
            let function = meta::Function::IfThenElse;
            check_function(y, z, &function, &arguments);
            meta::pattern::Value::Function(function, arguments)
        }
        ExprContextAll::NotExprContext(x) => {
            let arguments = vec![child_value(x.expr(), y, z)];
            let function = meta::Function::Not;
            check_function(y, z, &function, &arguments);
            meta::pattern::Value::Function(function, arguments)
        }
        ExprContextAll::MultilineDefinitionContext(x) => {
            // A derivation program used in expression position; the statements
            // are handled by parse_program, so here we only yield its result.
            analyze_multiline(x, y, z).expression
        }
        ExprContextAll::Error(_) => meta::pattern::Value::Unresolved,
    }
}

/// Analyzes a multi-line definition (`name = expr` assignments followed by a
/// final type) into a derivation program.
fn analyze_multiline(
    x: &MultilineDefinitionContext<'_>,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> meta::Program {
    // Note: the i-th `Eq`-assigned identifier pairs with the i-th right-hand
    // expression. We index identifiers via `Identifier(i)` (which filters by
    // token type) rather than `Identifier_all()`, which returns every terminal
    // child regardless of token type in this antlr4rust runtime.
    let expressions = x.expr_all();
    let mut statements = Vec::with_capacity(expressions.len());
    for (i, expression) in expressions.iter().enumerate() {
        // The right-hand side is analyzed first so its bindings are captured
        // before the left-hand binding name is introduced.
        let rhs_expression = analyze_expr(expression, y, z);
        let lhs_pattern = match x.Identifier(i) {
            Some(name) => identifier_to_value(name.symbol.text.as_ref(), None, y, z),
            None => meta::pattern::Value::Unresolved,
        };
        statements.push(meta::program::Statement {
            lhs_pattern,
            rhs_expression,
        });
    }
    let expression = match x.typeDef() {
        Some(td) => analyze_type_def(&td, y, z),
        None => meta::pattern::Value::Unresolved,
    };
    meta::Program {
        statements,
        expression,
    }
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

/// Parse a string as a meta-pattern. The grammar's `startRule` accepts a single
/// expression, which covers both plain type patterns (e.g. `decimal<P, S>`) and
/// derivation expressions.
pub fn parse_pattern(
    x: &str,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::pattern::Value> {
    let root = antlr_parse!(x, y, startRule)?;
    Ok(match root.expr() {
        Some(expr) => analyze_expr(&expr, y, z),
        None => meta::pattern::Value::Unresolved,
    })
}

/// Parse a string as a meta-program. A multi-line definition becomes a program
/// with assignment statements followed by the final type; any other expression
/// becomes a program with no statements and that expression as its result.
pub fn parse_program(
    x: &str,
    y: &mut context::Context,
    z: &mut AnalysisContext,
) -> Result<meta::Program> {
    let root = antlr_parse!(x, y, startRule)?;
    let program = match root.expr() {
        Some(expr) => match expr.as_ref() {
            ExprContextAll::MultilineDefinitionContext(multiline) => {
                analyze_multiline(multiline, y, z)
            }
            _ => meta::Program {
                statements: Vec::new(),
                expression: analyze_expr(&expr, y, z),
            },
        },
        None => meta::Program::default(),
    };
    if !program.expression.can_evaluate() {
        diagnostic!(
            y,
            Error,
            TypeDerivationInvalid,
            "return pattern of program can never be evaluated"
        );
    }
    Ok(program)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::output::tree;

    /// Declares the boilerplate locals (`$node`, `$state`, `$config`) and a
    /// parsing context bound to `$ctx`, used as a diagnostic sink by the
    /// derivation parser. Mirrors the setup inlined in [`test1`]/[`test2`].
    macro_rules! test_context {
        ($node:ident, $state:ident, $config:ident, $ctx:ident) => {
            let mut $node = tree::Node::from(tree::NodeType::ProtoMessage("test"));
            let mut $state = Default::default();
            let $config = crate::Config::new();
            let mut $ctx = context::Context::new("test", &mut $node, &mut $state, &$config);
        };
    }

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

        // The spec grammar (substrait-antlr) defines all binary operators in a
        // single left-recursive alternative, so they share one precedence level
        // and associate left-to-right -- there is no `*`-over-`+` precedence.
        // `1 + 2 * 3 - 4 / 5` therefore evaluates as `((((1+2)*3)-4)/5) == 1`,
        // not `7`. This differs from the validator's former bespoke grammar,
        // which encoded conventional arithmetic precedence; see
        // https://github.com/substrait-io/substrait-validator/issues/526.
        let program = parse_program(r#"1 + 2 * 3 - 4 / 5"#, &mut context, &mut analysis_context)
            .unwrap_or_default();

        let mut eval_context = meta::Context::default();
        assert_eq!(
            program.evaluate(&mut eval_context),
            Ok(meta::Value::Integer(1))
        );
    }

    // ----------------------------------------------------------------------
    // Characterization tests.
    //
    // These pin the parse/evaluate behavior of the derivation vocabulary that
    // actually appears in the bundled standard extensions
    // (rs/src/resources/extensions/*.yaml), so the grammar can be swapped to
    // the substrait-antlr crate without behavioral drift. They cover simple,
    // nullable, and parameterized types, the `anyN` binding passthrough used by
    // comparison functions, and the decimal arithmetic derivation programs.
    // ----------------------------------------------------------------------

    /// Simple and nullable scalar types round-trip through parse + evaluate.
    #[test]
    fn characterize_simple_and_nullable_types() {
        test_context!(node, state, config, y);
        let mut z = AnalysisContext::new(None);
        for (input, expected) in [
            ("i8", "i8"),
            ("i16", "i16"),
            ("i32", "i32"),
            ("i64", "i64"),
            ("fp32", "fp32"),
            ("fp64", "fp64"),
            ("boolean", "boolean"),
            ("string", "string"),
            ("date", "date"),
            ("time", "time"),
            ("timestamp", "timestamp"),
            ("timestamp_tz", "timestamp_tz"),
            ("interval_year", "interval_year"),
            ("fp32?", "fp32?"),
            ("i32?", "i32?"),
            ("boolean?", "boolean?"),
        ] {
            let ty = parse_type(input, &mut y, &mut z)
                .unwrap_or_else(|e| panic!("failed to parse {input:?}: {e}"));
            assert_eq!(ty.to_string(), expected, "for input {input:?}");
        }
    }

    /// Parameterized types with literal parameters round-trip.
    #[test]
    fn characterize_parameterized_literals() {
        test_context!(node, state, config, y);
        let mut z = AnalysisContext::new(None);
        for (input, expected) in [
            ("decimal<38,0>", "DECIMAL<38, 0>"),
            ("decimal<10,2>", "DECIMAL<10, 2>"),
            ("decimal?<10,2>", "DECIMAL?<10, 2>"),
            ("varchar<5>", "VARCHAR<5>"),
            ("fixedchar<3>", "FIXEDCHAR<3>"),
        ] {
            let ty = parse_type(input, &mut y, &mut z)
                .unwrap_or_else(|e| panic!("failed to parse {input:?}: {e}"));
            assert_eq!(ty.to_string(), expected, "for input {input:?}");
        }
    }

    /// The `anyN` binding used by comparison/identity functions captures an
    /// argument type during matching and is reproduced by the return program.
    #[test]
    fn characterize_any_binding_passthrough() {
        test_context!(node, state, config, y);
        let mut z = AnalysisContext::new(None);

        // Models e.g. `min(any1) -> any1`: bind any1 to the argument, return it.
        let arg = parse_pattern("any1", &mut y, &mut z).unwrap();
        let ret = parse_program("any1", &mut y, &mut z).unwrap();
        let bound = parse_type("i32", &mut y, &mut z).unwrap();

        let mut c = meta::Context::default();
        assert!(arg
            .match_pattern_with_context(&mut c, &bound.into())
            .unwrap());
        assert_eq!(ret.evaluate(&mut c).unwrap().to_string(), "i32");
    }

    /// The decimal `multiply` return derivation, evaluated against concrete
    /// argument types (companion to the `add` derivation in [`test1`]).
    #[test]
    fn characterize_decimal_multiply() {
        test_context!(node, state, config, y);
        let mut z = AnalysisContext::new(None);

        let x_slot = parse_pattern("decimal<P1,S1>", &mut y, &mut z).unwrap();
        let y_slot = parse_pattern("decimal<P2,S2>", &mut y, &mut z).unwrap();
        let return_program = parse_program(
            r#"init_scale = S1 + S2
            init_prec = P1 + P2 + 1
            min_scale = min(init_scale, 6)
            delta = init_prec - 38
            prec = min(init_prec, 38)
            scale_after_borrow = max(init_scale - delta, min_scale)
            scale = init_prec > 38 ? scale_after_borrow : init_scale
            DECIMAL<prec, scale>"#,
            &mut y,
            &mut z,
        )
        .unwrap();

        let x_binding = parse_type("decimal<20,10>", &mut y, &mut z).unwrap();
        let y_binding = parse_type("decimal<10,5>", &mut y, &mut z).unwrap();

        let mut c = meta::Context::default();
        assert!(x_slot
            .match_pattern_with_context(&mut c, &x_binding.into())
            .unwrap());
        assert!(y_slot
            .match_pattern_with_context(&mut c, &y_binding.into())
            .unwrap());
        let result = return_program.evaluate(&mut c).unwrap();
        assert_eq!(&result.to_string(), "DECIMAL<31, 15>");
    }

    /// A single-expression return program (the common case, e.g. `return: i64`)
    /// evaluates to that concrete type with no statements.
    #[test]
    fn characterize_single_expression_return() {
        test_context!(node, state, config, y);
        let mut z = AnalysisContext::new(None);
        let program = parse_program("i64", &mut y, &mut z).unwrap();
        assert!(program.statements.is_empty());
        let mut c = meta::Context::default();
        assert_eq!(program.evaluate(&mut c).unwrap().to_string(), "i64");
    }

    /// Syntax that the validator's former (stale) grammar could not parse but
    /// the spec grammar shipped in substrait-antlr handles: the user-defined
    /// type sigil `u!` and function types `func<... -> ...>`. We only assert
    /// that parsing succeeds here; full type-system support for `func` is a
    /// tracked follow-up (the class currently resolves as unsupported).
    #[test]
    fn parses_user_defined_and_func_types() {
        test_context!(node, state, config, y);
        let mut z = AnalysisContext::new(None);
        assert!(parse_pattern("u!geometry", &mut y, &mut z).is_ok());
        assert!(parse_pattern("func<any1 -> any2>", &mut y, &mut z).is_ok());
    }
}
