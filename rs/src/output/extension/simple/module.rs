// SPDX-License-Identifier: Apache-2.0

//! Module for representing simple extension modules/files.

use crate::output::extension;
use std::collections::HashMap;

/// Trait for structs that represent extension modules, providing functions for
/// resolving names in them.
pub trait Scope {
    /// Resolves a to-be-resolved reference to a type class.
    fn resolve_type_class<T>(&self, name: T) -> extension::simple::type_class::ResolutionResult
    where
        T: Into<extension::simple::type_class::UnresolvedReference>;

    /// Resolves a to-be-resolved reference to a type variation.
    fn resolve_type_variation<T>(
        &self,
        name: T,
    ) -> extension::simple::type_variation::ResolutionResult
    where
        T: Into<extension::simple::type_variation::UnresolvedReference>;

    /// Resolves a to-be-resolved reference to a function.
    fn resolve_function<T>(&self, name: T) -> extension::simple::function::ResolutionResult
    where
        T: Into<extension::simple::function::UnresolvedReference>;
}

/// Same as [Scope], but object-safe.
pub trait DynScope {
    /// Resolves a to-be-resolved reference to a type class.
    fn resolve_type_class_from_ref(
        &self,
        name: extension::simple::type_class::UnresolvedReference,
    ) -> extension::simple::type_class::ResolutionResult;

    /// Resolves a to-be-resolved reference to a type variation.
    fn resolve_type_variation_from_ref(
        &self,
        name: extension::simple::type_variation::UnresolvedReference,
    ) -> extension::simple::type_variation::ResolutionResult;

    /// Resolves a to-be-resolved reference to a function.
    fn resolve_function_from_ref(
        &self,
        name: extension::simple::function::UnresolvedReference,
    ) -> extension::simple::function::ResolutionResult;
}

impl<T: Scope> DynScope for T {
    fn resolve_type_class_from_ref(
        &self,
        name: extension::simple::type_class::UnresolvedReference,
    ) -> extension::simple::type_class::ResolutionResult {
        self.resolve_type_class(name)
    }

    fn resolve_type_variation_from_ref(
        &self,
        name: extension::simple::type_variation::UnresolvedReference,
    ) -> extension::simple::type_variation::ResolutionResult {
        self.resolve_type_variation(name)
    }

    fn resolve_function_from_ref(
        &self,
        name: extension::simple::function::UnresolvedReference,
    ) -> extension::simple::function::ResolutionResult {
        self.resolve_function(name)
    }
}

/// A parsed simple extension module/file.
#[derive(Clone, Debug, Default)]
pub struct Definition {
    /// Unique number within the tree that can be used to refer to this
    /// extension when exporting in protobuf form.
    pub extension_id: u64,

    /// Description of the module.
    pub description: String,

    /// The URI that was actually used to resolve the module.
    pub actual_uri: String,

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

impl Scope for Definition {
    /// Resolves a to-be-resolved reference to a type class.
    fn resolve_type_class<T>(&self, name: T) -> extension::simple::type_class::ResolutionResult
    where
        T: Into<extension::simple::type_class::UnresolvedReference>,
    {
        self.type_classes.resolve_public(name.into())
    }

    /// Resolves a to-be-resolved reference to a type variation.
    fn resolve_type_variation<T>(
        &self,
        name: T,
    ) -> extension::simple::type_variation::ResolutionResult
    where
        T: Into<extension::simple::type_variation::UnresolvedReference>,
    {
        self.type_variations.resolve_public(name.into())
    }

    /// Resolves a to-be-resolved reference to a function.
    fn resolve_function<T>(&self, name: T) -> extension::simple::function::ResolutionResult
    where
        T: Into<extension::simple::function::UnresolvedReference>,
    {
        self.function_impls.resolve_public(name.into())
    }
}

/// A potentially unresolved reference to a module. Includes the URI even if
/// unresolved. The name fields of ExtensionReference are unused.
pub type Reference = extension::Reference<Definition>;

impl Scope for Reference {
    /// Resolves a to-be-resolved reference to a type class.
    fn resolve_type_class<T>(&self, name: T) -> extension::simple::type_class::ResolutionResult
    where
        T: Into<extension::simple::type_class::UnresolvedReference>,
    {
        let reference = name.into();
        self.definition
            .as_ref()
            .map(|x| x.resolve_type_class(reference.clone()))
            .unwrap_or_else(|| extension::simple::type_class::ResolutionResult::new(reference))
    }

    /// Resolves a to-be-resolved reference to a type variation.
    fn resolve_type_variation<T>(
        &self,
        name: T,
    ) -> extension::simple::type_variation::ResolutionResult
    where
        T: Into<extension::simple::type_variation::UnresolvedReference>,
    {
        let reference = name.into();
        self.definition
            .as_ref()
            .map(|x| x.resolve_type_variation(reference.clone()))
            .unwrap_or_else(|| extension::simple::type_variation::ResolutionResult::new(reference))
    }

    /// Resolves a to-be-resolved reference to a function.
    fn resolve_function<T>(&self, name: T) -> extension::simple::function::ResolutionResult
    where
        T: Into<extension::simple::function::UnresolvedReference>,
    {
        let reference = name.into();
        self.definition
            .as_ref()
            .map(|x| x.resolve_function(reference.clone()))
            .unwrap_or_else(|| extension::simple::function::ResolutionResult::new(reference))
    }
}
