// SPDX-License-Identifier: Apache-2.0

//! Module providing a builder structure to be used while parsing a simple
//! extension file.

use crate::input::yaml;
use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::path;
use crate::parse::context;
use crate::parse::extensions::simple::functions;
use crate::parse::extensions::simple::modules;
use crate::parse::extensions::simple::type_classes;
use crate::parse::extensions::simple::type_variations;
use crate::parse::traversal;
use crate::util;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Builder {
    /// Identifier for the extension.
    pub identifier: extension::simple::common::Identifier,

    /// Common metadata for the extension.
    pub metadata: extension::simple::common::Metadata,

    /// The URI that was actually used to resolve the module.
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

impl From<extension::simple::common::Identifier> for Builder {
    fn from(identifier: extension::simple::common::Identifier) -> Self {
        modules::Builder {
            identifier,
            metadata: Default::default(),
            actual_uri: Default::default(),
            dependencies: Default::default(),
            type_classes: Default::default(),
            type_variations: Default::default(),
            function_impls: Default::default(),
        }
    }
}

impl From<Builder> for extension::simple::module::Definition {
    fn from(builder: Builder) -> Self {
        extension::simple::module::Definition {
            identifier: builder.identifier,
            metadata: builder.metadata,
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

/// Toplevel parse function for a simple extension YAML file.
fn parse_root(
    x: &yaml::Value,
    y: &mut context::Context,
    uri: &str,
    remapped_uri: &str,
) -> Result<extension::simple::module::Definition> {
    // Make an the module builder and configure metadata.
    let mut identifier = y.make_extension_id();
    identifier.uri = uri.to_string();
    let mut builder = modules::Builder::from(identifier);
    builder.metadata.description = yaml_field!(x, y, "description", yaml_prim!(str))?
        .1
        .unwrap_or_default();
    builder.actual_uri = remapped_uri.to_string();

    // FIXME: dependencies?
    yaml_repeated_field!(
        x,
        y,
        "types",
        type_classes::parse_type_class,
        0,
        &mut builder
    )?;
    yaml_repeated_field!(
        x,
        y,
        "type_variations",
        type_variations::parse_type_variation,
        0,
        &mut builder
    )?;
    yaml_repeated_field!(
        x,
        y,
        "scalar_functions",
        functions::parse_scalar_function,
        0,
        &mut builder
    )?;
    // FIXME: window functions?
    yaml_repeated_field!(
        x,
        y,
        "aggregate_functions",
        functions::parse_aggregate_function,
        0,
        &mut builder
    )?;
    Ok(builder.into())
}

fn make_module_reference(
    uri: &str,
    definition: Option<Arc<extension::simple::module::Definition>>,
    parse_context: &mut context::Context,
) -> extension::simple::module::Reference {
    Arc::new(extension::reference::Data {
        name: Default::default(),
        uri: extension::reference::Identifier::new(Some(uri), Some(parse_context.path_buf())),
        definition,
    })
}

/// Parse a YAML extension URI string.
pub fn parse_uri<S: AsRef<str>>(
    x: &S,
    y: &mut context::Context,
) -> Result<extension::simple::module::Reference> {
    // Check URI syntax.
    let uri = x.as_ref();
    if let Err(e) = util::string::check_uri(uri) {
        diagnostic!(y, Error, e);
    }

    // See if we've parsed this URI before. If we have, don't parse it again;
    // just link to the previous node.
    let module = if let Some(module) = y.extension_modules().get(uri).cloned() {
        if let Some(path) = module.uri.anchor_path() {
            link!(
                y,
                path.clone(),
                "Module was previously used here; not parsing again"
            );
        }
        make_module_reference(uri, module.definition.clone(), y)
    } else {
        // Load the schema for YAML extension files when this function is first
        // called.
        static SCHEMA: once_cell::sync::Lazy<jsonschema::JSONSchema> =
            once_cell::sync::Lazy::new(|| {
                jsonschema::JSONSchema::compile(
                    &yaml::yaml_to_json(
                        serde_yaml::from_str::<serde_yaml::Value>(include_str!(
                            "../../../resources/text/simple_extensions_schema.yaml"
                        ))
                        .unwrap(),
                        &path::Path::default(),
                    )
                    .unwrap(),
                )
                .unwrap()
            });

        // Parse the file.
        let definition = traversal::parse_uri(
            uri,
            y,
            |x, y| traversal::read_yaml(x, y, Some(&SCHEMA)),
            parse_root,
        )
        .1
        .map(Arc::new);

        // Create reference and insert into extension module list.
        let module = make_module_reference(uri, definition, y);
        y.extension_modules()
            .insert(uri.to_string(), module.clone());
        module
    };

    Ok(module)
}
