// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for parsing YAML extension
//! files.

use crate::input::yaml;
use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::path;
use crate::parse::context;
use crate::parse::extensions::simple::builder;
use crate::parse::extensions::simple::function_decls;
use crate::parse::extensions::simple::type_decls;
use crate::parse::extensions::simple::type_variation_decls;
use crate::parse::traversal;
use crate::util;
use std::sync::Arc;

/// Toplevel parse function for a simple extension YAML file.
fn parse_root(
    x: &yaml::Value,
    y: &mut context::Context,
) -> Result<extension::simple::module::Definition> {
    let mut builder = builder::Builder::default();
    yaml_repeated_field!(x, y, "types", type_decls::parse_type, 0, &mut builder)?;
    yaml_repeated_field!(
        x,
        y,
        "type_variations",
        type_variation_decls::parse_type_variation,
        0,
        &mut builder
    )?;
    yaml_repeated_field!(
        x,
        y,
        "scalar_functions",
        function_decls::parse_scalar_function,
        0,
        &mut builder
    )?;
    yaml_repeated_field!(
        x,
        y,
        "aggregate_functions",
        function_decls::parse_aggregate_function,
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
