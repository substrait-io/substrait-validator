// SPDX-License-Identifier: Apache-2.0

//! Module for type [`Parameter`]s.

use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::util;
use crate::util::string::Describe;

/// Parameter for parameterized types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    /// Parameter name, used for named struct/schema pseudotype elements.
    pub name: Option<String>,

    /// Value that the parameter is set to, if not skipped.
    pub value: Option<meta::Value>,
}

impl Describe for Option<meta::Value> {
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
        Parameter {
            name: None,
            value: Some(meta::Value::Unresolved),
        }
    }
}

impl Parameter {
    /// Constructor for placeholders for skipped parameters.
    pub fn null() -> Parameter {
        Parameter {
            name: None,
            value: None,
        }
    }

    /// Constructor for enum parameters. Note that the other types can be
    /// easily constructed using From/Into.
    pub fn enum_variant<S: ToString>(variant: S) -> Parameter {
        Parameter {
            name: None,
            value: Some(meta::Value::String(variant.to_string())),
        }
    }

    /// Returns the name of a named type parameter.
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| &x[..])
    }

    /// Splits the name annotation off from a named type parameter.
    pub fn split_name(self) -> (Parameter, Option<String>) {
        (
            Parameter {
                name: None,
                value: self.value,
            },
            self.name,
        )
    }

    /// Maps the value, if any.
    pub fn map<F>(self, f: F) -> Parameter
    where
        F: FnOnce(meta::Value) -> meta::Value,
    {
        Parameter {
            name: self.name,
            value: self.value.map(f),
        }
    }

    /// Maps the value, if any, passing through results.
    pub fn map_result<F, E>(self, f: F) -> Result<Parameter, E>
    where
        F: FnOnce(meta::Value) -> Result<meta::Value, E>,
    {
        Ok(Parameter {
            name: self.name,
            value: self.value.map(f).transpose()?,
        })
    }
}

impl From<bool> for Parameter {
    fn from(x: bool) -> Self {
        meta::Value::from(x).into()
    }
}

impl From<i64> for Parameter {
    fn from(x: i64) -> Self {
        meta::Value::from(x).into()
    }
}

impl From<String> for Parameter {
    fn from(x: String) -> Self {
        meta::Value::from(x).into()
    }
}

impl From<data::Type> for Parameter {
    fn from(x: data::Type) -> Self {
        meta::Value::from(x).into()
    }
}

impl From<meta::Value> for Parameter {
    fn from(value: meta::Value) -> Self {
        Parameter {
            name: None,
            value: Some(value),
        }
    }
}
