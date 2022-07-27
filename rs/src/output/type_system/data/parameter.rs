// SPDX-License-Identifier: Apache-2.0

//! Module for type [`Parameter`]s.

use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::util;
use crate::util::string::Describe;

/// Parameter for parameterized types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Parameter {
    /// Null, to skip optional parameters.
    Null,

    /// Assigned parameter with an optional name attached. The name is used for
    /// named struct/schema pseudotype elements.
    Some(Option<String>, meta::Value),
}

impl Describe for Parameter {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            Parameter::Null => write!(f, "null"),
            Parameter::Some(None, dt) => dt.describe(f, limit),
            Parameter::Some(Some(name), dt) => {
                let (name_limit, type_limit) = limit.split(name.len());
                util::string::describe_identifier(f, name, name_limit)?;
                write!(f, ": ")?;
                dt.describe(f, type_limit)
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
        Parameter::Some(None, meta::Value::Unresolved)
    }
}

impl Parameter {
    /// Constructor for enum parameters. Note that the other types can be
    /// easily constructed using From/Into.
    pub fn enum_variant<S: ToString>(variant: S) -> Self {
        Parameter::Some(None, meta::Value::String(variant.to_string()))
    }

    /// Returns the name of a named type parameter.
    pub fn get_name(&self) -> Option<&str> {
        match self {
            Parameter::Some(Some(n), _) => Some(n),
            _ => None,
        }
    }

    /// Splits the name annotation off from a named type parameter.
    pub fn split_name(self) -> (Parameter, Option<String>) {
        match self {
            Parameter::Some(Some(n), t) => (Parameter::Some(None, t), Some(n)),
            p => (p, None),
        }
    }

    /// Annotates the parameter with a name, if applicable. If the parameter
    /// was already named, the name is replaced. The function is only called
    /// for Types and NamedTypes. None is returned only if the function was
    /// called and returned None.
    pub fn with_name<E, F: FnOnce() -> Result<String, E>>(self, f: F) -> Result<Parameter, E> {
        Ok(match self {
            Parameter::Some(_, meta::Value::DataType(t)) => {
                Parameter::Some(Some(f()?), meta::Value::DataType(t))
            }
            p => p,
        })
    }

    /// If this parameter is a boolean, returns the boolean.
    pub fn get_boolean(&self) -> Option<bool> {
        if let Parameter::Some(_, x) = self {
            x.get_boolean()
        } else {
            None
        }
    }

    /// If this parameter is an integer, returns the integer.
    pub fn get_integer(&self) -> Option<i64> {
        if let Parameter::Some(_, x) = self {
            x.get_integer()
        } else {
            None
        }
    }

    /// If this parameter is an enum, returns the enum variant.
    pub fn get_enum(&self) -> Option<&str> {
        if let Parameter::Some(_, x) = self {
            x.get_enum()
        } else {
            None
        }
    }

    /// If this parameter is a string, returns the string.
    pub fn get_string(&self) -> Option<&str> {
        if let Parameter::Some(_, x) = self {
            x.get_string()
        } else {
            None
        }
    }

    /// If this parameter is a data type, returns the data type.
    pub fn get_data_type(&self) -> Option<data::Type> {
        if let Parameter::Some(_, x) = self {
            x.get_data_type()
        } else {
            None
        }
    }

    /// Maps the value, if any.
    pub fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(meta::Value) -> meta::Value,
    {
        match self {
            Parameter::Some(n, v) => Parameter::Some(n, f(v)),
            x => x,
        }
    }

    /// Maps the value, if any, passing through results.
    pub fn map_result<F, E>(self, f: F) -> Result<Self, E>
    where
        F: FnOnce(meta::Value) -> Result<meta::Value, E>,
    {
        Ok(match self {
            Parameter::Some(n, v) => Parameter::Some(n, f(v)?),
            x => x,
        })
    }
}

impl From<bool> for Parameter {
    fn from(x: bool) -> Self {
        Parameter::Some(None, x.into())
    }
}

impl From<i64> for Parameter {
    fn from(x: i64) -> Self {
        Parameter::Some(None, x.into())
    }
}

impl From<String> for Parameter {
    fn from(x: String) -> Self {
        Parameter::Some(None, x.into())
    }
}

impl From<data::Type> for Parameter {
    fn from(x: data::Type) -> Self {
        Parameter::Some(None, x.into())
    }
}
