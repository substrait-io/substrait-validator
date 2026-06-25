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

/// Parses and validates the top-level `urn` field of an extension file,
/// checking that it is a syntactically valid extension URN and that it matches
/// the URN that was used to reference the file.
fn parse_urn_field(x: &str, y: &mut context::Context, expected_urn: &str) -> Result<String> {
    if let Err(e) = util::string::check_urn(x) {
        diagnostic!(y, Error, e);
    } else if x != expected_urn {
        diagnostic!(
            y,
            Error,
            IllegalUrn,
            "the urn declared in this file ({x:?}) does not match the urn used \
            to reference it ({expected_urn:?})"
        );
    }
    Ok(x.to_owned())
}

/// Parses a single `dependencies` entry: the value must be an extension URN,
/// which is resolved (recursively) so that the dependency is validated and
/// transitive resolution/cycle detection applies.
fn parse_dependency(
    x: &str,
    y: &mut context::Context,
) -> Result<extension::simple::module::Reference> {
    parse_urn(&x, y)
}

/// Parses the `dependencies` field: a map from namespace alias to extension
/// URN. Each referenced extension is resolved and stored on the builder.
fn parse_dependencies(
    x: &yaml::Value,
    y: &mut context::Context,
    builder: &mut builder::Builder,
) -> Result<()> {
    if let yaml::Value::Object(object) = x {
        let mut aliases: Vec<&String> = object.keys().collect();
        aliases.sort();
        for alias in aliases {
            let (_, reference) =
                yaml_field!(x, y, alias.as_str(), yaml_prim!(str, parse_dependency))?;
            if let Some(reference) = reference {
                builder.dependencies.insert(alias.clone(), reference);
            }
        }
        Ok(())
    } else {
        Err(cause!(YamlInvalidType, "object expected"))
    }
}

/// Toplevel parse function for a simple extension YAML file. `expected_urn` is
/// the extension URN that was used to reference this file; the file's own `urn`
/// field is validated against it.
fn parse_root(
    x: &yaml::Value,
    y: &mut context::Context,
    expected_urn: &str,
) -> Result<extension::simple::module::Definition> {
    let mut builder = builder::Builder::default();
    if let (_, Some(urn)) = yaml_required_field!(
        x,
        y,
        "urn",
        yaml_prim!(str, |s, yy| parse_urn_field(s, yy, expected_urn))
    )? {
        builder.actual_urn = urn;
    }
    yaml_field!(x, y, "dependencies", parse_dependencies, &mut builder)?;
    // `metadata` holds arbitrary extension-author data; accept it without
    // inspecting its contents.
    crate::parse::traversal::push_yaml_field(x, y, "metadata", true, |_, _| Ok(()))?;
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
    urn: &str,
    definition: Option<Arc<extension::simple::module::Definition>>,
    parse_context: &mut context::Context,
) -> extension::simple::module::Reference {
    Arc::new(extension::reference::Data {
        name: Default::default(),
        urn: extension::reference::Identifier::new(Some(urn), Some(parse_context.path_buf())),
        definition,
    })
}

/// Parse a YAML extension URN string and resolve the file it identifies.
pub fn parse_urn<S: AsRef<str>>(
    x: &S,
    y: &mut context::Context,
) -> Result<extension::simple::module::Reference> {
    // Check URN syntax.
    let urn = x.as_ref();
    if let Err(e) = util::string::check_urn(urn) {
        diagnostic!(y, Error, e);
    }

    // See if we've parsed this URN before. If we have, don't parse it again;
    // just link to the previous node.
    let module = if let Some(module) = y.extension_modules().get(urn).cloned() {
        if let Some(path) = module.urn.anchor_path() {
            link!(
                y,
                path.clone(),
                "Module was previously used here; not parsing again"
            );
        }
        make_module_reference(urn, module.definition.clone(), y)
    } else {
        // Load the schema for YAML extension files when this function is first
        // called.
        static SCHEMA: once_cell::sync::Lazy<jsonschema::Validator> =
            once_cell::sync::Lazy::new(|| {
                jsonschema::Validator::new(
                    &yaml::yaml_to_json(
                        serde_yaml::from_str::<serde_yaml::Value>(
                            substrait_extensions::text::SIMPLE_EXTENSIONS_SCHEMA,
                        )
                        .unwrap(),
                        &path::Path::default(),
                    )
                    .unwrap(),
                )
                .unwrap()
            });

        // Parse the file.
        let definition = traversal::parse_urn(
            urn,
            y,
            |x, y| traversal::read_yaml(x, y, Some(&SCHEMA)),
            |x, y| parse_root(x, y, urn),
        )
        .1
        .map(Arc::new);

        // Create reference and insert into extension module list.
        let module = make_module_reference(urn, definition, y);
        y.extension_modules()
            .insert(urn.to_string(), module.clone());
        module
    };

    Ok(module)
}
