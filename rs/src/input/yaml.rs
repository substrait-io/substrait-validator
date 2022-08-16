// SPDX-License-Identifier: Apache-2.0

//! Module for representing YAML input.
//!
//! We (ab)use [`serde_json::value::Value`] for this; the primary reason being
//! that a [JSON schema](https://json-schema.org/) is used for basic schema
//! validation of the YAML files, and the [`jsonschema`] crate we use for that
//! uses [`serde_json`]'s representation). [`yaml_to_json()`] may be used to
//! convert the output from [`serde_yaml`] to this structure.

use crate::output::diagnostic;
use crate::output::path;
use crate::output::primitive_data;
use crate::output::tree;
use crate::parse::traversal;

use serde_yaml::value::Value as Yaml;

/// Type for the type used for arbitrary YAML values.
pub type Value = serde_json::value::Value;

/// Typedef for the type used for YAML arrays.
pub type Array = Vec<Value>;

/// Typedef for the type used for YAML maps.
pub type Map = serde_json::map::Map<String, Value>;

/// Converts a [`serde_yaml`] YAML structure into its equivalent JSON object
/// model using [`serde_json`]'s types.
pub fn yaml_to_json(y: Yaml, path: &path::Path) -> diagnostic::DiagResult<Value> {
    match y {
        Yaml::Null => Ok(Value::Null),
        Yaml::Bool(b) => Ok(Value::Bool(b)),
        Yaml::Number(n) => {
            if let Some(u) = n.as_u64() {
                Ok(Value::Number(u.into()))
            } else if let Some(i) = n.as_i64() {
                Ok(Value::Number(i.into()))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Number(
                    serde_json::value::Number::from_f64(f).ok_or_else(|| {
                        diag!(
                            path.to_path_buf(),
                            Error,
                            YamlParseFailed,
                            "{f} float is not supported"
                        )
                    })?,
                ))
            } else {
                panic!("encountered serde_yaml::number::Number that cannot be represented as u64, i64, of f64");
            }
        }
        Yaml::String(s) => Ok(Value::String(s)),
        Yaml::Sequence(a) => Ok(Value::Array(
            a.into_iter()
                .enumerate()
                .map(|(index, value)| yaml_to_json(value, &path.with_index(index)))
                .collect::<diagnostic::DiagResult<Vec<Value>>>()?,
        )),
        Yaml::Mapping(m) => Ok(Value::Object(
            m.into_iter()
                .map(|(key, value)| {
                    let key = key
                        .as_str()
                        .ok_or_else(|| {
                            diag!(
                                path.to_path_buf(),
                                Error,
                                YamlParseFailed,
                                "non-string map keys are not supported"
                            )
                        })?
                        .to_string();
                    let path = path.with_field(&key);
                    let value = yaml_to_json(value, &path)?;
                    Ok((key, value))
                })
                .collect::<diagnostic::DiagResult<serde_json::value::Map<String, Value>>>()?,
        )),
        Yaml::Tagged(_) => Err(diag!(
            path.to_path_buf(),
            Error,
            YamlParseFailed,
            "YAML tagged values are not supported"
        )),
    }
}

impl crate::input::traits::InputNode for Value {
    fn type_to_node() -> tree::Node {
        tree::NodeType::YamlMap.into()
    }

    fn data_to_node(&self) -> tree::Node {
        match self {
            Value::Null => tree::NodeType::YamlPrimitive(primitive_data::PrimitiveData::Null),
            Value::Bool(b) => {
                tree::NodeType::YamlPrimitive(primitive_data::PrimitiveData::Bool(*b))
            }
            Value::Number(n) => tree::NodeType::YamlPrimitive(
                n.as_u64()
                    .map(primitive_data::PrimitiveData::Unsigned)
                    .or_else(|| n.as_i64().map(primitive_data::PrimitiveData::Signed))
                    .or_else(|| n.as_f64().map(primitive_data::PrimitiveData::Float))
                    .unwrap(),
            ),
            Value::String(s) => {
                tree::NodeType::YamlPrimitive(primitive_data::PrimitiveData::String(s.clone()))
            }
            Value::Array(_) => tree::NodeType::YamlArray,
            Value::Object(_) => tree::NodeType::YamlMap,
        }
        .into()
    }

    fn oneof_variant(&self) -> Option<&'static str> {
        None
    }

    fn parse_unknown(&self, context: &mut crate::parse::context::Context<'_>) -> bool {
        match self {
            Value::Array(array) => {
                let mut any = false;
                for (index, _) in array.iter().enumerate() {
                    if !context.field_parsed(index.to_string()) {
                        traversal::push_yaml_element(array, context, index, true, |_, _| Ok(()));
                        any = true;
                    }
                }
                any
            }
            Value::Object(object) => {
                let mut any = false;
                let mut keys: Vec<_> = object.keys().collect();
                keys.sort();
                for field_name in keys {
                    if !context.field_parsed(field_name) {
                        traversal::push_yaml_field(self, context, field_name, true, |_, _| Ok(()))
                            .unwrap();
                        any = true;
                    }
                }
                any
            }
            _ => false,
        }
    }
}
