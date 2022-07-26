// SPDX-License-Identifier: Apache-2.0

//! Module for dealing with Substrait's meta-type system, used for type
//! parameters.
//!
//! See [`MetaType`], [`MetaValue`], and [`MetaPattern`].

use crate::output::data_type;
use crate::output::extension;
use crate::util;
use crate::util::string::Describe;
use std::fmt::Write;
use std::sync::Arc;

/// Metatypes are the types used in Substrait's type parameter type system.
/// Notably, this type system includes data types as *values*, allowing you
/// to pass data types to type parameters to form nested types. While data
/// values only exist while a plan is executed, meta values ([`MetaValue`])
/// and operations thereupon only exist in the definition of a plan.
///
/// Note that the meta type system is much simpler than the data type system,
/// and cannot be extended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetaType {
    /// Special validator-specific value used when the type is unknown due to
    /// recovery from previous errors.
    Unresolved,

    /// Type for booleans and predicates. Syntax: `metabool` (to disambiguate
    /// from normal booleans).
    Boolean,

    /// Type for integers. Signed 64-bit integers are used. Syntax: `metaint`
    /// (to disambiguate from the normal integer types).
    Integer,

    /// Type for enumerations. The available options are stored along with
    /// patterns for simplicity. Note that this means that enums are not
    /// type-safe; they sit somewhere between strings and integers. Enum
    /// values must be valid identifiers in order to be usable. Syntax:
    /// `metaenum` (for consistency with the other types, even though there's
    /// no normal `enum`).
    Enum,

    /// Type for quoted UTF-8 strings. Syntax: `metastr` (to disambiguate from
    /// the normal string type).
    String,

    /// Type for data types. Syntax: `typename`.
    DataType,
}

impl std::fmt::Display for MetaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaType::Unresolved => write!(f, "!"),
            MetaType::Boolean => write!(f, "metabool"),
            MetaType::Integer => write!(f, "metaint"),
            MetaType::Enum => write!(f, "metaenum"),
            MetaType::String => write!(f, "metastr"),
            MetaType::DataType => write!(f, "typename"),
        }
    }
}

impl Default for MetaType {
    fn default() -> Self {
        MetaType::Unresolved
    }
}

/// The value enumeration for metatypes ([`MetaType`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MetaValue {
    /// Special validator-specific value used when the value is unknown due to
    /// recovery from previous errors.
    Unresolved,

    /// Used for boolean values. Syntax: `true` or `false`.
    Boolean(bool),

    /// Used for integer values. Syntax: a decimal integer, -?[1-9][0-9]*.
    Integer(i64),

    /// Used for enumeration values. Syntax: any identifier that doesn't map
    /// to a keyword, type, or previously bound pattern.
    Enum(String),

    /// Used for string values. Syntax: quoted string.
    String(String),

    /// Used for data type values. Syntax:
    /// `<class><null-suffix><variation-suffix><parameter-suffix>`.
    DataType(Arc<data_type::DataType>),
}

impl Describe for MetaValue {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            MetaValue::Unresolved => write!(f, "!"),
            MetaValue::Boolean(value) => write!(f, "{value}"),
            MetaValue::Integer(value) => write!(f, "{value}"),
            MetaValue::Enum(variant) => util::string::describe_identifier(f, variant, limit),
            MetaValue::String(value) => util::string::describe_string(f, value, limit),
            MetaValue::DataType(data_type) => data_type.describe(f, limit),
        }
    }
}

impl std::fmt::Display for MetaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Default for MetaValue {
    fn default() -> Self {
        MetaValue::Unresolved
    }
}

impl From<bool> for MetaValue {
    fn from(x: bool) -> Self {
        MetaValue::Boolean(x)
    }
}

impl From<i64> for MetaValue {
    fn from(x: i64) -> Self {
        MetaValue::Integer(x)
    }
}

impl From<String> for MetaValue {
    fn from(x: String) -> Self {
        MetaValue::String(x)
    }
}

impl From<Arc<data_type::DataType>> for MetaValue {
    fn from(x: Arc<data_type::DataType>) -> Self {
        MetaValue::DataType(x)
    }
}

impl MetaValue {
    /// Modifies the data type via the given map function, if this is a data
    /// type.
    pub fn map_data_type<F>(self, f: F) -> Self
    where
        F: FnOnce(Arc<data_type::DataType>) -> Arc<data_type::DataType>,
    {
        if let MetaValue::DataType(t) = self {
            MetaValue::DataType(f(t))
        } else {
            self
        }
    }

    /// Modifies the data type via the given map function, if this is a data
    /// type.
    pub fn map_data_type_result<F, E>(self, f: F) -> Result<Self, E>
    where
        F: FnOnce(Arc<data_type::DataType>) -> Result<Arc<data_type::DataType>, E>,
    {
        Ok(if let MetaValue::DataType(t) = self {
            MetaValue::DataType(f(t)?)
        } else {
            self
        })
    }

    /// If this value is a boolean, returns the boolean.
    pub fn get_boolean(&self) -> Option<bool> {
        if let MetaValue::Boolean(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    /// If this value is an integer, returns the integer.
    pub fn get_integer(&self) -> Option<i64> {
        if let MetaValue::Integer(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    /// If this value is an enum, returns the enum variant.
    pub fn get_enum(&self) -> Option<&str> {
        if let MetaValue::Enum(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this value is a string, returns the string.
    pub fn get_string(&self) -> Option<&str> {
        if let MetaValue::String(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this value is a data type, returns the data type.
    pub fn get_data_type(&self) -> Option<Arc<data_type::DataType>> {
        if let MetaValue::DataType(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    /// Returns the metatype of this metavalue.
    pub fn get_type(&self) -> MetaType {
        match self {
            MetaValue::Unresolved => MetaType::Unresolved,
            MetaValue::Boolean(_) => MetaType::Boolean,
            MetaValue::Integer(_) => MetaType::Integer,
            MetaValue::Enum(_) => MetaType::Enum,
            MetaValue::String(_) => MetaType::String,
            MetaValue::DataType(_) => MetaType::DataType,
        }
    }
}

/// Context for evaluating patterns and expressions.
#[derive(Clone, Debug, Default)]
pub struct Context {
    /// Named bindings that have been previously assigned or matched via
    /// patterns.
    pub bindings: std::collections::HashMap<String, MetaValue>,
}

/// Patterns are used wherever a metavalue is expected, such as for type
/// parameter slots.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MetaPattern {
    /// Accepts any metavalue. Syntax: `?`. Also used when a pattern is unknown
    /// due to validator-specific error recovery.
    Any,

    /// A binding. These act sort of like variables in a given context. When
    /// the binding is first matched against a value, it acts like Any and
    /// assumes that value. When it is matched again in the same context later,
    /// it only matches metavalues that are exactly equal to the previous
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

impl Describe for MetaPattern {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            MetaPattern::Any => write!(f, "?"),
            MetaPattern::Binding(name) => util::string::describe_identifier(f, name, limit),
            MetaPattern::Boolean(None) => write!(f, "metabool"),
            MetaPattern::Boolean(Some(true)) => write!(f, "true"),
            MetaPattern::Boolean(Some(false)) => write!(f, "false"),
            MetaPattern::Integer(i64::MIN, i64::MAX) => write!(f, "metaint"),
            MetaPattern::Integer(i64::MIN, max) => write!(f, "..{max}"),
            MetaPattern::Integer(min, i64::MAX) => write!(f, "{min}.."),
            MetaPattern::Integer(min, max) => write!(f, "{min}..{max}"),
            MetaPattern::Enum(None) => write!(f, "metaenum"),
            MetaPattern::Enum(Some(variants)) => {
                util::string::describe_sequence(f, variants, limit, 10, |f, variant, _, limit| {
                    util::string::describe_identifier(f, variant, limit)
                })
            }
            MetaPattern::String(None) => write!(f, "metastr"),
            MetaPattern::String(Some(text)) => util::string::describe_string(f, text, limit),
            MetaPattern::DataType(None) => write!(f, "typename"),
            MetaPattern::DataType(Some(pattern)) => pattern.describe(f, limit),
        }
    }
}

impl std::fmt::Display for MetaPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl MetaPattern {
    /// Returns a pattern that matches the given value exactly.
    pub fn exactly(value: MetaValue) -> Self {
        match value {
            MetaValue::Unresolved => MetaPattern::Any,
            MetaValue::Boolean(x) => MetaPattern::Boolean(Some(x)),
            MetaValue::Integer(x) => MetaPattern::Integer(x, x),
            MetaValue::Enum(x) => MetaPattern::Enum(Some(vec![x])),
            MetaValue::String(x) => MetaPattern::String(Some(x)),
            MetaValue::DataType(x) => MetaPattern::DataType(Some(DataTypePattern::exactly(x))),
        }
    }

    /// Returns a pattern that matches the given type exactly.
    pub fn exactly_type(meta_type: MetaType) -> Self {
        match meta_type {
            MetaType::Unresolved => MetaPattern::Any,
            MetaType::Boolean => MetaPattern::Boolean(None),
            MetaType::Integer => MetaPattern::Integer(i64::MIN, i64::MAX),
            MetaType::Enum => MetaPattern::Enum(None),
            MetaType::String => MetaPattern::String(None),
            MetaType::DataType => MetaPattern::DataType(None),
        }
    }

    /// Matches this pattern without any provided context.
    pub fn match_pattern(&self, value: &MetaValue) -> bool {
        let mut context = Context::default();
        self.match_pattern_with_context(&mut context, value)
    }

    /// Matches this pattern with a provided context.
    pub fn match_pattern_with_context(&self, context: &mut Context, value: &MetaValue) -> bool {
        match self {
            MetaPattern::Any => true,
            MetaPattern::Binding(name) => {
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
            MetaPattern::Boolean(expected) => {
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
            MetaPattern::Integer(low, high) => {
                if let Some(value) = value.get_integer() {
                    value >= *low && value <= *high
                } else {
                    false
                }
            }
            MetaPattern::Enum(expected) => {
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
            MetaPattern::String(expected) => {
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
            MetaPattern::DataType(expected) => {
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
    pub class: data_type::Class,

    /// Nullability, defined using a (boolean) metapattern. Syntax:
    ///  - no suffix: Boolean(Some(false))
    ///  - `?` suffix: Boolean(Some(true))
    ///  - `??` suffix: Boolean(None)
    ///  - `?<identifier>` suffix: Binding(_)
    pub nullable: Arc<MetaPattern>,

    /// Type variation pattern.
    pub variation: VariationPattern,

    /// Type parameters for non-simple types.
    ///  - None: any number of parameters is accepted (type class willing).
    ///    Syntax: no suffix.
    ///  - Some: the number of parameters must match the length of the
    ///    contained vector, and the parameters themselves must match the given
    ///    patterns. Syntax: `<>`-enclosed list of patterns. None can be used
    ///    to only match an explicitly undefined optional parameter.
    pub parameters: Option<Vec<Option<MetaPattern>>>,
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
    pub fn exactly(value: Arc<data_type::DataType>) -> Self {
        DataTypePattern {
            class: value.class().clone(),
            nullable: Arc::new(MetaPattern::exactly(MetaValue::from(value.nullable()))),
            variation: VariationPattern::Exactly(value.variation().clone()),
            parameters: Some(
                value
                    .parameters()
                    .iter()
                    .cloned()
                    .map(|x| match x {
                        data_type::Parameter::Null => None,
                        data_type::Parameter::Some(_, value) => Some(MetaPattern::exactly(value)),
                    })
                    .collect(),
            ),
        }
    }

    /// Matches this pattern with a provided context.
    pub fn match_pattern_with_context(
        &self,
        context: &mut Context,
        value: &Arc<data_type::DataType>,
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
                    data_type::Parameter::Null => {
                        // null only matches null and ?.
                        if !matches!(expected, None | Some(MetaPattern::Any)) {
                            return false;
                        }
                    }
                    data_type::Parameter::Some(_, value) => {
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
    Exactly(data_type::Variation),
}

impl std::fmt::Display for VariationPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariationPattern::Any => write!(f, "[?]"),
            VariationPattern::Compatible => Ok(()),
            VariationPattern::Exactly(None) => write!(f, "[0]"),
            VariationPattern::Exactly(Some(variation)) => write!(f, "[{variation}]"),
        }
    }
}

impl VariationPattern {
    /// Matches this pattern with a provided context.
    pub fn match_pattern(&self, value: &data_type::Variation) -> bool {
        match self {
            VariationPattern::Any => true,
            VariationPattern::Compatible => {
                if let Some(variation) = value {
                    if let Some(definition) = &variation.definition {
                        definition.function_behavior == extension::FunctionBehavior::Inherits
                    } else {
                        // Assume "inherits" behavior if the definition is
                        // unknown. Whenever a definition is unknown the
                        // validator should already have warned when the
                        // variation was declared, and it's the default
                        // anyway.
                        true
                    }
                } else {
                    true
                }
            }
            VariationPattern::Exactly(expected) => value == expected,
        }
    }
}
