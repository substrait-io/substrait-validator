// SPDX-License-Identifier: Apache-2.0

//! Module for type classes.
//!
//! See [`Class`].

use crate::output::diagnostic;
use crate::output::extension;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use std::collections::HashSet;
use std::sync::Arc;
use strum_macros::{Display, EnumString};

/// Trait for checking the type parameters for a type class.
pub trait ParameterInfo {
    /// Checks whether the given parameter set is valid for this type class.
    fn check_parameters(&self, params: &[data::Parameter]) -> diagnostic::Result<()>;

    /// Returns the logical name of the given parameter. If the index is out of
    /// range or the name is unknown, yield None.
    fn parameter_name(&self, index: usize) -> Option<String>;

    /// Returns the logical name of the given parameter, returning its index
    /// if the name is not known.
    fn parameter_name_or_index(&self, index: usize) -> String {
        self.parameter_name(index)
            .unwrap_or_else(|| format!("{}", index))
    }

    /// Whether this type supports parameters. This is used to determine
    /// whether to print <> when the parameter list is empty. This is used to
    /// distinguish a concrete empty struct from a struct with unspecified
    /// fields. If it's unknown whether a type has parameters, false is to be
    /// returned.
    fn has_parameters(&self) -> bool;
}

/// Type class.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Class {
    /// Unresolved type. Used for error recovery.
    Unresolved,

    /// Well-known simple type.
    Simple(Simple),

    /// Well-known compound type.
    Compound(Compound),

    /// User-defined type.
    UserDefined(UserDefined),
}

impl Default for Class {
    fn default() -> Self {
        Class::Unresolved
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Simple(simple) => write!(f, "{simple}"),
            Class::Compound(compound) => write!(f, "{compound}"),
            Class::UserDefined(user_defined) => write!(f, "{user_defined}"),
            Class::Unresolved => write!(f, "!"),
        }
    }
}

impl ParameterInfo for Class {
    fn check_parameters(&self, params: &[data::Parameter]) -> diagnostic::Result<()> {
        match self {
            Class::Simple(_) => {
                if params.is_empty() {
                    Ok(())
                } else {
                    Err(cause!(
                        TypeMismatchedParameters,
                        "simple types cannot be parameterized"
                    ))
                }
            }
            Class::Compound(compound) => compound.check_parameters(params),
            Class::UserDefined(user_defined) => user_defined.check_parameters(params),
            Class::Unresolved => Ok(()),
        }
    }

    fn parameter_name(&self, index: usize) -> Option<String> {
        match self {
            Class::Compound(compound) => compound.parameter_name(index),
            Class::UserDefined(user_defined) => user_defined.parameter_name(index),
            _ => None,
        }
    }

    fn has_parameters(&self) -> bool {
        match self {
            Class::Compound(compound) => compound.has_parameters(),
            Class::UserDefined(user_defined) => user_defined.has_parameters(),
            _ => false,
        }
    }
}

impl Class {
    /// Checks whether two classes are equal, also returning true if either or
    /// both are unresolved.
    pub fn weak_equals(&self, rhs: &Class) -> bool {
        match (self, rhs) {
            (_, Class::Unresolved) | (Class::Unresolved, _) => true,
            (a, b) => a == b,
        }
    }
}

/// Enumeration of simple types defined by Substrait.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "snake_case")]
pub enum Simple {
    Boolean,
    I8,
    I16,
    I32,
    I64,
    Fp32,
    Fp64,
    String,
    Binary,
    Timestamp,
    TimestampTz,
    Date,
    Time,
    IntervalYear,
    IntervalDay,
    Uuid,
}

/// Enumeration of compound types defined by Substrait.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "UPPERCASE")]
pub enum Compound {
    FixedChar,
    VarChar,
    FixedBinary,
    Decimal,
    Struct,
    #[strum(serialize = "NSTRUCT")]
    NamedStruct,
    List,
    Map,
}

impl ParameterInfo for Compound {
    fn check_parameters(&self, params: &[data::Parameter]) -> diagnostic::Result<()> {
        match self {
            Compound::FixedChar | Compound::VarChar | Compound::FixedBinary => {
                if params.len() != 1 {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} expects a single parameter (length)"
                    ));
                }
                if let data::Parameter::Some(None, meta::Value::Integer(length)) = params[0] {
                    // Note: 2147483647 = 2^31-1 = maximum value for signed
                    // 32-bit integer. However, the significance of the number
                    // is just that the Substrait specification says this is
                    // the limit.
                    const MIN_LENGTH: i64 = 1;
                    const MAX_LENGTH: i64 = 2147483647;
                    if !(MIN_LENGTH..=MAX_LENGTH).contains(&length) {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} length {length} is out of range {MIN_LENGTH}..{MAX_LENGTH}"
                        ));
                    }
                } else {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} length parameter must be a positive integer"
                    ));
                }
            }
            Compound::Decimal => {
                if params.len() != 2 {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} expects two parameters (precision and scale)"
                    ));
                }
                if let data::Parameter::Some(None, meta::Value::Integer(precision)) = params[0] {
                    const MIN_PRECISION: i64 = 1;
                    const MAX_PRECISION: i64 = 38;
                    if !(MIN_PRECISION..=MAX_PRECISION).contains(&precision) {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} precision {precision} is out of range {MIN_PRECISION}..{MAX_PRECISION}"
                        ));
                    }
                    if let data::Parameter::Some(None, meta::Value::Integer(scale)) = params[1] {
                        if scale < 0 || scale > precision {
                            return Err(cause!(
                                TypeMismatchedParameters,
                                "{self} scale {scale} is out of range 0..{precision}"
                            ));
                        }
                    } else {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} scale parameter must be a positive integer"
                        ));
                    }
                } else {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} precision parameter must be a positive integer"
                    ));
                }
            }
            Compound::Struct => {
                for param in params.iter() {
                    if !matches!(param, data::Parameter::Some(None, meta::Value::DataType(_))) {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} parameters must be types"
                        ));
                    }
                }
            }
            Compound::NamedStruct => {
                let mut names = HashSet::with_capacity(params.len());
                for param in params.iter() {
                    if let data::Parameter::Some(Some(name), meta::Value::DataType(_)) = &param {
                        if !names.insert(name) {
                            return Err(cause!(
                                TypeMismatchedParameters,
                                "duplicate field name in {self}: {name}"
                            ));
                        }
                    } else {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} parameters must be name-types pairs"
                        ));
                    }
                }
            }
            Compound::List => {
                if params.len() != 1 {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} expects a single parameter (element type)"
                    ));
                }
                if !matches!(
                    params[0],
                    data::Parameter::Some(None, meta::Value::DataType(_))
                ) {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} element type parameter must be a type"
                    ));
                }
            }
            Compound::Map => {
                if params.len() != 2 {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} expects two parameters (key type and value type)"
                    ));
                }
                if !matches!(
                    params[0],
                    data::Parameter::Some(None, meta::Value::DataType(_))
                ) {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} key type parameter must be a type"
                    ));
                }
                if !matches!(
                    params[1],
                    data::Parameter::Some(None, meta::Value::DataType(_))
                ) {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} value type parameter must be a type"
                    ));
                }
            }
        }
        Ok(())
    }

    fn parameter_name(&self, index: usize) -> Option<String> {
        match (self, index) {
            (Compound::FixedChar, 0) => Some(String::from("length")),
            (Compound::VarChar, 0) => Some(String::from("length")),
            (Compound::FixedBinary, 0) => Some(String::from("length")),
            (Compound::Decimal, 0) => Some(String::from("precision")),
            (Compound::Decimal, 1) => Some(String::from("scale")),
            (Compound::Struct, i) => Some(format!("{}", i)),
            (Compound::NamedStruct, i) => Some(format!("{}", i)),
            (Compound::List, 0) => Some(String::from("element")),
            (Compound::Map, 0) => Some(String::from("key")),
            (Compound::Map, 1) => Some(String::from("value")),
            (_, _) => None,
        }
    }

    fn has_parameters(&self) -> bool {
        true
    }
}

/// Typedef for user-defined type class references.
pub type UserDefined = Arc<extension::Reference<UserDefinedDefinition>>;

/// User-defined type class.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UserDefinedDefinition {
    /// The underlying structure of the type.
    pub structure: Vec<(String, Simple)>,

    /// The parameters expected by the data type.
    pub parameter_slots: Vec<DataTypeParameterSlot>,

    /// Whether or not the last parameter slot is variadic.
    pub parameters_variadic: bool,
}

/// A parameter slot for a user-defined data type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTypeParameterSlot {
    /// YAML-provided name of the parameter.
    pub name: String,

    /// YAML-provided human-readable description of the parameter.
    pub description: String,

    /// Pattern for type- and bounds-checking parameters bound to this slot.
    pub pattern: meta::Pattern,

    /// Whether this parameter is optional. If optional, it may be skipped
    /// using null or omitted entirely if at the end of the list.
    pub optional: bool,
}

impl ParameterInfo for extension::Reference<UserDefinedDefinition> {
    fn check_parameters(&self, params: &[data::Parameter]) -> diagnostic::Result<()> {
        self.definition
            .as_ref()
            .map(|d| d.check_parameters(params))
            .unwrap_or(Ok(()))
    }

    fn parameter_name(&self, index: usize) -> Option<String> {
        self.definition
            .as_ref()
            .map(|d| d.parameter_name(index))
            .unwrap_or_default()
    }

    fn has_parameters(&self) -> bool {
        self.definition
            .as_ref()
            .map(|d| d.has_parameters())
            .unwrap_or_default()
    }
}

impl ParameterInfo for UserDefinedDefinition {
    fn check_parameters(&self, params: &[data::Parameter]) -> diagnostic::Result<()> {
        // Determine the minimum number of parameters and check whether we have
        // enough.
        let min_parameters = self
            .parameter_slots
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, slot)| if slot.optional { None } else { Some(index + 1) })
            .unwrap_or_default();
        if params.len() < min_parameters {
            return Err(cause!(
                TypeMismatchedParameters,
                "need at least {min_parameters} parameter(s), but only {} was/were provided",
                params.len()
            ));
        }

        // Match parameters to slots positionally.
        for (index, param) in params.iter().enumerate() {
            // Determine the slot that corresponds to this parameter.
            let slot = self
                .parameter_slots
                .get(index)
                .or_else(|| {
                    if self.parameters_variadic {
                        self.parameter_slots.last()
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    cause!(
                        TypeMismatchedParameters,
                        "type expects at most {index} parameters, but {} were provided",
                        params.len()
                    )
                })?;

            // Check the provided parameter against the information contained
            // in the slot.
            match param {
                data::Parameter::Null => {
                    if !slot.optional {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "parameter {} is not optional and can thus not be skipped with null",
                            self.parameter_name_or_index(index)
                        ));
                    }
                }
                data::Parameter::Some(_, value) => {
                    if !slot.pattern.match_pattern(value) {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "parameter {} does not match pattern {}",
                            self.parameter_name_or_index(index),
                            slot.pattern
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn parameter_name(&self, index: usize) -> Option<String> {
        if self.parameters_variadic && index + 1 >= self.parameter_slots.len() {
            if let Some(slot) = self.parameter_slots.last() {
                return Some(if slot.name.is_empty() {
                    format!("{}", index)
                } else {
                    format!("{}.{}", slot.name, index + 1 - self.parameter_slots.len())
                });
            }
        }
        self.parameter_slots.get(index).map(|slot| {
            if slot.name.is_empty() {
                format!("{}", index)
            } else {
                slot.name.clone()
            }
        })
    }

    fn has_parameters(&self) -> bool {
        !self.parameter_slots.is_empty()
    }
}
