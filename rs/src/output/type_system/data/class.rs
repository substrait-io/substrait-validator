// SPDX-License-Identifier: Apache-2.0

//! Module for type classes.
//!
//! See [`Class`].

use crate::output::diagnostic;
use crate::output::extension;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use std::collections::HashSet;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;

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
    UserDefined(extension::simple::type_class::Reference),
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display, EnumString, EnumIter)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display, EnumString, EnumIter)]
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

/// Most parameters don't support a name and are mandatory. This function just
/// checks both these things and yields a reference to the value of the
/// parameter if it's valid.
fn check_normal_parameter<F: FnOnce() -> String>(
    describe: F,
    param: &data::Parameter,
) -> diagnostic::Result<&meta::Value> {
    if param.name.is_some() {
        Err(cause!(
            TypeMismatchedParameters,
            "{} does not support naming",
            describe()
        ))
    } else if let Some(value) = &param.value {
        Ok(value)
    } else {
        Err(cause!(
            TypeMismatchedParameters,
            "{} is mandatory",
            describe()
        ))
    }
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
                if let meta::Value::Integer(length) =
                    check_normal_parameter(|| format!("{self} length parameter"), &params[0])?
                {
                    // Note: 2147483647 = 2^31-1 = maximum value for signed
                    // 32-bit integer. However, the significance of the number
                    // is just that the Substrait specification says this is
                    // the limit.
                    const MIN_LENGTH: i64 = 1;
                    const MAX_LENGTH: i64 = 2147483647;
                    if !(MIN_LENGTH..=MAX_LENGTH).contains(length) {
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
                if let meta::Value::Integer(precision) =
                    check_normal_parameter(|| format!("{self} precision parameter"), &params[0])?
                {
                    const MIN_PRECISION: i64 = 1;
                    const MAX_PRECISION: i64 = 38;
                    if !(MIN_PRECISION..=MAX_PRECISION).contains(precision) {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} precision {precision} is out of range {MIN_PRECISION}..{MAX_PRECISION}"
                        ));
                    }
                    if let meta::Value::Integer(scale) =
                        check_normal_parameter(|| format!("{self} scale parameter"), &params[1])?
                    {
                        if *scale < 0 || scale > precision {
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
                    if param.name.is_some() {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} parameters do not support naming (did you mean to use NSTRUCT?)"
                        ));
                    }
                    if !matches!(param.value, Some(meta::Value::DataType(_))) {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} parameters are mandatory and must be types"
                        ));
                    }
                }
            }
            Compound::NamedStruct => {
                let mut names = HashSet::with_capacity(params.len());
                for param in params.iter() {
                    if let (Some(name), Some(meta::Value::DataType(_))) =
                        (&param.name, &param.value)
                    {
                        if !names.insert(name) {
                            return Err(cause!(
                                TypeMismatchedParameters,
                                "duplicate field name in {self}: {name}"
                            ));
                        }
                    } else {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} parameters ara mandatory and must be name-type pairs"
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
                    check_normal_parameter(
                        || format!("{self} element type parameter"),
                        &params[0]
                    )?,
                    meta::Value::DataType(_)
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
                    check_normal_parameter(|| format!("{self} key type parameter"), &params[0])?,
                    meta::Value::DataType(_)
                ) {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} key type parameter must be a type"
                    ));
                }
                if !matches!(
                    check_normal_parameter(|| format!("{self} value type parameter"), &params[1])?,
                    meta::Value::DataType(_)
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
