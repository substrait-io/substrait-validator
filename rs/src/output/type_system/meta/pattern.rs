// SPDX-License-Identifier: Apache-2.0

//! Module for [`Pattern`]s that can be used to match one or more meta values,
//! for type and bounds checking.

use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::util;
use crate::util::string::Describe;
use std::fmt::Write;
use std::sync::Arc;

/// Patterns are used wherever a meta::Value is expected, such as for type
/// parameter slots.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pattern {
    /// Accepts any meta::Value. Syntax: `?`. Also used when a pattern is unknown
    /// due to validator-specific error recovery.
    Any,

    /// A binding. These act sort of like variables in a given context. When
    /// the binding is first matched against a value, it acts like Any and
    /// assumes that value. When it is matched again in the same context later,
    /// it only matches meta::Values that are exactly equal to the previous
    /// match. Syntax: any identifier that isn't recognized as anything else.
    Binding(String),

    /// Pattern for booleans.
    ///  - None: both true and false match the pattern. Syntax: `metabool`
    ///  - Some(true): only true matches the pattern. Syntax: `true`
    ///  - Some(false): only false matches the pattern. Syntax: `false`
    Boolean(Option<bool>),

    /// Pattern for integers. Only integers within the given inclusive
    /// range match the pattern. Syntax: `metaint`, `<int>..`, `..<int>`, or
    /// `<int>..<int>`.
    Integer(i64, i64),

    /// Pattern for enumerations.
    ///  - None: any enum value is accepted. Syntax: `metaenum`.
    ///  - Some(options): only enum values that case-insensitively
    ///    match one of the given options are accepted. Syntax:
    ///    `{<identifier,+>}`.
    Enum(Option<Vec<String>>),

    /// Pattern for strings.
    ///  - None: matches any string. Syntax: `metastr`.
    ///  - Some(_): matches the given string exactly. Syntax: quoted string.
    String(Option<String>),

    /// Pattern for data types.
    ///  - None: matches any data type. Syntax: `typename`.
    ///  - Some(_): matches what the contained pattern matches. Syntax:
    ///    `<class><null-pattern><variation-pattern><parameter-pattern>`.
    DataType(Option<DataTypePattern>),
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
            meta::Value::DataType(x) => Pattern::DataType(Some(DataTypePattern::exactly(x))),
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
        }
    }
}

/// Data type matching structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTypePattern {
    /// Type class (simple, compound, or user-defined). This must always match
    /// exactly. [DataTypePattern]s are wrapped in an Option if the class can
    /// also be omitted.
    pub class: data::Class,

    /// Nullability, defined using a (boolean) Pattern. Syntax:
    ///  - no suffix: Boolean(Some(false))
    ///  - `?` suffix: Boolean(Some(true))
    ///  - `??` suffix: Boolean(None)
    ///  - `?<identifier>` suffix: Binding(_)
    pub nullable: Arc<Pattern>,

    /// Type variation pattern.
    pub variation: VariationPattern,

    /// Type parameters for non-simple types.
    ///  - None: any number of parameters is accepted (type class willing).
    ///    Syntax: no suffix.
    ///  - Some: the number of parameters must match the length of the
    ///    contained vector, and the parameters themselves must match the given
    ///    patterns. Syntax: `<>`-enclosed list of patterns. None can be used
    ///    to only match an explicitly undefined optional parameter.
    pub parameters: Option<Vec<Option<Pattern>>>,
}

impl Describe for DataTypePattern {
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
                if let Some(param) = param {
                    param.describe(f, limit)
                } else {
                    write!(f, "null")
                }
            })?;
            write!(f, ">")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for DataTypePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl DataTypePattern {
    /// Returns a pattern that matches the given pattern exactly.
    pub fn exactly(value: data::Type) -> Self {
        DataTypePattern {
            class: value.class().clone(),
            nullable: Arc::new(Pattern::exactly(meta::Value::from(value.nullable()))),
            variation: VariationPattern::Exactly(value.variation().clone()),
            parameters: Some(
                value
                    .parameters()
                    .iter()
                    .cloned()
                    .map(|x| match x {
                        data::Parameter::Null => None,
                        data::Parameter::Some(_, value) => Some(Pattern::exactly(value)),
                    })
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
                match parameter {
                    data::Parameter::Null => {
                        // null only matches null and ?.
                        if !matches!(expected, None | Some(Pattern::Any)) {
                            return false;
                        }
                    }
                    data::Parameter::Some(_, value) => {
                        if let Some(expected) = expected {
                            if !expected.match_pattern_with_context(context, value) {
                                return false;
                            }
                        } else {
                            // Expected null, found non-null.
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

/// Type variation matching structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariationPattern {
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

impl std::fmt::Display for VariationPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariationPattern::Any => write!(f, "[?]"),
            VariationPattern::Compatible => Ok(()),
            VariationPattern::Exactly(variation) => write!(f, "[{variation}]"),
        }
    }
}

impl VariationPattern {
    /// Matches this pattern with a provided context.
    pub fn match_pattern(&self, value: &data::Variation) -> bool {
        match self {
            VariationPattern::Any => true,
            VariationPattern::Compatible => value.is_compatible_with_system_preferred(),
            VariationPattern::Exactly(expected) => value == expected,
        }
    }
}
