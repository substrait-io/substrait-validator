// SPDX-License-Identifier: Apache-2.0

//! Module for representing Substrait protobuf input.
//!
//! The core `substrait`/`substrait.extensions` types come from the
//! [`substrait_prost`] crate; the validator-specific `substrait.validator`
//! package is generated locally by `build.rs`. Both carry a set of
//! introspection traits from [`traits`] ([`InputNode`], [`ProtoMessage`],
//! [`ProtoOneOf`], [`ProtoEnum`]) that let the parsing code detect when the
//! validation code ignored a subtree, which implies the validator hasn't
//! checked everything and thus should not warrant that the plan is valid.
//!
//! These impls come from two sources, both of which delegate their
//! unknown-field enumeration to runtime reflection rather than reconstructing
//! prost's code generation:
//!  - the foreign substrait-prost types: from `build.rs` (see the `prost_meta`
//!    module), leaning on the `prost::Name` + `prost_reflect::ReflectMessage`
//!    impls that substrait-prost's `reflect` feature provides; and
//!  - the local validator types: from the [`substrait_validator_derive`]
//!    `ProtoMeta` derive, using [`DESCRIPTOR_POOL`].
//!
//! [`InputNode`]: crate::input::traits::InputNode
//! [`ProtoMessage`]: crate::input::traits::ProtoMessage
//! [`ProtoOneOf`]: crate::input::traits::ProtoOneOf
//! [`ProtoEnum`]: crate::input::traits::ProtoEnum

use crate::input::traits;
use crate::output::primitive_data;
use crate::output::tree;

use heck::ToUpperCamelCase;

/// Runtime descriptor pool for the locally-generated `substrait.validator`
/// package, decoded from the `FileDescriptorSet` that `build.rs` embeds. The
/// `ProtoMeta` derive's `ReflectMessage` and `ProtoEnum` impls look up their
/// descriptors here. (The foreign substrait-prost types resolve their
/// descriptors through substrait-prost's own embedded pool instead.)
pub static DESCRIPTOR_POOL: once_cell::sync::Lazy<prost_reflect::DescriptorPool> =
    once_cell::sync::Lazy::new(|| {
        prost_reflect::DescriptorPool::decode(
            include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
        )
        .expect("failed to decode validator descriptor pool")
    });

/// Interns a string as a `&'static str`, leaking each distinct value at most
/// once. The output tree's `ProtoPrimitive` node stores `&'static str` type and
/// enum-variant names, but enum names discovered via reflection are owned
/// `String`s. Interning caps the leaked memory at the finite set of names in
/// the schema instead of leaking on every [`field_descriptor_to_node`] call.
fn intern(s: &str) -> &'static str {
    static POOL: once_cell::sync::Lazy<std::sync::Mutex<std::collections::HashSet<&'static str>>> =
        once_cell::sync::Lazy::new(|| std::sync::Mutex::new(std::collections::HashSet::new()));
    let mut pool = POOL.lock().unwrap();
    if let Some(&existing) = pool.get(s) {
        return existing;
    }
    let leaked: &'static str = Box::leak(s.to_owned().into_boxed_str());
    pool.insert(leaked);
    leaked
}

/// Builds a tree node representing an unrecognized field from its descriptor
/// alone (i.e. without a typed Rust value). Used by
/// [`parse_proto_message_unknown`](crate::parse::traversal::parse_proto_message_unknown)
/// to render fields the validator did not visit.
pub fn field_descriptor_to_node(field: &prost_reflect::FieldDescriptor) -> tree::Node {
    use prost_reflect::Kind;
    let kind = field.kind();
    match &kind {
        Kind::Message(desc) => tree::NodeType::ProtoMessage(desc.full_name().to_string()).into(),
        Kind::Enum(desc) => tree::NodeType::ProtoPrimitive(
            intern(desc.full_name()),
            primitive_data::PrimitiveData::Enum(intern(desc.default_value().name())),
        )
        .into(),
        Kind::Bool => {
            tree::NodeType::ProtoPrimitive("bool", primitive_data::PrimitiveData::Bool(false))
                .into()
        }
        Kind::Int32 | Kind::Sint32 | Kind::Sfixed32 => tree::NodeType::ProtoPrimitive(
            match &kind {
                Kind::Sint32 => "sint32",
                Kind::Sfixed32 => "sfixed32",
                _ => "int32",
            },
            primitive_data::PrimitiveData::Signed(0),
        )
        .into(),
        Kind::Int64 | Kind::Sint64 | Kind::Sfixed64 => tree::NodeType::ProtoPrimitive(
            match &kind {
                Kind::Sint64 => "sint64",
                Kind::Sfixed64 => "sfixed64",
                _ => "int64",
            },
            primitive_data::PrimitiveData::Signed(0),
        )
        .into(),
        Kind::Uint32 | Kind::Fixed32 => tree::NodeType::ProtoPrimitive(
            if matches!(&kind, Kind::Fixed32) {
                "fixed32"
            } else {
                "uint32"
            },
            primitive_data::PrimitiveData::Unsigned(0),
        )
        .into(),
        Kind::Uint64 | Kind::Fixed64 => tree::NodeType::ProtoPrimitive(
            if matches!(&kind, Kind::Fixed64) {
                "fixed64"
            } else {
                "uint64"
            },
            primitive_data::PrimitiveData::Unsigned(0),
        )
        .into(),
        Kind::Float => {
            tree::NodeType::ProtoPrimitive("float", primitive_data::PrimitiveData::Float(0.0))
                .into()
        }
        Kind::Double => {
            tree::NodeType::ProtoPrimitive("double", primitive_data::PrimitiveData::Float(0.0))
                .into()
        }
        Kind::String => tree::NodeType::ProtoPrimitive(
            "string",
            primitive_data::PrimitiveData::String(String::new()),
        )
        .into(),
        Kind::Bytes => {
            tree::NodeType::ProtoPrimitive("bytes", primitive_data::PrimitiveData::Bytes(vec![]))
                .into()
        }
    }
}

#[allow(
    clippy::large_enum_variant,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_lazy_continuation
)] // caused by generated code
pub mod substrait {
    // The core `substrait` and `substrait.extensions` packages come from the
    // pre-generated substrait-prost crate. The glob re-export makes the types
    // (and the nested `extensions`/`expression`/`r#type`/... submodules)
    // available under the same paths the rest of the validator already uses.
    pub use ::substrait_prost::*;

    // The validator-specific `substrait.validator` package is not provided by
    // substrait-prost, so it is still generated locally with the ProtoMeta
    // derive (see build.rs). substrait-prost exposes no `validator` module, so
    // there is no conflict with the glob re-export above.
    pub mod validator {
        include!(concat!(env!("OUT_DIR"), "/substrait.validator.rs"));
    }
}

/// Introspection trait impls (`InputNode`/`ProtoMessage`/`ProtoOneOf`/
/// `ProtoEnum`) for the substrait-prost types, generated by build.rs by walking
/// substrait-prost's embedded protobuf descriptor for the type names and
/// structure. The impls are thin: `parse_unknown` defers to
/// [`parse_proto_message_unknown`](crate::parse::traversal::parse_proto_message_unknown),
/// which introspects the message at runtime via the `prost_reflect::ReflectMessage`
/// impls substrait-prost provides — so the generator no longer reconstructs
/// prost's field boxing/naming decisions.
// `deprecated` because the emitted `proto_enum_from_i32` calls prost's
// deprecated `from_i32`; `clippy::all` to leave generated code unlinted.
#[allow(deprecated, clippy::all)]
mod substrait_prost_meta {
    include!(concat!(env!("OUT_DIR"), "/substrait_prost_meta.rs"));
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

#[cfg(test)]
mod prost_meta_tests {
    //! Regression tests for the introspection trait impls generated by
    //! build.rs from substrait-prost's protobuf descriptor. These pin the
    //! descriptor-derived naming (Rust module paths, protobuf type-name
    //! strings, enum variants, oneof variants) so that prost code-generation
    //! drift in a future substrait-prost release is caught here rather than as
    //! a confusing behavioral change. The boxing decisions are guarded by the
    //! compiler (a wrong choice fails to compile the generated file).

    use super::substrait;
    use crate::input::traits::{ProtoEnum, ProtoMessage, ProtoOneOf};

    #[test]
    fn nested_message_type_names() {
        // Nested-message module naming with a Rust-keyword segment (`r#type`).
        assert_eq!(
            substrait::r#type::List::proto_message_type(),
            "substrait.Type.List"
        );
        // Top-level message in the core package.
        assert_eq!(substrait::Rel::proto_message_type(), "substrait.Rel");
        // The `substrait.extensions` package maps to a separate module, and the
        // `URN` acronym must keep its original casing in the protobuf name even
        // though the Rust type is `SimpleExtensionUrn`.
        assert_eq!(
            substrait::extensions::SimpleExtensionUrn::proto_message_type(),
            "substrait.extensions.SimpleExtensionURN"
        );
    }

    #[test]
    fn nested_enum() {
        assert_eq!(
            substrait::sort_field::SortDirection::proto_enum_type(),
            "substrait.SortField.SortDirection"
        );
        assert_eq!(
            substrait::sort_field::SortDirection::proto_enum_default_variant(),
            "SORT_DIRECTION_UNSPECIFIED"
        );
        // Multi-word variant: the enum-name prefix is stripped to form the Rust
        // ident, but the protobuf value name is reported in full.
        assert_eq!(
            substrait::sort_field::SortDirection::AscNullsFirst.proto_enum_variant(),
            "SORT_DIRECTION_ASC_NULLS_FIRST"
        );
        assert_eq!(
            substrait::sort_field::SortDirection::proto_enum_from_i32(1),
            Some(substrait::sort_field::SortDirection::AscNullsFirst)
        );
    }

    #[test]
    fn oneof_with_mixed_boxing() {
        // `rel::RelType` has both boxed (`Read(Box<ReadRel>)`) and unboxed
        // (`Set(SetRel)`) variants; both must report their member field name.
        let read = substrait::rel::RelType::Read(Box::default());
        assert_eq!(read.proto_oneof_variant(), "read");
        let set = substrait::rel::RelType::Set(substrait::SetRel::default());
        assert_eq!(set.proto_oneof_variant(), "set");
    }
}
