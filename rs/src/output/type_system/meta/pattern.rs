// SPDX-License-Identifier: Apache-2.0

//! Module for [`Pattern`]s that can be used to match one or more meta values
//! for type and bounds checking, or can be evaluated (reverse direction) to
//! obtain procedurally-generated value.

use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::util;
use crate::util::string::Describe;
use std::fmt::Write;
use std::sync::Arc;

// TODO: s/Pattern/Value/g, then make Pattern trait for exactly(),
// match_pattern*(), evaluate*(), and some derived stuff

// TODO: implement meta::Function

/// Patterns are used wherever a meta::Value is expected, such as for type
/// parameter slots. When they appear as an input (argument type, LHS of
/// assignment, etc), the pattern is *matched*; see [Pattern::match_pattern()]
/// and [Pattern::match_pattern_with_context()]. This can obviously succeed or
/// fail, but when it succeeds, it notably can also bind values, akin to
/// assigning variables. When a pattern appears on the RHS of an assignment,
/// it acts like an expression and is evaluated; see [Pattern::evaluate()]
/// and [Pattern::evaluate_with_context()]. This either fails or yields a
/// [meta::Value].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pattern {
    /// Accepts any meta::Value. Syntax: `?`. Also used when a pattern is unknown
    /// due to validator-specific error recovery. Cannot be evaluated.
    Any,

    /// A binding. These act sort of like variables in a given context. When
    /// the binding is first matched against a value, it acts like Any and
    /// assumes that value. When it is matched again in the same context later,
    /// it only matches meta::Values that are exactly equal to the previous
    /// match. When evaluated, it evaluates to the value that the binding was
    /// bound to, or fails if it was not bound. Syntax: any identifier that
    /// isn't recognized as anything else.
    Binding(String),

    /// A special binding that accepts any boolean. When the binding is first
    /// matched against a value, the binding assumes the value. When it is
    /// matched again in the same context later, the binding assumes the
    /// boolean OR of the previous value of the binding and the matched value.
    /// This is used to handle MIRROR nullability behavior. When evaluated, it
    /// evaluates to the value that the binding was bound to, or to false if
    /// it was not found. Syntax: any identifier that isn't recognized as
    /// anything else, followed by a `?`.
    ImplicitOrBinding(String),

    /// Pattern for booleans.
    ///  - None: both true and false match the pattern. Cannot be evaluated.
    ///    Syntax: `metabool`
    ///  - Some(true): only true matches the pattern, and evaluates to true.
    ///    Syntax: `true`
    ///  - Some(false): only false matches the pattern, and evaluates to false.
    ///    Syntax: `false`
    Boolean(Option<bool>),

    /// Pattern for integers. Only integers within the given inclusive
    /// range match the pattern. Can only be evaluated if the lower and upper
    /// bound are equal. Syntax: `metaint`, `<int>..`, `..<int>`, or
    /// `<int>..<int>`.
    Integer(i64, i64),

    /// Pattern for enumerations.
    ///  - None: any enum value is accepted. Cannot be evaluated. Syntax:
    ///    `metaenum`.
    ///  - Some(options): only enum values that case-insensitively
    ///    match one of the given options are matched. Can only be evaluated
    ///    if there is only one option, in which case it evaluates to that
    ///    option. Syntax: `{<identifier,+>}`.
    Enum(Option<Vec<String>>),

    /// Pattern for strings.
    ///  - None: matches any string. Cannot be evaluated. Syntax: `metastr`.
    ///  - Some(_): matches the given string exactly. Evaluates to the given
    ///    string. Syntax: quoted string.
    String(Option<String>),

    /// Pattern for data types.
    ///  - None: matches any data type. Cannot be evaluated. Syntax: `typename`.
    ///  - Some(_): matches what the contained pattern matches, and evaluates to
    ///    what it evaluates to. Syntax:
    ///    `<class><null-pattern><variation-pattern><parameter-pattern>`.
    DataType(Option<DataType>),

    /// A function applied to a number of patterns. Functions cannot be
    /// matched; they can only be evaluated. Syntax:
    /// `<function-name>(<args>, ...)`, as well as expression-style syntactic
    /// sugar for some of the functions:
    ///  - Boolean arithmetic: `a && b`, `a || b`, `!a`;
    ///  - Integer arithmetic: `a + b`, `a - b`, `a * b`, `a / b`, `-a`, `-b`;
    ///  - Comparisons: `a == b`, `a != b`, `a < b`, `a <= b`, `a > b`,
    ///    `a >= b`;
    ///  - Ternary: `if a then b else c`.
    Function(meta::Function, Vec<Pattern>),
}

impl Describe for Pattern {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            Pattern::Any => write!(f, "?"),
            Pattern::Binding(name) => util::string::describe_identifier(f, name, limit),
            Pattern::ImplicitOrBinding(name) => {
                util::string::describe_identifier(f, name, limit)?;
                write!(f, "?")
            }
            Pattern::Boolean(None) => write!(f, "metabool"),
            Pattern::Boolean(Some(true)) => write!(f, "true"),
            Pattern::Boolean(Some(false)) => write!(f, "false"),
            Pattern::Integer(i64::MIN, i64::MAX) => write!(f, "metaint"),
            Pattern::Integer(i64::MIN, max) => write!(f, "..{max}"),
            Pattern::Integer(min, i64::MAX) => write!(f, "{min}.."),
            Pattern::Integer(min, max) => write!(f, "{min}..{max}"),
            Pattern::Enum(None) => write!(f, "metaenum"),
            Pattern::Enum(Some(variants)) => {
                util::string::describe_sequence(f, variants, limit, 10, |f, variant, _, limit| {
                    util::string::describe_identifier(f, variant, limit)
                })
            }
            Pattern::String(None) => write!(f, "metastr"),
            Pattern::String(Some(text)) => util::string::describe_string(f, text, limit),
            Pattern::DataType(None) => write!(f, "typename"),
            Pattern::DataType(Some(pattern)) => pattern.describe(f, limit),
            Pattern::Function(func, args) => {
                let (a, b) = limit.split(20);
                func.describe(f, a)?;
                write!(f, "(")?;
                util::string::describe_sequence(f, args, b, 10, |f, arg, _, limit| {
                    arg.describe(f, limit)
                })
            }
        }
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Pattern {
    /// Returns a pattern that matches the given value exactly.
    pub fn exactly(value: meta::Value) -> Self {
        match value {
            meta::Value::Unresolved => Pattern::Any,
            meta::Value::Boolean(x) => Pattern::Boolean(Some(x)),
            meta::Value::Integer(x) => Pattern::Integer(x, x),
            meta::Value::Enum(x) => Pattern::Enum(Some(vec![x])),
            meta::Value::String(x) => Pattern::String(Some(x)),
            meta::Value::DataType(x) => Pattern::DataType(Some(DataType::exactly(x))),
        }
    }

    /// Returns a pattern that matches the given type exactly.
    pub fn exactly_type(meta_type: meta::Type) -> Self {
        match meta_type {
            meta::Type::Unresolved => Pattern::Any,
            meta::Type::Boolean => Pattern::Boolean(None),
            meta::Type::Integer => Pattern::Integer(i64::MIN, i64::MAX),
            meta::Type::Enum => Pattern::Enum(None),
            meta::Type::String => Pattern::String(None),
            meta::Type::DataType => Pattern::DataType(None),
        }
    }

    /// Matches this pattern without any provided context.
    pub fn match_pattern(&self, value: &meta::Value) -> bool {
        let mut context = meta::Context::default();
        self.match_pattern_with_context(&mut context, value)
    }

    /// Matches this pattern with a provided context.
    pub fn match_pattern_with_context(
        &self,
        context: &mut meta::Context,
        value: &meta::Value,
    ) -> bool {
        match self {
            Pattern::Any => true,
            Pattern::Binding(name) => {
                if let Some(expected) = context.bindings.get(name) {
                    if value != expected {
                        return false;
                    }
                } else {
                    context
                        .bindings
                        .insert(name.clone(), value.clone())
                        .unwrap();
                }
                true
            }
            Pattern::ImplicitOrBinding(name) => {
                if let Some(mut value) = value.get_boolean() {
                    if let Some(expected) = context.bindings.get(name) {
                        if let Some(current) = expected.get_boolean() {
                            value |= current;
                        } else {
                            return false;
                        }
                    }
                    context.bindings.insert(name.clone(), value.into()).unwrap();
                    true
                } else {
                    false
                }
            }
            Pattern::Boolean(expected) => {
                if let Some(value) = value.get_boolean() {
                    if let Some(expected) = expected {
                        value == *expected
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            Pattern::Integer(low, high) => {
                if let Some(value) = value.get_integer() {
                    value >= *low && value <= *high
                } else {
                    false
                }
            }
            Pattern::Enum(expected) => {
                if let Some(value) = value.get_enum() {
                    if let Some(variants) = expected {
                        variants
                            .iter()
                            .any(|variant| variant.eq_ignore_ascii_case(value))
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            Pattern::String(expected) => {
                if let Some(value) = value.get_string() {
                    if let Some(expected) = expected {
                        value == expected
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            Pattern::DataType(expected) => {
                if let Some(value) = value.get_data_type() {
                    if let Some(expected) = expected {
                        expected.match_pattern_with_context(context, &value)
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            Pattern::Function(_, _) => false,
        }
    }

    /// Evaluates this pattern without any provided context.
    pub fn evaluate(&self) -> diagnostic::Result<meta::Value> {
        let mut context = meta::Context::default();
        self.evaluate_with_context(&mut context)
    }

    /// Evaluates this pattern with a provided context.
    pub fn evaluate_with_context(
        &self,
        context: &mut meta::Context,
    ) -> diagnostic::Result<meta::Value> {
        match self {
            Pattern::Any => Err(cause!(DerivationInvalid, "? patterns cannot be evaluated")),
            Pattern::Binding(name) => {
                if let Some(value) = context.bindings.get(name) {
                    Ok(value.clone())
                } else {
                    Err(cause!(DerivationInvalid, "{name} was never bound"))
                }
            }
            Pattern::ImplicitOrBinding(name) => {
                if let Some(value) = context.bindings.get(name) {
                    if value.get_boolean().is_none() {
                        Err(cause!(
                            DerivationInvalid,
                            "cannot evaluate {name}? because {name} was not bound to a boolean"
                        ))
                    } else {
                        Ok(value.clone())
                    }
                } else {
                    Ok(false.into())
                }
            }
            Pattern::Boolean(value) => {
                if let Some(value) = value {
                    Ok((*value).into())
                } else {
                    Err(cause!(
                        DerivationInvalid,
                        "cannot evaluate boolean with unknown value"
                    ))
                }
            }
            Pattern::Integer(low, high) => {
                if low == high {
                    Ok((*low).into())
                } else {
                    Err(cause!(
                        DerivationInvalid,
                        "cannot evaluate integer with unknown value"
                    ))
                }
            }
            Pattern::Enum(values) => {
                if let Some(values) = values {
                    if values.len() == 1 {
                        Ok(meta::Value::Enum(values[0].clone()))
                    } else {
                        Err(cause!(
                            DerivationInvalid,
                            "cannot evaluate enum with unknown value"
                        ))
                    }
                } else {
                    Err(cause!(DerivationInvalid, "cannot evaluate undefined enum"))
                }
            }
            Pattern::String(value) => {
                if let Some(value) = value {
                    Ok(value.clone().into())
                } else {
                    Err(cause!(
                        DerivationInvalid,
                        "cannot evaluate string with unknown value"
                    ))
                }
            }
            Pattern::DataType(value) => {
                if let Some(value) = value {
                    value.evaluate_with_context(context).map(meta::Value::from)
                } else {
                    Err(cause!(
                        DerivationInvalid,
                        "cannot evaluate undefined data type"
                    ))
                }
            }
            Pattern::Function(func, args) => func.evaluate_with_context(context, &args[..]),
        }
    }
}

/// Data type matching structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataType {
    /// Type class (simple, compound, or user-defined). This must always match
    /// exactly. [DataTypePattern]s are wrapped in an Option if the class can
    /// also be omitted.
    pub class: data::Class,

    /// Nullability, defined using a (boolean) Pattern. Syntax:
    ///  - no suffix: Boolean(Some(false))
    ///  - `?` suffix: Boolean(Some(true))
    ///  - `??` suffix: Boolean(None)
    ///  - `?<identifier>` suffix: Binding(_)
    ///  - `?<identifier>?` suffix: ImplicitOrBinding(_)
    pub nullable: Arc<Pattern>,

    /// Type variation pattern.
    pub variation: Variation,

    /// Type parameters for non-simple types.
    ///  - None: any number of parameters is accepted (type class willing).
    ///    Syntax: no suffix.
    ///  - Some: the number of parameters must match the length of the
    ///    contained vector, and the parameters themselves must match the given
    ///    patterns. Syntax: `<>`-enclosed list of patterns. None can be used
    ///    to only match an explicitly undefined optional parameter.
    pub parameters: Option<Vec<Parameter>>,
}

impl Describe for DataType {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        let mut non_recursive = String::new();
        write!(
            &mut non_recursive,
            "{}{}{}",
            self.class, self.nullable, self.variation
        )?;
        write!(f, "{}", non_recursive)?;
        if let Some(parameters) = &self.parameters {
            write!(f, "<")?;
            util::string::describe_sequence(f, parameters, limit, 20, |f, param, _, limit| {
                param.describe(f, limit)
            })?;
            write!(f, ">")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl DataType {
    /// Returns a pattern that matches the given type exactly.
    pub fn exactly(value: data::Type) -> Self {
        DataType {
            class: value.class().clone(),
            nullable: Arc::new(Pattern::exactly(meta::Value::from(value.nullable()))),
            variation: Variation::Exactly(value.variation().clone()),
            parameters: Some(
                value
                    .parameters()
                    .iter()
                    .cloned()
                    .map(Parameter::exactly)
                    .collect(),
            ),
        }
    }

    /// Matches this pattern with a provided context.
    pub fn match_pattern_with_context(
        &self,
        context: &mut meta::Context,
        value: &data::Type,
    ) -> bool {
        // As a rule, unresolved things match everything, in order to not emit
        // excessive amounts of diagnostics due to a single error.
        if value.is_unresolved() {
            return true;
        }
        if !value.class().weak_equals(&self.class) {
            return false;
        }
        if self
            .nullable
            .match_pattern_with_context(context, &value.nullable().into())
        {
            return false;
        }
        if self.variation.match_pattern(value.variation()) {
            return false;
        }
        if let Some(expected) = &self.parameters {
            let parameters = value.parameters();
            if parameters.len() != expected.len() {
                return false;
            }
            for (parameter, expected) in parameters.iter().zip(expected.iter()) {
                if !expected.match_pattern_with_context(context, parameter) {
                    return false;
                }
            }
        }
        true
    }

    /// Evaluates this data type pattern with a provided context.
    pub fn evaluate_with_context(
        &self,
        context: &mut meta::Context,
    ) -> diagnostic::Result<data::Type> {
        let class = self.class.clone();
        let nullable = self
            .nullable
            .evaluate_with_context(context)?
            .get_boolean()
            .ok_or_else(|| {
                cause!(
                    DerivationInvalid,
                    "nullability pattern evaluated to non-boolean"
                )
            })?;
        let variation = self.variation.evaluate()?;
        let parameters = if let Some(parameters) = &self.parameters {
            parameters
                .iter()
                .map(|parameter| parameter.evaluate_with_context(context))
                .collect::<diagnostic::Result<Vec<data::Parameter>>>()?
        } else {
            vec![]
        };
        data::new_type(class, nullable, variation, parameters)
    }
}

/// Type variation matching structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Variation {
    /// Matches any variation. Syntax: `[?]` suffix.
    Any,

    /// Matches any variation that is compatible with the system-preferred
    /// variation; that is, matches the system-preferred variation and any
    /// variation declared with INHERITS function behavior.
    Compatible,

    /// Matches exactly the given variation, regardless of INHERITS function
    /// behavior. Syntax:
    ///  - exactly the system-preferred variation: `[0]` suffix.
    ///  - exactly the given variation: `[<identifier>]` suffix.
    Exactly(data::Variation),
}

impl std::fmt::Display for Variation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variation::Any => write!(f, "[?]"),
            Variation::Compatible => Ok(()),
            Variation::Exactly(variation) => write!(f, "[{variation}]"),
        }
    }
}

impl Variation {
    /// Returns a pattern that matches the given variation.
    pub fn exactly(value: data::Variation) -> Self {
        Variation::Exactly(value)
    }

    /// Matches this pattern.
    pub fn match_pattern(&self, value: &data::Variation) -> bool {
        match self {
            Variation::Any => true,
            Variation::Compatible => value.is_compatible_with_system_preferred(),
            Variation::Exactly(expected) => value == expected,
        }
    }

    /// Evaluates this pattern.
    pub fn evaluate(&self) -> diagnostic::Result<data::Variation> {
        match self {
            Variation::Any => Err(cause!(
                DerivationInvalid,
                "cannot evaluate undefined variation"
            )),
            Variation::Compatible => Ok(data::Variation::SystemPreferred),
            Variation::Exactly(expected) => Ok(expected.clone()),
        }
    }
}

/// Pattern for parameters for parameterized types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    /// Parameter name, used for named struct/schema pseudotype elements. Names
    /// are not matched; they are only used for evaluation.
    pub name: Option<String>,

    /// Pattern for the value that the parameter is set to, if not skipped.
    /// Some(Pattern::Any) is special-cased to also match skipped parameters.
    pub value: Option<Pattern>,
}

impl Describe for Option<Pattern> {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            None => write!(f, "null"),
            Some(value) => value.describe(f, limit),
        }
    }
}

impl Describe for Parameter {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match &self.name {
            None => self.value.describe(f, limit),
            Some(name) => {
                let (name_limit, type_limit) = limit.split(name.len());
                util::string::describe_identifier(f, name, name_limit)?;
                write!(f, ": ")?;
                self.value.describe(f, type_limit)
            }
        }
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Parameter {
    /// Returns a pattern that matches the given variation.
    pub fn exactly(param: data::Parameter) -> Self {
        Parameter {
            name: param.name,
            value: param.value.map(Pattern::exactly),
        }
    }

    /// Matches this pattern. Note the special case to let the ? pattern match
    /// nulls, and note that names are ignored.
    pub fn match_pattern_with_context(
        &self,
        context: &mut meta::Context,
        param: &data::Parameter,
    ) -> bool {
        match &self.value {
            None => {
                // The null pattern only matches nulls.
                param.value.is_none()
            }
            Some(pattern) => match &param.value {
                None => {
                    // Special case for nulls and ? to make ? match null.
                    matches!(pattern, Pattern::Any)
                }
                Some(value) => pattern.match_pattern_with_context(context, value),
            },
        }
    }

    /// Evaluates this pattern.
    pub fn evaluate_with_context(
        &self,
        context: &mut meta::Context,
    ) -> diagnostic::Result<data::Parameter> {
        Ok(data::Parameter {
            name: self.name.clone(),
            value: self
                .value
                .as_ref()
                .map(|x| x.evaluate_with_context(context))
                .transpose()?,
        })
    }
}
