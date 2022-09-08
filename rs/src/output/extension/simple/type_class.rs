// SPDX-License-Identifier: Apache-2.0

//! Module for representing simple type class extensions.

use crate::output::diagnostic;
use crate::output::extension;
use crate::output::type_system::data;
use crate::output::type_system::data::class::ParameterInfo;
use crate::output::type_system::meta;
use crate::output::type_system::meta::pattern::Pattern;

/// A definition of a user-defined type class.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Definition {
    /// Unique number within the tree that can be used to refer to this
    /// extension when exporting in protobuf form.
    pub extension_id: u64,

    /// Description of the type class.
    pub description: String,

    /// The underlying structure of the type.
    pub structure: Vec<(String, data::class::Simple)>,

    /// The parameters expected by the data type.
    pub parameter_slots: Vec<ParameterSlot>,

    /// Whether or not the last parameter slot is variadic.
    pub parameters_variadic: bool,
}

/// A parameter slot for a user-defined data type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterSlot {
    /// YAML-provided name of the parameter.
    pub name: String,

    /// YAML-provided human-readable description of the parameter.
    pub description: String,

    /// Pattern for type- and bounds-checking parameters bound to this slot.
    pub pattern: meta::pattern::Value,

    /// Whether this parameter is optional. If optional, it may be skipped
    /// using null or omitted entirely if at the end of the list.
    pub optional: bool,
}

impl ParameterInfo for extension::reference::Reference<Definition> {
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

impl ParameterInfo for Definition {
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
            if param.name.is_some() {
                return Err(cause!(
                    TypeMismatchedParameters,
                    "parameter {} cannot be named",
                    self.parameter_name_or_index(index)
                ));
            }
            if let Some(value) = &param.value {
                if !slot.pattern.match_pattern(value)? {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "parameter {} does not match pattern {}",
                        self.parameter_name_or_index(index),
                        slot.pattern
                    ));
                }
            } else if !slot.optional {
                return Err(cause!(
                    TypeMismatchedParameters,
                    "parameter {} is not optional and can thus not be skipped with null",
                    self.parameter_name_or_index(index)
                ));
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

/// A reference to a completed type class namespace.
pub type NamespaceReference = extension::namespace::Reference<Definition>;

/// A potentially mutable type class namespace definition.
pub type NamespaceDefinition = extension::namespace::Definition<Definition>;

/// A to-be-resolved reference to a user-defined type class. Includes the name
/// and URI even if unresolved.
pub type UnresolvedReference = extension::reference::Data<Definition>;

/// The result of a name resolution. May consist of any number of definitions
/// that are ambiguously referred to, though for type classes this should
/// never happen for valid plans.
pub type ResolutionResult = extension::namespace::ResolutionResult<Definition>;

/// A potentially unresolved reference to a user-defined type class. Includes
/// the name and URI even if unresolved.
pub type Reference = extension::Reference<Definition>;
