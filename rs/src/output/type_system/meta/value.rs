// SPDX-License-Identifier: Apache-2.0

//! Module for meta [`Value`]s.

use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::util;
use crate::util::string::Describe;

/// The value enumeration for metatypes ([`meta::Type`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
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
    DataType(data::Type),
}

impl Describe for Value {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            Value::Unresolved => write!(f, "!"),
            Value::Boolean(true) => write!(f, "true"),
            Value::Boolean(false) => write!(f, "false"),
            Value::Integer(value) => write!(f, "{value}"),
            Value::Enum(variant) => util::string::describe_identifier(f, variant, limit),
            Value::String(value) => util::string::describe_string(f, value, limit),
            Value::DataType(data_type) => data_type.describe(f, limit),
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

impl From<bool> for Value {
    fn from(x: bool) -> Self {
        Value::Boolean(x)
    }
}

impl From<i64> for Value {
    fn from(x: i64) -> Self {
        Value::Integer(x)
    }
}

impl From<String> for Value {
    fn from(x: String) -> Self {
        Value::String(x)
    }
}

impl From<data::Type> for Value {
    fn from(x: data::Type) -> Self {
        Value::DataType(x)
    }
}

impl Value {
    /// Modifies the data type via the given map function, if this is a data
    /// type.
    pub fn map_data_type<F>(self, f: F) -> Self
    where
        F: FnOnce(data::Type) -> data::Type,
    {
        if let Value::DataType(t) = self {
            Value::DataType(f(t))
        } else {
            self
        }
    }

    /// Modifies the data type via the given map function, if this is a data
    /// type.
    pub fn map_data_type_result<F, E>(self, f: F) -> Result<Self, E>
    where
        F: FnOnce(data::Type) -> Result<data::Type, E>,
    {
        Ok(if let Value::DataType(t) = self {
            Value::DataType(f(t)?)
        } else {
            self
        })
    }

    /// Returns whether this is an unresolved value.
    pub fn is_unresolved(&self) -> bool {
        matches!(self, Value::Unresolved)
    }

    /// If this value is a boolean, returns the boolean.
    pub fn get_boolean(&self) -> Option<bool> {
        if let Value::Boolean(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    /// If this value is an integer, returns the integer.
    pub fn get_integer(&self) -> Option<i64> {
        if let Value::Integer(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    /// If this value is an enum, returns the enum variant.
    pub fn get_enum(&self) -> Option<&str> {
        if let Value::Enum(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this value is a string, returns the string.
    pub fn get_string(&self) -> Option<&str> {
        if let Value::String(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this value is a data type, returns the data type.
    pub fn get_data_type(&self) -> Option<data::Type> {
        match self {
            Value::Unresolved => Some(data::new_unresolved_type()),
            Value::DataType(x) => Some(x.clone()),
            _ => None,
        }
    }

    /// Returns the metatype of this metavalue.
    pub fn get_type(&self) -> meta::Type {
        match self {
            Value::Unresolved => meta::Type::Unresolved,
            Value::Boolean(_) => meta::Type::Boolean,
            Value::Integer(_) => meta::Type::Integer,
            Value::Enum(_) => meta::Type::Enum,
            Value::String(_) => meta::Type::String,
            Value::DataType(_) => meta::Type::DataType,
        }
    }
}
