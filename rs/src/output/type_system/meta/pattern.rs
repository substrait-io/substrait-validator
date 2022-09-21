// SPDX-License-Identifier: Apache-2.0

//! Module for [`Pattern`]s that can be used to match one or more meta values
//! for type and bounds checking, or can be evaluated (reverse direction) to
//! obtain procedurally-generated value.

use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::util;
use crate::util::string::describe_identifier;
use crate::util::string::Describe;
use std::fmt::Write;
use std::sync::Arc;

/// Trait for patterns that can match a particular value or be evaluated to
/// one.
pub trait Pattern {
    /// The value type that this pattern matches.
    type Value;

    /// Returns a pattern that matches the given value exactly.
    fn exactly(value: Self::Value) -> Self;

    /// Matches this pattern without any provided context.
    fn match_pattern(&self, value: &Self::Value) -> diagnostic::Result<bool> {
        let mut context = meta::Context::default();
        self.match_pattern_with_context(&mut context, value)
    }

    /// Matches this pattern with a provided context.
    fn match_pattern_with_context(
        &self,
        context: &mut meta::Context,
        value: &Self::Value,
    ) -> diagnostic::Result<bool>;

    /// Evaluates this pattern without any provided context.
    fn evaluate(&self) -> diagnostic::Result<Self::Value> {
        let mut context = meta::Context::default();
        self.evaluate_with_context(&mut context)
    }

    /// Evaluates this pattern with a provided context.
    fn evaluate_with_context(&self, context: &mut meta::Context)
        -> diagnostic::Result<Self::Value>;

    /// Returns whether this pattern can be evaluated under ideal conditions.
    fn can_evaluate(&self) -> bool;
}

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
pub enum Value {
    /// Used when a pattern is unknown due to validator-specific error
    /// recovery. Matches all values, and evaluates to an unresolved value.
    /// Syntax (only for printing): `!`.
    Unresolved,

    /// Accepts any meta::Value. Cannot be evaluated. Syntax: `?`.
    Any,

    /// A binding. These act sort of like variables in a given context. Four
    /// variations on the matching and evaluation rules exist; see [Binding].
    Binding(Binding),

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

    /// Pattern for data types. Syntax:
    /// `<class><null-pattern><variation-pattern><parameter-pattern>` or
    /// `typename<null-pattern>`.
    DataType(DataType),

    /// A function applied to a number of patterns. Functions cannot be
    /// matched; they can only be evaluated. Syntax:
    /// `<function-name>(<args>, ...)`, as well as expression-style syntactic
    /// sugar for some of the functions:
    ///  - Boolean arithmetic: `a && b`, `a || b`, `!a`;
    ///  - Integer arithmetic: `a + b`, `a - b`, `a * b`, `a / b`, `-a`, `-b`;
    ///  - Comparisons: `a == b`, `a != b`, `a < b`, `a <= b`, `a > b`,
    ///    `a >= b`;
    ///  - Ternary: `if a then b else c`.
    Function(meta::Function, Vec<Value>),
}

impl Describe for Value {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            Value::Unresolved => write!(f, "!"),
            Value::Any => write!(f, "?"),
            Value::Binding(binding) => binding.describe(f, limit),
            Value::Boolean(None) => write!(f, "metabool"),
            Value::Boolean(Some(true)) => write!(f, "true"),
            Value::Boolean(Some(false)) => write!(f, "false"),
            Value::Integer(i64::MIN, i64::MAX) => write!(f, "metaint"),
            Value::Integer(i64::MIN, max) => write!(f, "..{max}"),
            Value::Integer(min, i64::MAX) => write!(f, "{min}.."),
            Value::Integer(min, max) => write!(f, "{min}..{max}"),
            Value::Enum(None) => write!(f, "metaenum"),
            Value::Enum(Some(variants)) => {
                util::string::describe_sequence(f, variants, limit, 10, |f, variant, _, limit| {
                    util::string::describe_identifier(f, variant, limit)
                })
            }
            Value::String(None) => write!(f, "metastr"),
            Value::String(Some(text)) => util::string::describe_string(f, text, limit),
            Value::DataType(pattern) => pattern.describe(f, limit),
            Value::Function(func, args) => {
                write!(f, "{func}(")?;
                util::string::describe_sequence(f, args, limit, 10, |f, arg, _, limit| {
                    arg.describe(f, limit)
                })
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Unresolved
    }
}

impl Value {
    /// Match the given value without being lenient about unresolved values.
    /// Whenever this returns false, the public match_pattern_with_context()
    /// function will check if the value was unresolved, and override the
    /// result with true if so; unresolved values should always match
    /// everything in order to avoid flooding the user with error messages
    /// when the validator is confused due to a previous error.
    fn match_strictly(
        &self,
        context: &mut meta::Context,
        value: &meta::Value,
    ) -> diagnostic::Result<bool> {
        Ok(match self {
            Value::Unresolved => true,
            Value::Any => true,
            Value::Binding(binding) => binding.match_strictly(context, value)?,
            Value::Boolean(expected) => {
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
            Value::Integer(low, high) => {
                if let Some(value) = value.get_integer() {
                    value >= *low && value <= *high
                } else {
                    false
                }
            }
            Value::Enum(expected) => {
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
            Value::String(expected) => {
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
            Value::DataType(expected) => {
                if let Some(value) = value.get_data_type() {
                    expected.match_pattern_with_context(context, &value)?
                } else {
                    false
                }
            }
            Value::Function(func, args) => Value::exactly(func.evaluate(context, args)?)
                .match_pattern_with_context(context, value)?,
        })
    }

    /// Returns a pattern that matches the given type exactly.
    pub fn exactly_type(meta_type: meta::Type) -> Self {
        match meta_type {
            meta::Type::Unresolved => Value::Any,
            meta::Type::Boolean => Value::Boolean(None),
            meta::Type::Integer => Value::Integer(i64::MIN, i64::MAX),
            meta::Type::Enum => Value::Enum(None),
            meta::Type::String => Value::String(None),
            meta::Type::DataType => Value::DataType(DataType {
                class: None,
                nullable: Arc::new(Value::Boolean(None)),
                variation: Variation::Compatible,
                parameters: None,
            }),
        }
    }

    /// Returns what type this pattern matches or evaluates to. If unknown or
    /// multiple types can be matched, yield unresolved.
    pub fn determine_type(&self) -> meta::Type {
        match self {
            Value::Unresolved | Value::Any => meta::Type::Unresolved,
            Value::Binding(binding) => binding.determine_type(),
            Value::Boolean(_) => meta::Type::Boolean,
            Value::Integer(_, _) => meta::Type::Integer,
            Value::Enum(_) => meta::Type::Enum,
            Value::String(_) => meta::Type::String,
            Value::DataType(_) => meta::Type::DataType,
            Value::Function(function, arguments) => function.determine_type(arguments),
        }
    }
}

impl Pattern for Value {
    type Value = meta::Value;

    fn exactly(value: Self::Value) -> Self {
        match value {
            meta::Value::Unresolved => Value::Unresolved,
            meta::Value::Boolean(x) => Value::Boolean(Some(x)),
            meta::Value::Integer(x) => Value::Integer(x, x),
            meta::Value::Enum(x) => Value::Enum(Some(vec![x])),
            meta::Value::String(x) => Value::String(Some(x)),
            meta::Value::DataType(x) => Value::DataType(DataType::exactly(x)),
        }
    }

    fn match_pattern_with_context(
        &self,
        context: &mut meta::Context,
        value: &Self::Value,
    ) -> diagnostic::Result<bool> {
        Ok(self.match_strictly(context, value)? || value.is_unresolved())
    }

    fn evaluate_with_context(
        &self,
        context: &mut meta::Context,
    ) -> diagnostic::Result<Self::Value> {
        match self {
            Value::Unresolved => Ok(meta::Value::Unresolved),
            Value::Any => Err(cause!(
                TypeDerivationInvalid,
                "? patterns cannot be evaluated"
            )),
            Value::Binding(binding) => binding.evaluate_with_context(context),
            Value::Boolean(value) => {
                if let Some(value) = value {
                    Ok((*value).into())
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "cannot evaluate boolean with unknown value"
                    ))
                }
            }
            Value::Integer(low, high) => {
                if low == high {
                    Ok((*low).into())
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "cannot evaluate integer with unknown value"
                    ))
                }
            }
            Value::Enum(values) => {
                if let Some(values) = values {
                    if values.len() == 1 {
                        Ok(meta::Value::Enum(values[0].clone()))
                    } else {
                        Err(cause!(
                            TypeDerivationInvalid,
                            "cannot evaluate enum with unknown value"
                        ))
                    }
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "cannot evaluate undefined enum"
                    ))
                }
            }
            Value::String(value) => {
                if let Some(value) = value {
                    Ok(value.clone().into())
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "cannot evaluate string with unknown value"
                    ))
                }
            }
            Value::DataType(value) => value.evaluate_with_context(context).map(meta::Value::from),
            Value::Function(func, args) => func.evaluate(context, args),
        }
    }

    fn can_evaluate(&self) -> bool {
        match self {
            Value::Unresolved => true,
            Value::Any => false,
            Value::Binding(_) => true,
            Value::Boolean(x) => x.is_some(),
            Value::Integer(a, b) => a == b,
            Value::Enum(x) => x.is_some(),
            Value::String(x) => x.is_some(),
            Value::DataType(x) => x.can_evaluate(),
            Value::Function(_, _) => true,
        }
    }
}

/// Binding matching structure. Four variations exist, as detailed below.
///
/// ## Consistent bindings without nullability suffix
///
/// (inconsistent = false, nullability = None)
///
/// When the binding is first matched against a value, it matches any
/// non-typename metavalue and any non-nullable typename and binds the name to
/// that value. When it is matched again in the same context later, it only
/// matches metavalues that are exactly equal to the previous match. When
/// evaluated, it evaluates to the value that the name was bound to, or fails
/// if it was not bound. Syntax: any identifier that isn't recognized as
/// anything else.
///
/// ## Inconsistent bindings without nullability suffix
///
/// (inconsistent = true, nullability = None)
///
/// Behaves exactly like a consistent binding without nullability, except for
/// the following.
///
///  - When matched and the name is already bound, it still matches any
///    non-typename metavalue and any non-nullable typename. This is used for
///    inconsistently-typed variadic argument slots.
///  - When matched against `true` and the name was previously bound to
///    `false`, the name is rebound to `true`. This is used to handle
///    `MIRROR` nullability behavior.
///  - When evaluated while the name is not yet bound, yields `false` as a
///    default value instead of failing. This is, again, used to handle
///    `MIRROR` nullability behavior.
///
/// Syntax: a `?` followed by any identifier.
///    
/// ## Consistent bindings with nullability suffix
///
/// (inconsistent = false, nullability = Some(pattern))
///
/// A normal binding, but with overrides for nullability. Both the incoming
/// and (if any) previously bound value must be typenames. The bound typename
/// is always the non-nullable variant of the matched typename. When matching
/// against a previously bound typename, the nullability field of said typename
/// is ignored; instead, the nullability of the matched typename is always
/// matched against the contained nullability pattern. When evaluating, the
/// nullability of the previously bound value is overridden by the nullability
/// as evaluated by the contained pattern. Otherwise, the rules are the same
/// as for normal bindings.
///
/// ## Inconsistent bindings with nullability suffix
///
/// (inconsistent = true, nullability = Some(pattern))
///
/// Behaves exactly like a consistent binding with nullability suffix, except
/// that any previously bound value is ignored when matching. The name is never
/// rebound if it was previously bound.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binding {
    /// The name of the binding, using its original case convention. Note that
    /// bindings are matched case-insensitively.
    pub name: String,

    /// Whether the binding uses consistent or inconsistent matching rules.
    pub inconsistent: bool,

    /// Whether this is a normal binding or a binding with nullability
    /// override, and if the latter, the nullability pattern.
    pub nullability: Option<Arc<Value>>,
}

impl Describe for Binding {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        if self.inconsistent {
            write!(f, "?")?;
        }
        if let Some(nullability) = &self.nullability {
            let (a, b) = limit.split(self.name.len());
            describe_identifier(f, &self.name, a)?;
            match nullability.as_ref() {
                Value::Boolean(Some(false)) => write!(f, "!")?,
                Value::Boolean(Some(true)) => write!(f, "?")?,
                Value::Boolean(None) => write!(f, "??")?,
                x => {
                    write!(f, "?")?;
                    x.describe(f, b)?;
                }
            }
        } else {
            describe_identifier(f, &self.name, limit)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Binding {
    /// Match the given binding without being lenient about unresolved values.
    /// Whenever this returns false, the public match_pattern_with_context()
    /// function will check if the value was unresolved, and override the
    /// result with true if so; unresolved values should always match
    /// everything in order to avoid flooding the user with error messages
    /// when the validator is confused due to a previous error.
    pub fn match_strictly(
        &self,
        context: &mut meta::Context,
        value: &meta::Value,
    ) -> diagnostic::Result<bool> {
        // If nullability is specified, the value must be a data type and its
        // nullability must match the pattern.
        if let Some(nullability) = &self.nullability {
            if let meta::Value::DataType(dt) = value {
                if !nullability.match_pattern(&dt.nullable().into())? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        // Handle the rest of the matching process.
        if let Some(current) = context.bindings.get(&self.name.to_ascii_lowercase()) {
            if self.inconsistent {
                // Handle the special rule for handling MIRROR nullability.
                if current == &meta::Value::Boolean(false) && value == &meta::Value::Boolean(true) {
                    context
                        .bindings
                        .insert(self.name.to_ascii_lowercase(), meta::Value::Boolean(true));
                }

                // Match anything.
                Ok(true)
            } else if self.nullability.is_some() {
                // Match all parts of the metavalue *except* nullability.
                if let meta::Value::DataType(current) = current {
                    DataType::exactly(current.clone()).match_strictly(
                        context,
                        &value.get_data_type().unwrap_or_default(),
                        true,
                    )
                } else {
                    Ok(false)
                }
            } else {
                // Match the complete metavalues exactly.
                Value::exactly(current.clone()).match_strictly(context, value)
            }
        } else {
            // Bind the non-nullable version of the incoming value to the name.
            let value_to_bind = if let meta::Value::DataType(dt) = value {
                meta::Value::DataType(dt.override_nullable(false))
            } else {
                value.clone()
            };
            context
                .bindings
                .insert(self.name.to_ascii_lowercase(), value_to_bind);

            // Match anything.
            Ok(true)
        }
    }

    pub fn evaluate_with_context(
        &self,
        context: &mut meta::Context,
    ) -> diagnostic::Result<meta::Value> {
        if let Some(nullability) = &self.nullability {
            // Yield the current value of the binding, augmented with the
            // nullability field.
            if let Some(current) = context.bindings.get(&self.name.to_ascii_lowercase()) {
                if let meta::Value::DataType(current) = current {
                    let nullability = nullability.evaluate()?;
                    if let meta::Value::Boolean(nullability) = nullability {
                        Ok(current.override_nullable(nullability).into())
                    } else {
                        Err(cause!(
                            TypeDerivationInvalid,
                            "nullability pattern must yield a metabool, but yielded {nullability}"
                        ))
                    }
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{} must be a data type, but was bound to {current}",
                        &self.name
                    ))
                }
            } else {
                Err(cause!(
                    TypeDerivationInvalid,
                    "{} was never bound",
                    &self.name
                ))
            }
        } else if let Some(current) = context.bindings.get(&self.name.to_ascii_lowercase()) {
            Ok(current.clone())
        } else if self.inconsistent {
            Ok(false.into())
        } else {
            Err(cause!(
                TypeDerivationInvalid,
                "{} was never bound",
                &self.name
            ))
        }
    }

    /// Returns what type this pattern matches or evaluates to. If unknown or
    /// multiple types can be matched, yield unresolved.
    pub fn determine_type(&self) -> meta::Type {
        if self.nullability.is_some() {
            meta::Type::DataType
        } else {
            meta::Type::Unresolved
        }
    }
}

/// Data type matching structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataType {
    /// Type class (simple, compound, or user-defined) to be matched, if any.
    /// The syntax for omitting the class name is to use the `typename` keyword
    /// instead. In this case, the variation and parameter pack fields are
    /// unused, and must be set to [Variation::Compatible] and None
    /// respectively.
    pub class: Option<data::Class>,

    /// Nullability, defined using a (boolean) Pattern. Syntax:
    ///  - no suffix: Boolean(Some(false))
    ///  - `!` suffix: Boolean(Some(false))
    ///  - `?` suffix: Boolean(Some(true))
    ///  - `??` suffix: Boolean(None)
    ///  - `?<identifier>` suffix: Binding(_)
    ///  - `??<identifier>` suffix: InconsistentBinding(_)
    pub nullable: Arc<Value>,

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
        if let Some(class) = &self.class {
            write!(&mut non_recursive, "{}", class)?;
        } else {
            write!(&mut non_recursive, "typename")?;
        }
        match self.nullable.as_ref() {
            Value::Boolean(Some(false)) => (),
            Value::Boolean(Some(true)) => write!(&mut non_recursive, "?")?,
            Value::Boolean(None) => write!(&mut non_recursive, "??")?,
            x => write!(&mut non_recursive, "?{x}")?,
        }
        write!(&mut non_recursive, "{}", self.variation)?;
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
    /// Match the given value without being lenient about unresolved values.
    /// Whenever this returns false, the public match_pattern_with_context()
    /// function will check if the value was unresolved, and override the
    /// result with true if so; unresolved values should always match
    /// everything in order to avoid flooding the user with error messages
    /// when the validator is confused due to a previous error.
    fn match_strictly(
        &self,
        context: &mut meta::Context,
        value: &data::Type,
        ignore_nullability: bool,
    ) -> diagnostic::Result<bool> {
        if !ignore_nullability
            && !self
                .nullable
                .match_pattern_with_context(context, &value.nullable().into())?
        {
            return Ok(false);
        }
        if let Some(class) = &self.class {
            if !value.class().weak_equals(class) {
                return Ok(false);
            }
            if !self.variation.match_pattern(value.variation())? {
                return Ok(false);
            }
            if let Some(expected) = &self.parameters {
                let parameters = value.parameters();
                if parameters.len() != expected.len() {
                    return Ok(false);
                }
                for (parameter, expected) in parameters.iter().zip(expected.iter()) {
                    if !expected.match_pattern_with_context(context, parameter)? {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }
}

impl Pattern for DataType {
    type Value = data::Type;

    fn exactly(value: Self::Value) -> Self {
        DataType {
            class: Some(value.class().clone()),
            nullable: Arc::new(Value::exactly(meta::Value::from(value.nullable()))),
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

    fn match_pattern_with_context(
        &self,
        context: &mut meta::Context,
        value: &Self::Value,
    ) -> diagnostic::Result<bool> {
        Ok(self.match_strictly(context, value, false)? || value.is_unresolved())
    }

    fn evaluate_with_context(
        &self,
        context: &mut meta::Context,
    ) -> diagnostic::Result<Self::Value> {
        let class = if let Some(class) = self.class.clone() {
            class
        } else {
            return Err(cause!(
                TypeDerivationInvalid,
                "typename pattern cannot be evaluated"
            ));
        };
        let nullable = self
            .nullable
            .evaluate_with_context(context)?
            .get_boolean()
            .ok_or_else(|| {
                cause!(
                    TypeDerivationInvalid,
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

    fn can_evaluate(&self) -> bool {
        if let Some(parameters) = &self.parameters {
            if !parameters.iter().all(|x| x.can_evaluate()) {
                return false;
            }
        }
        self.class.is_some() && self.nullable.can_evaluate() && self.variation.can_evaluate()
    }
}

/// Type variation matching structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Variation {
    /// Matches any variation. Syntax: `[?]` suffix.
    Any,

    /// Matches any variation that is compatible with the system-preferred
    /// variation; that is, matches the system-preferred variation and any
    /// variation declared with INHERITS function behavior. Syntax: no
    /// suffix.
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

impl Pattern for Variation {
    type Value = data::Variation;

    fn exactly(value: Self::Value) -> Self {
        Variation::Exactly(value)
    }

    fn match_pattern_with_context(
        &self,
        _context: &mut meta::Context,
        value: &Self::Value,
    ) -> diagnostic::Result<bool> {
        Ok(match self {
            Variation::Any => true,
            Variation::Compatible => value.is_compatible_with_system_preferred(),
            Variation::Exactly(expected) => value == expected,
        })
    }

    fn evaluate_with_context(
        &self,
        _context: &mut meta::Context,
    ) -> diagnostic::Result<Self::Value> {
        match self {
            Variation::Any => Err(cause!(
                TypeDerivationInvalid,
                "cannot evaluate undefined variation"
            )),
            Variation::Compatible => Ok(data::Variation::SystemPreferred),
            Variation::Exactly(expected) => Ok(expected.clone()),
        }
    }

    fn can_evaluate(&self) -> bool {
        match self {
            Variation::Any => false,
            Variation::Compatible | Variation::Exactly(_) => true,
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
    pub value: Option<Value>,
}

impl Describe for Option<Value> {
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

impl Default for Parameter {
    fn default() -> Self {
        Self {
            name: None,
            value: Some(Value::Unresolved),
        }
    }
}

impl Pattern for Parameter {
    type Value = data::Parameter;

    fn exactly(param: Self::Value) -> Self {
        Parameter {
            name: param.name,
            value: param.value.map(Value::exactly),
        }
    }

    /// Matches this pattern. Note the special case to let the ? pattern match
    /// nulls, and note that names are ignored.
    fn match_pattern_with_context(
        &self,
        context: &mut meta::Context,
        param: &Self::Value,
    ) -> diagnostic::Result<bool> {
        Ok(match &self.value {
            None => {
                // The null pattern only matches nulls.
                param.value.is_none()
            }
            Some(pattern) => match &param.value {
                None => {
                    // Special case for nulls and ? to make ? match null.
                    matches!(pattern, Value::Any)
                }
                Some(value) => pattern.match_pattern_with_context(context, value)?,
            },
        })
    }

    fn evaluate_with_context(
        &self,
        context: &mut meta::Context,
    ) -> diagnostic::Result<Self::Value> {
        Ok(data::Parameter {
            name: self.name.clone(),
            value: self
                .value
                .as_ref()
                .map(|x| x.evaluate_with_context(context))
                .transpose()?,
        })
    }

    fn can_evaluate(&self) -> bool {
        if let Some(value) = &self.value {
            value.can_evaluate()
        } else {
            // Evaluates to null.
            true
        }
    }
}
