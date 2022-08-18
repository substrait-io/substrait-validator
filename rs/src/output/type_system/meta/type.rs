// SPDX-License-Identifier: Apache-2.0

//! Module for meta [`Type`]s.

/// Metatypes are the types used in Substrait's type parameter type system.
/// Notably, this type system includes data types as *values*, allowing you
/// to pass data types to type parameters to form nested types. While data
/// values only exist while a plan is executed, meta values and operations
/// thereupon only exist in the definition of a plan.
///
/// Note that the meta type system is much simpler than the data type system,
/// and cannot be extended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unresolved => write!(f, "!"),
            Type::Boolean => write!(f, "metabool"),
            Type::Integer => write!(f, "metaint"),
            Type::Enum => write!(f, "metaenum"),
            Type::String => write!(f, "metastr"),
            Type::DataType => write!(f, "typename"),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Unresolved
    }
}
