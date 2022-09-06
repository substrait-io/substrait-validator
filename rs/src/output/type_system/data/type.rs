// SPDX-License-Identifier: Apache-2.0

//! Module for [`Type`].

use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::output::type_system::data::class::ParameterInfo;
use crate::output::type_system::meta;
use crate::util;
use crate::util::string::Describe;
use std::fmt::Write;
use std::sync::Arc;

/// Types are copied around a lot and are recursive, so they're almost always
/// reference-counted.
pub type Type = Arc<Definition>;

/// A Substrait data type. Includes facilities for storing unresolved or
/// partially-resolved types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Definition {
    /// Type class (simple, compound, or user-defined).
    class: data::Class,

    /// Nullability.
    nullable: bool,

    /// Type variation, if any.
    variation: data::Variation,

    /// Type parameters for non-simple types.
    parameters: Vec<data::Parameter>,
}

impl Describe for Definition {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        let mut name = String::new();
        write!(&mut name, "{}", self.class)?;
        if self.nullable {
            write!(&mut name, "?")?;
        }
        if let data::Variation::UserDefined(variation) = &self.variation {
            write!(&mut name, "[{variation}]")?;
        }
        write!(f, "{}", name)?;
        let (_, limit) = limit.split(name.len());
        if self.class.has_parameters() {
            write!(f, "<")?;
            util::string::describe_sequence(
                f,
                &self.parameters,
                limit,
                20,
                |f, param, _, limit| param.describe(f, limit),
            )?;
            write!(f, ">")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Definition {
    /// Creates a new type.
    pub fn new(
        class: data::Class,
        nullable: bool,
        variation: data::Variation,
        parameters: Vec<data::Parameter>,
    ) -> diagnostic::Result<Type> {
        // Check whether class and parameters work together.
        class.check_parameters(&parameters)?;

        // Check whether the specified type variation is applicable to this
        // type.
        match &variation {
            data::Variation::SystemPreferred => (),
            data::Variation::UserDefined(variation) => {
                if let Some(definition) = &variation.definition {
                    if !definition.base.weak_equals(&class) {
                        return Err(cause!(
                            TypeMismatchedVariation,
                            "variation {variation} is derived from {}, not {class}",
                            definition.base
                        ));
                    }
                }
            }
        }

        Ok(Arc::new(Definition {
            class,
            nullable,
            variation,
            parameters,
        }))
    }

    /// Returns a nullable variant of this type.
    pub fn make_nullable(&self) -> Arc<Definition> {
        Arc::new(Definition {
            class: self.class.clone(),
            nullable: true,
            variation: self.variation.clone(),
            parameters: self.parameters.clone(),
        })
    }

    /// Returns a variant of this type with the nullability overridden as
    /// specified.
    pub fn override_nullable(&self, nullable: bool) -> Arc<Definition> {
        Arc::new(Definition {
            class: self.class.clone(),
            nullable,
            variation: self.variation.clone(),
            parameters: self.parameters.clone(),
        })
    }

    /// Returns the type class.
    pub fn class(&self) -> &data::Class {
        &self.class
    }

    /// Returns whether the type is nullable.
    pub fn nullable(&self) -> bool {
        self.nullable
    }

    /// Returns the type variation.
    pub fn variation(&self) -> &data::Variation {
        &self.variation
    }

    /// Returns the type parameters.
    pub fn parameters(&self) -> &Vec<data::Parameter> {
        &self.parameters
    }

    /// Returns the value of the given boolean parameter.
    pub fn boolean_parameter(&self, index: usize) -> Option<bool> {
        self.parameters
            .get(index)
            .and_then(|x| x.value.as_ref())
            .and_then(meta::Value::get_boolean)
    }

    /// Returns the value of the given integer parameter.
    pub fn integer_parameter(&self, index: usize) -> Option<i64> {
        self.parameters
            .get(index)
            .and_then(|x| x.value.as_ref())
            .and_then(meta::Value::get_integer)
    }

    /// Returns the value of the given enum parameter.
    pub fn enum_parameter(&self, index: usize) -> Option<&str> {
        self.parameters
            .get(index)
            .and_then(|x| x.value.as_ref())
            .and_then(meta::Value::get_enum)
    }

    /// Returns the value of the given string parameter.
    pub fn string_parameter(&self, index: usize) -> Option<&str> {
        self.parameters
            .get(index)
            .and_then(|x| x.value.as_ref())
            .and_then(meta::Value::get_string)
    }

    /// Returns the value of the given type parameter.
    pub fn data_type_parameter(&self, index: usize) -> Option<Arc<Definition>> {
        self.parameters
            .get(index)
            .and_then(|x| x.value.as_ref())
            .and_then(meta::Value::get_data_type)
    }

    /// Returns whether this is an unresolved type.
    pub fn is_unresolved(&self) -> bool {
        matches!(self.class, data::Class::Unresolved)
    }

    /// Returns whether any part of this type tree is an unresolved type.
    pub fn is_unresolved_deep(&self) -> bool {
        self.is_unresolved()
            || self.parameters.iter().any(|p| match &p.value {
                Some(meta::Value::Unresolved) => true,
                Some(meta::Value::DataType(t)) => t.is_unresolved_deep(),
                _ => false,
            })
    }

    /// Returns whether this is a STRUCT or NSTRUCT type.
    pub fn is_struct(&self) -> bool {
        matches!(
            self.class,
            data::Class::Compound(data::class::Compound::Struct)
                | data::Class::Compound(data::class::Compound::NamedStruct)
        )
    }

    /// Returns Some(Vec<T>)) when this is a STRUCT or NSTRUCT type, where the
    /// vector contains the field types. Returns None otherwise.
    pub fn unwrap_struct(&self) -> Option<Vec<Arc<Definition>>> {
        if self.is_struct() {
            Some(
                self.parameters
                    .iter()
                    .map(|x| {
                        x.value
                            .as_ref()
                            .and_then(meta::Value::get_data_type)
                            .unwrap_or_default()
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    /// Returns Some(T) when this is a STRUCT or NSTRUCT type with only a
    /// single element of type T, or None otherwise.
    pub fn unwrap_singular_struct(&self) -> Option<Arc<Definition>> {
        if self.is_struct() && self.parameters.len() == 1 {
            self.data_type_parameter(0)
        } else {
            None
        }
    }

    /// Returns whether this is a LIST type.
    pub fn is_list(&self) -> bool {
        matches!(
            self.class,
            data::Class::Compound(data::class::Compound::List)
        )
    }

    /// Returns Some(T) when this is a LIST type with element type T, or None
    /// otherwise.
    pub fn unwrap_list(&self) -> Option<Arc<Definition>> {
        if self.is_list() {
            self.data_type_parameter(0)
        } else {
            None
        }
    }

    /// Returns whether this is a MAP type.
    pub fn is_map(&self) -> bool {
        matches!(
            self.class,
            data::Class::Compound(data::class::Compound::Map)
        )
    }

    /// Returns Some(T) when this is a MAP type with value type T, or None
    /// otherwise.
    pub fn unwrap_map(&self) -> Option<Arc<Definition>> {
        if self.is_map() {
            self.data_type_parameter(1)
        } else {
            None
        }
    }

    /// Returns Some(T) when this is a MAP type with key type T, or None
    /// otherwise.
    pub fn unwrap_map_key(&self) -> Option<Arc<Definition>> {
        if self.is_map() {
            self.data_type_parameter(0)
        } else {
            None
        }
    }

    /// Returns the type of the nth field of this struct. Returns None if
    /// out of range or if this is known to not be a struct.
    pub fn index_struct(&self, index: usize) -> Option<Arc<Definition>> {
        if self.is_unresolved() {
            Some(data::new_unresolved_type())
        } else if self.is_struct() {
            self.data_type_parameter(index)
        } else {
            None
        }
    }

    /// Internal helper for split_field_names() and strip_field_names().
    fn split_field_names_internal<F: FnMut(String)>(&self, namer: &mut F) -> Arc<Definition> {
        let is_struct = self.is_struct();
        let parameters = self
            .parameters
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, p)| {
                let p = if is_struct {
                    let (p, name) = p.split_name();
                    namer(name.unwrap_or_else(|| i.to_string()));
                    p
                } else {
                    p
                };
                p.map(|v| v.map_data_type(|t| t.split_field_names_internal(namer)))
            })
            .collect();
        let class = if self.class == data::Class::Compound(data::class::Compound::NamedStruct) {
            data::Class::Compound(data::class::Compound::Struct)
        } else {
            self.class.clone()
        };
        Arc::new(Definition {
            class,
            nullable: self.nullable,
            variation: self.variation.clone(),
            parameters,
        })
    }

    /// Converts all NSTRUCT types in the tree to STRUCT, and returns the
    /// flattened list of field names encountered. The fields of STRUCT types
    /// are also returned, to ensure that the returned Vec is applicable to
    /// apply_field_names(); their names are simply their zero-based index
    /// converted to a string.
    pub fn split_field_names(&self) -> (Arc<Definition>, Vec<String>) {
        let mut names = vec![];
        let data_type = self.split_field_names_internal(&mut |s| names.push(s));
        (data_type, names)
    }

    /// Like split_field_names(), but drops the name strings.
    pub fn strip_field_names(&self) -> Arc<Definition> {
        self.split_field_names_internal(&mut |_| ())
    }

    /// Internal helper function for apply_field_names().
    fn apply_field_names_internal<F: FnMut() -> diagnostic::Result<String>>(
        &self,
        namer: &mut F,
    ) -> diagnostic::Result<Arc<Definition>> {
        if self.is_struct() {
            let parameters: Result<Vec<_>, _> = self
                .parameters
                .iter()
                .cloned()
                .map(|mut p| {
                    p.name.replace(namer()?);
                    p.map_result(|v| {
                        v.map_data_type_result(|t| t.apply_field_names_internal(namer))
                    })
                })
                .collect();

            // The data type may be invalid after renaming, so we need to
            // call new() to perform check validity.
            data::new_type(
                data::Class::Compound(data::class::Compound::NamedStruct),
                self.nullable,
                self.variation.clone(),
                parameters?,
            )
        } else {
            let parameters: Result<Vec<_>, _> = self
                .parameters
                .iter()
                .cloned()
                .map(|p| {
                    p.map_result(|v| {
                        v.map_data_type_result(|t| t.apply_field_names_internal(namer))
                    })
                })
                .collect();

            // Data types generated this way can never become invalid, so we
            // can construct directly.
            Ok(Arc::new(Definition {
                class: self.class.clone(),
                nullable: self.nullable,
                variation: self.variation.clone(),
                parameters: parameters?,
            }))
        }
    }

    /// Applies names to STRUCTs, or renames the names in NSTRUCTs, based on a
    /// flattened vector of names.
    pub fn apply_field_names<S: ToString>(
        &self,
        names: &[S],
    ) -> diagnostic::Result<Arc<Definition>> {
        let mut names = names.iter();
        let mut num_too_few = 0;
        let mut namer = || {
            Ok(names.next().map(|s| s.to_string()).unwrap_or_else(|| {
                num_too_few += 1;
                format!("unnamed{num_too_few}")
            }))
        };
        let new_type = self.apply_field_names_internal(&mut namer)?;
        let remainder = names.count();
        if self.is_unresolved_deep() {
            Ok(new_type)
        } else if remainder > 0 {
            Err(cause!(
                TypeMismatchedFieldNameAssociations,
                "received {remainder} too many field name(s)"
            ))
        } else if num_too_few > 0 {
            Err(cause!(
                TypeMismatchedFieldNameAssociations,
                "received {num_too_few} too few field name(s)"
            ))
        } else {
            Ok(new_type)
        }
    }
}

impl Default for Definition {
    fn default() -> Self {
        Definition {
            class: data::Class::Unresolved,
            nullable: false,
            variation: data::Variation::SystemPreferred,
            parameters: vec![],
        }
    }
}
