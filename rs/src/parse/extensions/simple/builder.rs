// SPDX-License-Identifier: Apache-2.0

//! Module providing a builder structure to be used while parsing a simple
//! extension file.

use crate::output::extension;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct Builder {
    /// Unique identifier for this extension.
    pub extension_id: u64,

    /// Description of the extension.
    pub description: String,

    /// The URI that was actually used to resolve this extension.
    pub actual_uri: String,

    /// Map with references to dependencies.
    pub dependencies: HashMap<String, extension::simple::module::Reference>,

    /// Namespace used for type classes defined in this extension and its
    /// dependencies.
    pub type_classes: extension::simple::type_class::NamespaceDefinition,

    /// Namespace used for type variations defined in this extension and its
    /// dependencies.
    pub type_variations: extension::simple::type_variation::NamespaceDefinition,

    /// Namespace used for functions defined in this extension and its
    /// dependencies. Both simple and compound names are registered.
    pub function_impls: extension::simple::function::NamespaceDefinition,
}

impl From<Builder> for extension::simple::module::Definition {
    fn from(builder: Builder) -> Self {
        extension::simple::module::Definition {
            extension_id: builder.extension_id,
            description: builder.description,
            actual_uri: builder.actual_uri,
            dependencies: builder.dependencies,
            type_classes: Arc::new(builder.type_classes),
            type_variations: Arc::new(builder.type_variations),
            function_impls: Arc::new(builder.function_impls),
        }
    }
}

impl extension::simple::module::Scope for Builder {
    /// Resolves a to-be-resolved reference to a type class.
    fn resolve_type_class<T>(&self, name: T) -> extension::simple::type_class::ResolutionResult
    where
        T: Into<extension::simple::type_class::UnresolvedReference>,
    {
        self.type_classes.resolve_local(name.into())
    }

    /// Resolves a to-be-resolved reference to a type variation.
    fn resolve_type_variation<T>(
        &self,
        name: T,
    ) -> extension::simple::type_variation::ResolutionResult
    where
        T: Into<extension::simple::type_variation::UnresolvedReference>,
    {
        self.type_variations.resolve_local(name.into())
    }

    /// Resolves a to-be-resolved reference to a function.
    fn resolve_function<T>(&self, name: T) -> extension::simple::function::ResolutionResult
    where
        T: Into<extension::simple::function::UnresolvedReference>,
    {
        self.function_impls.resolve_local(name.into())
    }
}
