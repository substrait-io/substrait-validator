// SPDX-License-Identifier: Apache-2.0

//! Module for representing simple extension modules/files.

use crate::output::extension;
use std::collections::HashMap;

/// A parsed simple extension module/file.
#[derive(Clone, Debug, Default)]
pub struct Definition {
    /// Map with references to dependencies.
    pub dependencies: HashMap<String, Reference>,

    /// Reference to the namespace in which this module's type classes were
    /// defined. Unlike the type variation and function implementation
    /// namespaces, these should never be ambiguous, but we use the same
    /// mechanism anyway.
    pub type_classes: extension::simple::type_class::NamespaceReference,

    /// Reference to the namespace in which this module's type variations were
    /// defined. Note that variations are technically namespaced to the type
    /// class they were defined for; however, this would make type variation
    /// anchors very poorly defined. Instead, we pretend that they're all in
    /// the same namespace, but allow multiple definitions with the same name
    /// within our namespace system. Only when a type variation
    /// anchor/reference pair is referred to in the plan and the type class is
    /// known do we attempt to solve this ambiguity.
    pub type_variations: extension::simple::type_variation::NamespaceReference,

    /// Reference to the namespace in which this module's function
    /// implementations were defined. Each implementation is mapped twice: once
    /// for its simple function name, and once for its compound name. The
    /// former will be ambiguous if multiple implementations exist for a
    /// function, which we do indeed try to solve for using the type patterns,
    /// even though Substrait explicitly disallows referring to a function by
    /// its simple name if the function has multiple definitions. Likewise, we
    /// also solve this if a compound name is ambigous, again despite Substrait
    /// disallowing extensions from creating this situation.
    pub function_impls: extension::simple::function::NamespaceReference,
}

impl Definition {
    /// Resolves a to-be-resolved reference to a type class.
    pub fn resolve_type_class(
        &self,
        name: extension::simple::type_class::UnresolvedReference,
    ) -> extension::simple::type_class::ResolutionResult {
        self.type_classes.resolve_public(name)
    }

    /// Resolves a to-be-resolved reference to a type variation.
    pub fn resolve_type_variation(
        &self,
        name: extension::simple::type_variation::UnresolvedReference,
    ) -> extension::simple::type_variation::ResolutionResult {
        self.type_variations.resolve_public(name)
    }

    /// Resolves a to-be-resolved reference to a function.
    pub fn resolve_function(
        &self,
        name: extension::simple::function::UnresolvedReference,
    ) -> extension::simple::function::ResolutionResult {
        self.function_impls.resolve_public(name)
    }
}

/// A potentially unresolved reference to a module. Includes the URI even if
/// unresolved. The name fields of ExtensionReference are unused.
pub type Reference = extension::Reference<Definition>;
