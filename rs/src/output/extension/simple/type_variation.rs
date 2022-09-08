// SPDX-License-Identifier: Apache-2.0

//! Module for representing simple type variation extensions.

use crate::output::extension;
use crate::output::type_system::data;

/// Type variation extension.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Definition {
    /// Unique number within the tree that can be used to refer to this
    /// extension when exporting in protobuf form.
    pub extension_id: u64,

    /// Description of the type variation.
    pub description: String,

    /// The base type for this variation.
    pub base: data::Class,

    /// Whether this variation is compatible with the "system-preferred"
    // variation for the purpose of (function argument) pattern matching.
    // Corresponds with the "functions" field in the YAML syntax; INHERITS
    // means compatible, SEPARATE means incompatible.
    pub compatible: bool,
}

/// A reference to a completed type variation namespace.
pub type NamespaceReference = extension::namespace::Reference<Definition>;

/// A potentially mutable type variation namespace definition.
pub type NamespaceDefinition = extension::namespace::Definition<Definition>;

/// A to-be-resolved reference to a type variation. Includes the name and URI
/// even if unresolved.
pub type UnresolvedReference = extension::reference::Data<Definition>;

/// The result of a name resolution. May consist of any number of definitions
/// that are ambiguously referred to. The results may be further refined at a
/// later stage. For type variations, this is used to let us define all
/// variations in the same namespace, instead of scoping them to the type class
/// they are defined for. We do this because at the time the name is resolved
/// (type variation anchor definition), we don't yet know what type class(es)
/// the name resolution will be used for. Any ambiguity is resolved when the
/// class is known, so the net result is the same.
pub type ResolutionResult = extension::namespace::ResolutionResult<Definition>;

/// A potentially unresolved reference to a type variation. Includes the name
/// and URI even if unresolved.
pub type Reference = extension::Reference<Definition>;
