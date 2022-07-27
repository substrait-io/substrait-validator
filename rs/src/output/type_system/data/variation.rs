// SPDX-License-Identifier: Apache-2.0

//! Module for representing type [`Variation`]s.

use crate::output::extension;
use crate::output::type_system::data;
use std::sync::Arc;

/// A type variation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Variation {
    /// The special system-preferred variation, also known as \[0\].
    SystemPreferred,

    /// Reference to a user-defined variation.
    UserDefined(UserDefined),
}

impl Default for Variation {
    fn default() -> Self {
        Variation::SystemPreferred
    }
}

impl std::fmt::Display for Variation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variation::SystemPreferred => write!(f, "0"),
            Variation::UserDefined(reference) => write!(f, "{reference}"),
        }
    }
}

impl Variation {
    /// Returns true if this is the system-preferred variation.
    pub fn is_system_preferred(&self) -> bool {
        matches!(self, Variation::SystemPreferred)
    }

    /// Returns true if this is a user-defined variation.
    pub fn is_user_defined(&self) -> bool {
        matches!(self, Variation::UserDefined(_))
    }

    /// Returns true if this variation is compatible with the
    /// system-preferred variation. If the definition is unavailable, this
    /// guesses true, since this is the default and will generate less
    /// diagnostic spam (we'll already have warned about the definition not
    /// being available).
    pub fn is_compatible_with_system_preferred(&self) -> bool {
        match self {
            Variation::SystemPreferred => true,
            Variation::UserDefined(x) => x
                .definition
                .as_ref()
                .map(|x| x.function_behavior == FunctionBehavior::Inherits)
                .unwrap_or(true),
        }
    }
}

/// A reference to a user-defined type variation.
pub type UserDefined = Arc<extension::Reference<UserDefinedDefinition>>;

/// Type variation extension.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UserDefinedDefinition {
    /// The base type for this variation.
    pub base: data::Class,

    /// Function behavior for this variation.
    pub function_behavior: FunctionBehavior,
}

/// Type variation function behavior.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionBehavior {
    Inherits,
    Separate,
}

impl Default for FunctionBehavior {
    fn default() -> Self {
        FunctionBehavior::Inherits
    }
}
