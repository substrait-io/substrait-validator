// SPDX-License-Identifier: Apache-2.0

//! Module for representing Substrait protobuf input.
//!
//! The structures here are generated using [`prost`], but have a bunch of
//! extra traits from [`traits`] associated with them,
//! for which the implementations are generated using
//! [`substrait_validator_derive`]. The purpose of these traits is to add basic
//! introspection capabilities to the prost structures. One of the use cases
//! for this is to let the parsing code automatically detect when the
//! validation code ignored a subtree while validating, which implies that the
//! validator hasn't checked everything and thus should not warrant that the
//! received plan is valid.

use crate::input::traits;
use crate::output::primitive_data;
use crate::output::tree;

use heck::ToUpperCamelCase;

pub static DESCRIPTOR_POOL: once_cell::sync::Lazy<prost_reflect::DescriptorPool> =
    once_cell::sync::Lazy::new(|| {
        prost_reflect::DescriptorPool::decode(
            include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
        )
        .expect("failed to decode descriptor pool")
    });

/// Creates a tree node representing an unknown field of the given descriptor.
/// Used by parse_unknown implementations to push unrecognized fields into the
/// output tree without access to the typed Rust field value.
pub fn field_descriptor_to_node(field: &prost_reflect::FieldDescriptor) -> tree::Node {
    use prost_reflect::Kind;
    match field.kind() {
        Kind::Message(desc) => {
            let name: &'static str =
                Box::leak(desc.full_name().to_string().into_boxed_str());
            tree::NodeType::ProtoMessage(name).into()
        }
        Kind::Enum(desc) => {
            let type_name: &'static str =
                Box::leak(desc.full_name().to_string().into_boxed_str());
            let default_variant: &'static str =
                Box::leak(desc.default_value().name().to_string().into_boxed_str());
            tree::NodeType::ProtoPrimitive(
                type_name,
                primitive_data::PrimitiveData::Enum(default_variant),
            )
            .into()
        }
        Kind::Bool => tree::NodeType::ProtoPrimitive(
            "bool",
            primitive_data::PrimitiveData::Bool(false),
        )
        .into(),
        Kind::Int32 | Kind::Sint32 | Kind::Sfixed32 => tree::NodeType::ProtoPrimitive(
            match field.kind() {
                Kind::Sint32 => "sint32",
                Kind::Sfixed32 => "sfixed32",
                _ => "int32",
            },
            primitive_data::PrimitiveData::Signed(0),
        )
        .into(),
        Kind::Int64 | Kind::Sint64 | Kind::Sfixed64 => tree::NodeType::ProtoPrimitive(
            match field.kind() {
                Kind::Sint64 => "sint64",
                Kind::Sfixed64 => "sfixed64",
                _ => "int64",
            },
            primitive_data::PrimitiveData::Signed(0),
        )
        .into(),
        Kind::Uint32 | Kind::Fixed32 => tree::NodeType::ProtoPrimitive(
            if matches!(field.kind(), Kind::Fixed32) {
                "fixed32"
            } else {
                "uint32"
            },
            primitive_data::PrimitiveData::Unsigned(0),
        )
        .into(),
        Kind::Uint64 | Kind::Fixed64 => tree::NodeType::ProtoPrimitive(
            if matches!(field.kind(), Kind::Fixed64) {
                "fixed64"
            } else {
                "uint64"
            },
            primitive_data::PrimitiveData::Unsigned(0),
        )
        .into(),
        Kind::Float => tree::NodeType::ProtoPrimitive(
            "float",
            primitive_data::PrimitiveData::Float(0.0),
        )
        .into(),
        Kind::Double => tree::NodeType::ProtoPrimitive(
            "double",
            primitive_data::PrimitiveData::Float(0.0),
        )
        .into(),
        Kind::String => tree::NodeType::ProtoPrimitive(
            "string",
            primitive_data::PrimitiveData::String(String::new()),
        )
        .into(),
        Kind::Bytes => tree::NodeType::ProtoPrimitive(
            "bytes",
            primitive_data::PrimitiveData::Bytes(vec![]),
        )
        .into(),
    }
}

#[allow(
    clippy::large_enum_variant,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_lazy_continuation
)] // caused by generated code
pub mod substrait {
    include!(concat!(env!("OUT_DIR"), "/substrait.rs"));
    pub mod extensions {
        include!(concat!(env!("OUT_DIR"), "/substrait.extensions.rs"));
    }
    pub mod validator {
        include!(concat!(env!("OUT_DIR"), "/substrait.validator.rs"));
    }
}

/// Converts a Rust module path and name (the latter already processed by
/// cook_ident()) to a protobuf type path.
pub fn cook_path(module_path: &str, type_name: &str) -> String {
    let mut iter = module_path
        .split("::")
        .skip(module_path!().split("::").count())
        .map(cook_ident)
        .chain(::std::iter::once(type_name))
        .peekable();
    let mut items = vec![];
    if matches!(iter.peek(), Some(&"substrait")) {
        items.push(iter.next().unwrap().to_string());
        if matches!(iter.peek(), Some(&"extensions") | Some(&"validator")) {
            items.push(iter.next().unwrap().to_string());
        }
    }
    items.extend(iter.map(|x| x.to_upper_camel_case()));
    ::itertools::Itertools::intersperse(items.iter().map(|x| x.as_ref()), ".").collect()
}

/// Converts a Rust identifier string generated via stringify!() to the
/// original identifier by "cooking" raw identifiers.
pub fn cook_ident(ident: &str) -> &str {
    if let Some((_, keyword)) = ident.split_once('#') {
        keyword
    } else {
        ident
    }
}

impl traits::ProtoPrimitive for bool {
    fn proto_primitive_type() -> &'static str {
        "bool"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Bool(false)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Bool(*self)
    }

    fn proto_primitive_is_default(&self) -> bool {
        !*self
    }
}

impl traits::ProtoPrimitive for u32 {
    fn proto_primitive_type() -> &'static str {
        "uint32"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Unsigned(0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Unsigned((*self).into())
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0
    }
}

impl traits::ProtoPrimitive for u64 {
    fn proto_primitive_type() -> &'static str {
        "uint64"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Unsigned(0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Unsigned(*self)
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0
    }
}

impl traits::ProtoPrimitive for i32 {
    fn proto_primitive_type() -> &'static str {
        "int32"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Signed(0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Signed((*self).into())
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0
    }
}

impl traits::ProtoPrimitive for i64 {
    fn proto_primitive_type() -> &'static str {
        "int64"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Signed(0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Signed(*self)
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0
    }
}

impl traits::ProtoPrimitive for f32 {
    fn proto_primitive_type() -> &'static str {
        "float"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Float(0.0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Float((*self).into())
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0.0
    }
}

impl traits::ProtoPrimitive for f64 {
    fn proto_primitive_type() -> &'static str {
        "double"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Float(0.0)
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Float(*self)
    }

    fn proto_primitive_is_default(&self) -> bool {
        *self == 0.0
    }
}

impl traits::ProtoPrimitive for String {
    fn proto_primitive_type() -> &'static str {
        "string"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::String(String::new())
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::String(self.clone())
    }

    fn proto_primitive_is_default(&self) -> bool {
        self.is_empty()
    }
}

impl traits::ProtoPrimitive for Vec<u8> {
    fn proto_primitive_type() -> &'static str {
        "bytes"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Bytes(vec![])
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Bytes(self.clone())
    }

    fn proto_primitive_is_default(&self) -> bool {
        self.is_empty()
    }
}

impl traits::ProtoPrimitive for prost_types::Any {
    fn proto_primitive_type() -> &'static str {
        "any"
    }

    fn proto_primitive_default() -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Any(prost_types::Any::default())
    }

    fn proto_primitive_data(&self) -> primitive_data::PrimitiveData {
        primitive_data::PrimitiveData::Any(self.clone())
    }

    fn proto_primitive_is_default(&self) -> bool {
        self.type_url.is_empty()
    }
}
