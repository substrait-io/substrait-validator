// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for advanced extensions, i.e.
//! those based around YAML files.

use crate::input::proto::substrait;
use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::extension::simple::module::Scope;
use crate::output::type_system::data;
use crate::parse::context;

mod builder;
mod derivations;
mod function_decls;
mod type_decls;
mod type_variation_decls;
mod yaml;

/// Parse a user-defined name. Note that names are matched case-insensitively
/// because we return the name as lowercase.
#[allow(clippy::ptr_arg)]
pub fn parse_name(x: &String, _y: &mut context::Context) -> Result<String> {
    // FIXME: nothing seems to say anything about the validity of names for
    // things, but this seems rather important to define.
    if x.is_empty() {
        Err(cause!(IllegalValue, "names cannot be empty"))
    } else {
        Ok(x.to_lowercase())
    }
}

/// "Parse" an anchor. This just reports a warning if the anchor is 0.
fn parse_anchor(x: &u32, y: &mut context::Context) -> Result<u32> {
    if *x == 0 {
        diagnostic!(
            y,
            Warning,
            LinkAnchorZero,
            "use of anchor zero is discouraged, as references set to \
            zero may be confused with \"unspecified\"."
        );
    }
    Ok(*x)
}

/// Parse a mapping from a URI anchor to a YAML extension.
fn parse_simple_extension_yaml_uri_mapping(
    x: &substrait::extensions::SimpleExtensionUri,
    y: &mut context::Context,
) -> Result<()> {
    // Parse the fields.
    let anchor = proto_primitive_field!(x, y, extension_uri_anchor, parse_anchor).1;
    let yaml_data = proto_primitive_field!(x, y, uri, yaml::parse_uri)
        .1
        .unwrap();

    // If the specified anchor is valid, insert a mapping for it.
    if let Some(anchor) = anchor {
        if let Err((prev_data, prev_path)) = y.define_extension_uri(anchor, yaml_data) {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "anchor {anchor} is already in use for URI {}",
                prev_data.uri
            );
            link!(y, prev_path, "Previous definition was here.");
        }
    }

    Ok(())
}

/// Parse an URI reference and resolve it.
fn parse_uri_reference(
    x: &u32,
    y: &mut context::Context,
) -> Result<extension::simple::module::Reference> {
    match y.extension_uris().resolve(x).cloned() {
        Some((yaml_data, path)) => {
            describe!(y, Misc, "{}", yaml_data.uri);
            link!(y, path, "URI anchor is defined here");
            Ok(yaml_data)
        }
        None => {
            describe!(y, Misc, "Unresolved URI");
            Err(cause!(LinkMissingAnchor, "URI anchor {x} does not exist"))
        }
    }
}

/// Parse a type variation reference and resolve it, along with a given class
/// (the anchor is otherwise not unique!).
pub fn parse_type_variation_reference_with_class(
    x: &u32,
    y: &mut context::Context,
    class: &data::Class,
) -> Result<data::Variation> {
    match y.type_variations().resolve(x).cloned() {
        Some((variations, _)) => {
            let variation = resolve_variation_by_class(y, Some(variations), class);
            describe!(y, Misc, "{}", variation);
            Ok(variation)
        }
        None => {
            if x == &0 {
                describe!(y, Misc, "System-preferred variation");
                Ok(data::Variation::SystemPreferred)
            } else {
                describe!(y, Misc, "Unresolved type variation");
                Err(cause!(
                    LinkMissingAnchor,
                    "type variation anchor {x} does not exist"
                ))
            }
        }
    }
}

/// Parse a type variation reference and resolve it to the set of variations
/// defined with that name in a given simple extension. Note that there can be
/// more than one of those, because names are scoped to a type class. None
/// is returned to reference the system-preferred variation. Resolution can
/// be completed once the class is known via resolve_variation_by_class().
pub fn parse_type_variation_reference_without_class(
    x: &u32,
    y: &mut context::Context,
) -> Result<Option<extension::simple::type_variation::ResolutionResult>> {
    match y.type_variations().resolve(x).cloned() {
        Some((variations, _)) => {
            describe!(y, Misc, "{}", &variations);
            Ok(Some(variations))
        }
        None => {
            if x == &0 {
                describe!(y, Misc, "System-preferred variation");
                Ok(None)
            } else {
                describe!(y, Misc, "Unresolved type variation");
                Err(cause!(
                    LinkMissingAnchor,
                    "type variation anchor {x} does not exist"
                ))
            }
        }
    }
}

/// Resolves a partial variation lookup previously performed by
/// parse_type_variation_reference_without_class() once the class is known.
pub fn resolve_variation_by_class(
    y: &mut context::Context,
    variations: Option<extension::simple::type_variation::ResolutionResult>,
    class: &data::Class,
) -> data::Variation {
    if let Some(variations) = variations {
        data::Variation::UserDefined(
            variations
                .filter_items(|x| &x.base == class)
                .expect_one(
                    y,
                    |x, y| {
                        diagnostic!(
                            y,
                            Error,
                            LinkMissingTypeVariationNameAndClass,
                            "{x} exists, but is not a variation of {class} data types"
                        );
                        true
                    },
                    |_, _| false,
                )
                .as_item(),
        )
    } else {
        data::Variation::SystemPreferred
    }
}

/// Parse a type reference and resolve it.
pub fn parse_type_reference(
    x: &u32,
    y: &mut context::Context,
) -> Result<extension::simple::type_class::Reference> {
    match y.type_classes().resolve(x).cloned() {
        Some((type_class, path)) => {
            describe!(y, Misc, "{}", &type_class);
            link!(y, path, "Type anchor is defined here");
            Ok(type_class.as_item())
        }
        None => {
            describe!(y, Misc, "Unresolved type");
            Err(cause!(LinkMissingAnchor, "type anchor {x} does not exist"))
        }
    }
}

/// Parse a function reference and resolve it.
pub fn parse_function_reference(
    x: &u32,
    y: &mut context::Context,
) -> Result<extension::simple::function::ResolutionResult> {
    match y.functions().resolve(x).cloned() {
        Some((function, path)) => {
            describe!(y, Misc, "{}", &function);
            link!(y, path, "Function anchor is defined here");
            Ok(function)
        }
        None => {
            describe!(y, Misc, "Unresolved function");
            Err(cause!(
                LinkMissingAnchor,
                "function anchor {x} does not exist"
            ))
        }
    }
}

/// Parse a mapping from a function/type/variation anchor to an extension.
fn parse_extension_mapping_data(
    x: &substrait::extensions::simple_extension_declaration::MappingType,
    y: &mut context::Context,
) -> Result<()> {
    match x {
        substrait::extensions::simple_extension_declaration::MappingType::ExtensionType(x) => {

            // Parse the fields.
            let module_ref_opt = proto_primitive_field!(x, y, extension_uri_reference, parse_uri_reference).1;
            let anchor = proto_primitive_field!(x, y, type_anchor, parse_anchor).1;
            let name = proto_primitive_field!(x, y, name, parse_name).1;

            // Construct an unresolved reference for this data type.
            let reference_data = extension::reference::Data {
                name: extension::reference::Identifier::new(name.as_ref(), Some(y.path_buf())),
                uri: module_ref_opt.as_ref().map(|x| x.uri.clone()).unwrap_or_default(),
                definition: None
            };

            // If we successfully resolved the URI reference to a URI, resolved
            // that URI, and managed to parse the YAML it pointed to, try to
            // resolve the data type in it.
            let resolution_result = module_ref_opt.as_ref().and_then(|module_ref| {
                module_ref.definition.as_ref().and_then(|module| {
                    name.as_ref().map(|_| {
                        module
                            .resolve_type_class(reference_data.clone())
                            .filter_all_items()
                            .expect_one(y, |_, _| false, |_, _| false)
                    })
                })
            }).unwrap_or_else(|| {
                extension::namespace::ResolutionResult::new(reference_data)
            });

            // If the specified anchor is valid, insert a mapping for it.
            if let Some(anchor) = anchor {
                if let Err((prev_data, prev_path)) = y.define_type(anchor, resolution_result) {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "anchor {anchor} is already in use for data type {prev_data}"
                    );
                    link!(y, prev_path, "Previous definition was here.");
                }
            }

        }
        substrait::extensions::simple_extension_declaration::MappingType::ExtensionTypeVariation(x) => {

            // Parse the fields.
            let module_ref_opt = proto_primitive_field!(x, y, extension_uri_reference, parse_uri_reference).1;
            let anchor = proto_primitive_field!(x, y, type_variation_anchor, parse_anchor).1;
            let name = proto_primitive_field!(x, y, name, parse_name).1;

            // Construct an unresolved reference for this data type.
            let reference_data = extension::reference::Data {
                name: extension::reference::Identifier::new(name.as_ref(), Some(y.path_buf())),
                uri: module_ref_opt.as_ref().map(|x| x.uri.clone()).unwrap_or_default(),
                definition: None
            };

            // If we successfully resolved the URI reference to a URI, resolved
            // that URI, and managed to parse the YAML it pointed to, try to
            // resolve the type variations for it. Note that an
            // anchor/reference pair can legally refer to multiple variations
            // at once, as Substrait scopes them to the type class they are
            // defined for.
            let resolution_result = module_ref_opt.as_ref().and_then(|module_ref| {
                module_ref.definition.as_ref().and_then(|module| {
                    name.as_ref().map(|_| {
                        module
                            .resolve_type_variation(reference_data.clone())
                            .filter_all_items()
                            .expect_multiple(y, |_, _| false)
                    })
                })
            }).unwrap_or_else(|| {
                extension::namespace::ResolutionResult::new(reference_data)
            });

            // If the specified anchor is valid, insert a mapping for it.
            if let Some(anchor) = anchor {
                if let Err((prev_data, prev_path)) = y.define_type_variation(anchor, resolution_result) {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "anchor {anchor} is already in use for type variation {prev_data}"
                    );
                    link!(y, prev_path, "Previous definition was here.");
                }
            }

        }
        substrait::extensions::simple_extension_declaration::MappingType::ExtensionFunction(x) => {

            // Parse the fields.
            let module_ref_opt = proto_primitive_field!(x, y, extension_uri_reference, parse_uri_reference).1;
            let anchor = proto_primitive_field!(x, y, function_anchor, parse_anchor).1;
            let name = proto_primitive_field!(x, y, name).1;

            // Construct an unresolved reference for this data type.
            let reference_data = extension::reference::Data {
                name: extension::reference::Identifier::new(name.as_ref(), Some(y.path_buf())),
                uri: module_ref_opt.as_ref().map(|x| x.uri.clone()).unwrap_or_default(),
                definition: None
            };

            // If we successfully resolved the URI reference to a URI, resolved
            // that URI, and managed to parse the YAML it pointed to, try to
            // resolve the type variations for it. Note that an
            // anchor/reference pair can legally refer to multiple variations
            // at once, as Substrait scopes them to the type class they are
            // defined for.
            let resolution_result = module_ref_opt.as_ref().and_then(|module_ref| {
                module_ref.definition.as_ref().and_then(|module| {
                    name.as_ref().map(|_| {
                        module
                            .resolve_function(reference_data.clone())
                            .filter_all_items()
                            .expect_one(
                                y,
                                |_, _| false,
                                |x, y| {
                                    if x.contains(':') {
                                        false
                                    } else {
                                        diagnostic!(
                                            y,
                                            Error,
                                            LinkCompoundVsSimpleFunctionName,
                                            "this function has multiple implementations, but \
                                            is being referred to using its simple name. Substrait \
                                            does not allow disambiguation by means of function \
                                            arguments; you must specify a compound name to \
                                            disambiguate instead."
                                        );
                                        true
                                    }
                                }
                            )
                    })
                })
            }).unwrap_or_else(|| {
                extension::namespace::ResolutionResult::new(reference_data)
            });

            // If the specified anchor is valid, insert a mapping for it.
            if let Some(anchor) = anchor {
                if let Err((prev_data, prev_path)) = y.define_function(anchor, resolution_result) {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "anchor {anchor} is already in use for function {prev_data}"
                    );
                    link!(y, prev_path, "Previous definition was here.");
                }
            }

        }
    };
    Ok(())
}

/// Parse a mapping from a function/type/variation anchor to an extension.
fn parse_extension_mapping(
    x: &substrait::extensions::SimpleExtensionDeclaration,
    y: &mut context::Context,
) -> Result<()> {
    proto_required_field!(x, y, mapping_type, parse_extension_mapping_data);
    Ok(())
}

/// Parses the simple extension information in a plan.
pub fn parse_plan(x: &substrait::Plan, y: &mut context::Context) {
    proto_repeated_field!(
        x,
        y,
        extension_uris,
        parse_simple_extension_yaml_uri_mapping
    );
    proto_repeated_field!(x, y, extensions, parse_extension_mapping);
}

/// Generate Info diagnostics for any extension definitions that weren't used.
pub fn check_unused_definitions(y: &mut context::Context) {
    // List unused function declarations.
    for (anchor, info, path) in y.functions().iter_unused().collect::<Vec<_>>().into_iter() {
        diagnostic!(
            y,
            Info,
            RedundantFunctionDeclaration,
            "anchor {anchor} for function {info} is not present in the plan"
        );
        link!(y, path, "Declaration was here.");
    }

    // List unused type declarations.
    for (anchor, info, path) in y
        .type_classes()
        .iter_unused()
        .collect::<Vec<_>>()
        .into_iter()
    {
        diagnostic!(
            y,
            Info,
            RedundantTypeDeclaration,
            "anchor {anchor} for type class {info} is not present in the plan"
        );
        link!(y, path, "Declaration was here.");
    }

    // List unused type variation declarations.
    for (anchor, info, path) in y
        .type_variations()
        .iter_unused()
        .collect::<Vec<_>>()
        .into_iter()
    {
        diagnostic!(
            y,
            Info,
            RedundantTypeVariationDeclaration,
            "anchor {anchor} for type variation {info} is not present in the plan"
        );
        link!(y, path, "Declaration was here.");
    }
}
