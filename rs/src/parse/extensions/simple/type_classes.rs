// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for parsing YAML type class
//! declarations.

use itertools::Itertools;

use crate::input::yaml;
use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::output::type_system::meta::Pattern;
use crate::parse::context;
use crate::parse::extensions::simple::common;
use crate::parse::extensions::simple::derivations;
use crate::parse::extensions::simple::modules;
use std::collections::HashSet;
use std::sync::Arc;

/// Builder for type classes.
pub struct Builder<'a> {
    /// The definition we're constructing.
    pub definition: extension::simple::type_class::Definition,

    /// Context for analyzing type patterns and derivations.
    pub analysis_context: derivations::AnalysisContext<'a>,
}

/// Tries to check that the given pattern is evaluable.
fn check_pattern_is_evaluable(x: &meta::pattern::Value, y: &mut context::Context) {
    if !x.can_evaluate() {
        diagnostic!(
            y,
            Error,
            TypeDerivationInvalid,
            "pattern cannot be evaluated"
        );
    }
}

/// Tries to check that the given pattern might match a data type.
fn check_pattern_is_data_type(x: &meta::pattern::Value, y: &mut context::Context) {
    if !x.is_data_type() {
        diagnostic!(y, Error, TypeDerivationInvalid, "expected data type");
    }
}

/// Parse a metatype.
fn parse_metatype(x: &str, _y: &mut context::Context) -> Result<meta::Type> {
    match x {
        "dataType" => Ok(meta::Type::DataType),
        "boolean" => Ok(meta::Type::Boolean),
        "integer" => Ok(meta::Type::Integer),
        "enumeration" => Ok(meta::Type::Enum),
        "string" => Ok(meta::Type::String),
        _ => Err(cause!(IllegalValue, "unknown type parameter type {x}")),
    }
}

// Parse the minimum/maximum value constraint.
fn parse_min_max(x: &i64, _y: &mut context::Context, metatype: meta::Type) -> Result<i64> {
    if !matches!(metatype, meta::Type::Integer | meta::Type::Unresolved) {
        Err(cause!(
            IllegalValue,
            "min/max is only applicable for integer metatypes"
        ))
    } else {
        Ok(*x)
    }
}

/// Parse a parameter slot definition.
fn parse_parameter(
    x: &yaml::Value,
    y: &mut context::Context,
    z: &mut Builder,
) -> Result<extension::simple::type_class::ParameterSlot> {
    // Parse name. Names are optional, but if specified, an inconsistent
    // binding is inferred such that the parameter can be used in the
    // structure declaration.
    let name = yaml_field!(x, y, "name", yaml_prim!(str))?
        .1
        .unwrap_or_default();

    // Description is also optional, and only used for docs.
    let description = yaml_field!(x, y, "description", yaml_prim!(str))?
        .1
        .unwrap_or_default();

    // Parse parameter type. This is the only thing that's actually required.
    let metatype = yaml_required_field!(x, y, "type", yaml_prim!(str, parse_metatype))?
        .1
        .unwrap_or_default();

    // Parse and check integer constraint fields.
    let int_min = yaml_field!(x, y, "min", yaml_prim!(i64, parse_min_max, metatype))?
        .1
        .unwrap_or(i64::MIN);
    let int_max = yaml_field!(x, y, "max", yaml_prim!(i64, parse_min_max, metatype))?
        .1
        .unwrap_or(i64::MAX);
    let int_max = if int_min > int_max {
        diagnostic!(
            y,
            Error,
            IllegalValue,
            "minimum value cannot be less than maximum value"
        );
        int_min
    } else {
        if int_min == int_max {
            diagnostic!(
                y,
                Info,
                Redundant,
                "parameter slot only matches a single value ({int_min})"
            );
        }
        int_max
    };

    // Parse and check enum constraint fields.
    let enum_options = yaml_repeated_field!(x, y, "options", yaml_prim!(str))?
        .1
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    if !enum_options.is_empty() {
        if !matches!(metatype, meta::Type::Enum | meta::Type::Unresolved)
            && !enum_options.is_empty()
        {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "enum options are only applicable for enum metatypes"
            )
        }
        let mut unique_names = HashSet::new();
        let mut repeated_names = HashSet::new();
        for name in enum_options.iter() {
            if !unique_names.insert(name.to_ascii_uppercase()) {
                repeated_names.insert(name.to_ascii_uppercase());
            }
        }
        if !repeated_names.is_empty() {
            diagnostic!(
                y,
                Error,
                RedundantEnumVariant,
                "enumeration variant names should be case-insensitively unique: {}",
                repeated_names.iter().join(", ")
            );
        }
        if unique_names.len() == 1 {
            diagnostic!(
                y,
                Info,
                Redundant,
                "parameter slot only matches a single value ({})",
                unique_names.into_iter().next().unwrap()
            );
        }
        z.analysis_context.register_enum_variants(&enum_options);
    }

    // Construct constraint pattern from the above information.
    let pattern = match metatype {
        meta::Type::Integer => meta::pattern::Value::Integer(int_min, int_max),
        meta::Type::Enum => {
            if enum_options.is_empty() {
                meta::pattern::Value::Enum(None)
            } else {
                meta::pattern::Value::Enum(Some(enum_options))
            }
        }
        _ => meta::pattern::Value::exactly_type(metatype),
    };

    // If the parameter has a name, also match it as an inconsistent binding.
    // Annoying special case here... because of the way nullability works for
    // patterns, we need a different pattern depending on whether we're
    // matching a data type or something else. Note that we use inconsistent
    // bindings such that the binding never imposes an unintentional
    // constraint, for example when someone names two parameters the same way.
    let pattern = if name.is_empty() {
        pattern
    } else {
        meta::pattern::Value::Intersection(vec![
            meta::pattern::Value::Binding(meta::pattern::Binding {
                name: name.clone(),
                inconsistent: true,
                nullability: if metatype == meta::Type::DataType {
                    Some(Arc::new(meta::pattern::Value::Boolean(None)))
                } else {
                    None
                },
            }),
            pattern,
        ])
    };

    // Determine whether the parameter is optional.
    let optional = yaml_field!(x, y, "optional", yaml_prim!(bool))?
        .1
        .unwrap_or_default();

    Ok(extension::simple::type_class::ParameterSlot {
        name,
        description,
        pattern,
        optional,
    })
}

/// Parse the structure field of a type class.
fn parse_structure_element(
    k: &str,
    x: &yaml::Value,
    y: &mut context::Context,
    z: &mut Builder,
) -> Result<meta::pattern::Parameter> {
    let pattern = if let serde_json::Value::String(x) = x {
        match derivations::parse_pattern(x, y, &mut z.analysis_context) {
            Ok(pattern) => pattern,
            Err(e) => {
                diagnostic!(y, Error, e);
                meta::pattern::Value::Unresolved
            }
        }
    } else {
        diagnostic!(y, Error, YamlInvalidType, "expected string");
        meta::pattern::Value::Unresolved
    };
    check_pattern_is_evaluable(&pattern, y);
    check_pattern_is_data_type(&pattern, y);
    Ok(meta::pattern::Parameter {
        name: Some(k.to_string()),
        value: Some(pattern),
    })
}

/// Parse the structure field of a type class.
fn parse_structure(x: &yaml::Value, y: &mut context::Context, z: &mut Builder) -> Result<()> {
    match &x {
        serde_json::Value::String(x) => {
            let program = match derivations::parse_program(x, y, &mut z.analysis_context) {
                Ok(program) => program,
                Err(e) => {
                    diagnostic!(y, Error, e);
                    meta::Program::default()
                }
            };
            check_pattern_is_evaluable(&program.expression, y);
            check_pattern_is_data_type(&program.expression, y);
            z.definition.structure = Some(program.expression);
            z.definition
                .contraints
                .extend(program.statements.into_iter());
        }
        serde_json::Value::Object(_) => {
            let fields = yaml_object!(x, y, parse_structure_element, 0, z)?
                .1
                .into_iter()
                .map(|x| x.unwrap_or_default())
                .collect();
            z.definition.structure =
                Some(meta::pattern::Value::DataType(meta::pattern::DataType {
                    class: Some(data::class::Class::Compound(
                        data::class::Compound::NamedStruct,
                    )),
                    nullable: Arc::new(meta::pattern::Value::Boolean(Some(false))),
                    variation: meta::pattern::Variation::Compatible,
                    parameters: Some(fields),
                }));
        }
        _ => diagnostic!(y, Error, YamlInvalidType, "expected string or object"),
    }
    Ok(())
}

/// Parse a type class declaration.
pub fn parse_type_class(
    x: &yaml::Value,
    y: &mut context::Context,
    z: &mut modules::Builder,
) -> Result<()> {
    let mut builder = Builder {
        definition: Default::default(),
        analysis_context: derivations::AnalysisContext::new(Some(z)),
    };

    // Parse name.
    let name = yaml_required_field!(
        x,
        y,
        "name",
        yaml_prim!(str, common::parse_name, "type class")
    )?
    .1;

    // Check uniqueness of name.
    if let Some(name) = &name {
        z.type_classes
            .resolve_local(&name[..])
            .expect_not_yet_defined(y);
    }

    // Parse parameters.
    builder.definition.parameter_slots =
        yaml_repeated_field!(x, y, "parameters", parse_parameter, 0, &mut builder)?
            .1
            .into_iter()
            .map(|x| x.unwrap_or_default())
            .collect();

    // Parse variadicity of parameters.
    if let Some(variadic) = yaml_field!(x, y, "variadic", yaml_prim!(bool))?.1 {
        if builder.definition.parameter_slots.is_empty() {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "variadic must be left unspecified for type classes \
                without parameters"
            );
        } else {
            builder.definition.parameters_variadic = variadic;
        }
    }

    // Parse structure.
    yaml_field!(x, y, "structure", parse_structure, &mut builder)?;

    // Register the type class.
    if let Some(name) = name {
        z.type_classes
            .define_item(name, Arc::new(builder.definition), true);
    }

    // Describe the type class.
    // TODO

    Ok(())
}
