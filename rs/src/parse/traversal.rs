// SPDX-License-Identifier: Apache-2.0

//! Module for the boilerplate code involved with traversing an input
//! protobuf/YAML tree to form the output [tree](tree::Node).
//!
//! Refer to the documentation for [`parse`](mod@crate::parse) for more
//! information.

// TODO: remove once validation code is finished.
#![allow(dead_code)]
#![allow(unused_macros)]

use crate::input::config;
use crate::input::traits::InputNode;
use crate::input::traits::ProtoEnum;
use crate::input::yaml;
use crate::output::diagnostic;
use crate::output::parse_result;
use crate::output::path;
use crate::output::primitive_data;
use crate::output::tree;
use crate::parse::context;
use std::sync::Arc;

//=============================================================================
// Type definitions
//=============================================================================

// Return value for parse macros for optional fields. The first element refers
// to the node for the field, if the field was present. The second is the
// return value of the supplied parse function, if it was called and didn't
// fail.
type OptionalResult<T> = (Option<Arc<tree::Node>>, Option<T>);

// Return value for parse macros for required fields. The first element refers
// to the node for the field; if the required field wasn't actually specified,
// a dummy node would have been made, so this is not an Option. The second is
// the return value of the supplied parse function, if it was called and didn't
// fail, just like for OptionalResult<T>.
type RequiredResult<T> = (Arc<tree::Node>, Option<T>);

// Return value for parse macros for repeated fields. Same as RequiredResult,
// but with each tuple entry wrapped in a vector. Both vectors will have equal
// length.
type RepeatedResult<T> = (Vec<Arc<tree::Node>>, Vec<Option<T>>);

//=============================================================================
// Macros for pushing annotations
//=============================================================================

/// Convenience/shorthand macro for pushing diagnostic messages to a node.
macro_rules! diagnostic {
    ($context:expr, $level:ident, $class:ident, $($args:expr),*) => {
        diagnostic!($context, $level, cause!($class, $($args),*))
    };
    ($context:expr, $level:ident, $cause:expr) => {
        crate::parse::traversal::push_diagnostic($context, crate::output::diagnostic::Level::$level, $cause)
    };
    ($context:expr, $diag:expr) => {
        $context.push_diagnostic($diag)
    };
}
macro_rules! ediagnostic {
    ($context:expr, $level:ident, $class:ident, $err:expr) => {
        diagnostic!($context, $level, ecause!($class, $err))
    };
}

/// Pushes a diagnostic message to the node information list.
pub fn push_diagnostic(
    context: &mut context::Context,
    level: diagnostic::Level,
    cause: diagnostic::Cause,
) {
    context.push_diagnostic(diagnostic::RawDiagnostic {
        cause,
        level,
        path: context.path_buf(),
    });
}

/// Convenience/shorthand macro for pushing formatted comments to a node.
macro_rules! comment {
    ($context:expr, $($fmts:expr),*) => {
        $context.push_comment(format!($($fmts),*))
    };
}

/// Convenience/shorthand macro for pushing formatted comments that link to
/// some path to a node.
macro_rules! link {
    ($context:expr, $path:expr, $($fmts:expr),*) => {
        $context.push_comment(crate::output::comment::Comment::new().link(format!($($fmts),*), $path))
    };
}

/// Convenience/shorthand macro for setting descriptive information for a node.
macro_rules! describe {
    ($context:expr, $class:ident, $($fmts:expr),*) => {
        $context.set_description(crate::output::tree::Class::$class, Some(format!($($fmts),*)))
    };
}

/// Convenience/shorthand macro for appending plain text to the summary of a
/// node.
macro_rules! summary {
    ($context:expr, $($fmts:expr),*) => {
        $context.push_summary(format!($($fmts),*))
    };
}

//=============================================================================
// Generic code for field handling
//=============================================================================

/// Parses a child node and pushes it into the provided parent context.
fn push_child<TF, TR, FP>(
    context: &mut context::Context,
    child: &TF,
    path_element: path::PathElement,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    TF: InputNode,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    // Create the node for the child.
    let mut field_output = child.data_to_node();

    // Create the context for calling the parse function for the child.
    let mut field_context = context.child(&mut field_output, path_element.clone());

    // Call the provided parser function.
    let result = parser(child, &mut field_context)
        .map_err(|cause| {
            diagnostic!(&mut field_context, Error, cause);
        })
        .ok();

    // Handle any fields not handled by the provided parse function. Only
    // generate a warning diagnostic for unhandled children if the parse
    // function succeeded and we're not already in an unknown subtree.
    handle_unknown_children(
        child,
        &mut field_context,
        result.is_some() && !unknown_subtree,
    );

    // Push and return the completed node.
    let field_output = Arc::new(field_output);
    context.push(tree::NodeData::Child(tree::Child {
        path_element,
        node: field_output.clone(),
        recognized: !unknown_subtree,
    }));

    (field_output, result)
}

/// Handle all children that haven't already been handled. If with_diagnostic
/// is set, this also generates a diagnostic message if there were
/// populated/non-default unhandled fields.
fn handle_unknown_children<T: InputNode>(
    input: &T,
    context: &mut context::Context,
    with_diagnostic: bool,
) {
    if input.parse_unknown(context) && with_diagnostic {
        let mut fields = vec![];
        for data in context.node_data().iter() {
            if let tree::NodeData::Child(child) = data {
                if !child.recognized {
                    fields.push(child.path_element.to_string_without_dot());
                }
            }
        }
        if !fields.is_empty() {
            let fields: String =
                itertools::Itertools::intersperse(fields.into_iter(), ", ".to_string()).collect();
            diagnostic!(
                context,
                Warning,
                NotYetImplemented,
                "the following child nodes were not recognized by the validator: {fields}"
            );
        }
    }
}

//=============================================================================
// Protobuf optional field handling
//=============================================================================

/// Convenience/shorthand macro for parsing optional protobuf fields.
macro_rules! proto_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_field(
            $context,
            &$input.$field.as_ref(),
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Convenience/shorthand macro for parsing optional protobuf fields that were
/// wrapped in a Box<T> by prost.
macro_rules! proto_boxed_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_boxed_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_field(
            $context,
            &$input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_boxed_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push a protobuf optional field.
pub fn push_proto_field<TF, TR, FP>(
    context: &mut context::Context,
    field: &Option<impl std::ops::Deref<Target = TF>>,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
) -> OptionalResult<TR>
where
    TF: InputNode,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    if !context.set_field_parsed(field_name) {
        panic!("field {field_name} was parsed multiple times");
    }

    if let Some(field_input) = field {
        let path_element = if let Some(variant) = field_input.oneof_variant() {
            path::PathElement::Variant(field_name.to_string(), variant.to_string())
        } else {
            path::PathElement::Field(field_name.to_string())
        };
        let (field_output, result) = push_child(
            context,
            field_input.deref(),
            path_element,
            unknown_subtree,
            parser,
        );
        (Some(field_output), result)
    } else {
        (None, None)
    }
}

//=============================================================================
// Protobuf required and primitive field handling
//=============================================================================

/// Convenience/shorthand macro for parsing required protobuf fields.
macro_rules! proto_required_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_required_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_required_field(
            $context,
            &$input.$field.as_ref(),
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_required_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Convenience/shorthand macro for parsing required protobuf fields that were
/// wrapped in a Box<T> by prost.
macro_rules! proto_boxed_required_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_boxed_required_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_required_field(
            $context,
            &$input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_boxed_required_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Convenience/shorthand macro for parsing primitive protobuf fields.
macro_rules! proto_primitive_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_primitive_field!($input, $context, $field, |x, _| Ok(x.to_owned()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        crate::parse::traversal::push_proto_required_field(
            $context,
            &Some(&$input.$field),
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $($args:expr),*) => {
        proto_primitive_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push a required field of some message type. If the field is
/// not populated, a MissingField diagnostic is pushed automatically, and
/// an empty node is returned as an error recovery placeholder.
pub fn push_proto_required_field<TF, TR, FP>(
    context: &mut context::Context,
    field: &Option<impl std::ops::Deref<Target = TF>>,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    TF: InputNode,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let (Some(node), result) =
        push_proto_field(context, field, field_name, unknown_subtree, parser)
    {
        (node, result)
    } else {
        ediagnostic!(context, Error, ProtoMissingField, field_name);
        (Arc::new(TF::type_to_node()), None)
    }
}

/// Convenience/shorthand macro for parsing enumeration protobuf fields.
macro_rules! proto_enum_field {
    ($input:expr, $context:expr, $field:ident, $typ:ty) => {
        proto_enum_field!($input, $context, $field, $typ, |x, _| Ok(x.to_owned()))
    };
    ($input:expr, $context:expr, $field:ident, $typ:ty, $parser:expr) => {
        crate::parse::traversal::push_proto_enum_field::<$typ, _, _>(
            $context,
            $input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $typ:ty, $parser:expr, $($args:expr),*) => {
        proto_enum_field!($input, $context, $field, $typ, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push an enumeration field of some message type. The i32 in the
/// struct generated by prost is automatically converted to the enum; if the
/// value is out of range, an error is generated.
pub fn push_proto_enum_field<TF, TR, FP>(
    context: &mut context::Context,
    field: i32,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    TF: ProtoEnum,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let Some(field) = TF::proto_enum_from_i32(field) {
        push_proto_required_field(context, &Some(&field), field_name, unknown_subtree, parser)
    } else {
        (
            push_proto_required_field(
                context,
                &Some(&field),
                field_name,
                unknown_subtree,
                |x, y| {
                    diagnostic!(
                        y,
                        Error,
                        IllegalValue,
                        "unknown value {x} for {}",
                        TF::proto_enum_type()
                    );
                    Ok(())
                },
            )
            .0,
            None,
        )
    }
}

/// Convenience/shorthand macro for parsing enumeration protobuf fields of
/// which the value must be specified.
macro_rules! proto_required_enum_field {
    ($input:expr, $context:expr, $field:ident, $typ:ty) => {
        proto_required_enum_field!($input, $context, $field, $typ, |x, _| Ok(x.to_owned()))
    };
    ($input:expr, $context:expr, $field:ident, $typ:ty, $parser:expr) => {
        crate::parse::traversal::push_proto_required_enum_field::<$typ, _, _>(
            $context,
            $input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
        )
    };
    ($input:expr, $context:expr, $field:ident, $typ:ty, $parser:expr, $($args:expr),*) => {
        proto_required_enum_field!($input, $context, $field, $typ, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push an enumeration field of some message type. The i32 in the
/// struct generated by prost is automatically converted to the enum; if the
/// value is out of range, an error is generated.
pub fn push_proto_required_enum_field<TF, TR, FP>(
    context: &mut context::Context,
    field: i32,
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    TF: ProtoEnum,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    push_proto_enum_field(context, field, field_name, unknown_subtree, |x, y| {
        if field == 0 {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "this enum may not be left unspecified"
            );
        }
        parser(x, y)
    })
}

//=============================================================================
// Protobuf repeated field handling
//=============================================================================

/// Convenience/shorthand macro for parsing repeated protobuf fields.
macro_rules! proto_repeated_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_repeated_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        proto_repeated_field!($input, $context, $field, $parser, |_, _, _, _, _| ())
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_repeated_field(
            $context,
            &$input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
            $validator,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr, $($args:expr),*) => {
        proto_repeated_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*), $validator)
    };
}

/// Parse and push a repeated field of some message type.
pub fn push_proto_repeated_field<TF, TR, FP, FV>(
    context: &mut context::Context,
    field: &[TF],
    field_name: &'static str,
    unknown_subtree: bool,
    mut parser: FP,
    mut validator: FV,
) -> RepeatedResult<TR>
where
    TF: InputNode,
    FP: FnMut(&TF, &mut context::Context) -> diagnostic::Result<TR>,
    FV: FnMut(&TF, &mut context::Context, usize, &Arc<tree::Node>, Option<&TR>),
{
    if !context.set_field_parsed(field_name) {
        panic!("field {field_name} was parsed multiple times");
    }

    field
        .iter()
        .enumerate()
        .map(|(index, child)| {
            let (node, result) = push_child(
                context,
                child,
                path::PathElement::Repeated(field_name.to_string(), index),
                unknown_subtree,
                &mut parser,
            );
            validator(child, context, index, &node, result.as_ref());
            (node, result)
        })
        .unzip()
}

/// Convenience/shorthand macro for parsing repeated protobuf fields for which
/// at least one element must exist.
macro_rules! proto_required_repeated_field {
    ($input:expr, $context:expr, $field:ident) => {
        proto_required_repeated_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr) => {
        proto_required_repeated_field!($input, $context, $field, $parser, |_, _, _, _, _| ())
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr) => {
        crate::parse::traversal::push_proto_required_repeated_field(
            $context,
            &$input.$field,
            crate::input::proto::cook_ident(stringify!($field)),
            false,
            $parser,
            $validator,
        )
    };
    ($input:expr, $context:expr, $field:ident, $parser:expr, $validator:expr, $($args:expr),*) => {
        proto_required_repeated_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*), $validator)
    };
}

/// Parse and push a repeated field of some message type, and check that at
/// least one element exists.
pub fn push_proto_required_repeated_field<TF, TR, FP, FV>(
    context: &mut context::Context,
    field: &[TF],
    field_name: &'static str,
    unknown_subtree: bool,
    parser: FP,
    validator: FV,
) -> RepeatedResult<TR>
where
    TF: InputNode,
    FP: FnMut(&TF, &mut context::Context) -> diagnostic::Result<TR>,
    FV: FnMut(&TF, &mut context::Context, usize, &Arc<tree::Node>, Option<&TR>),
{
    if field.is_empty() {
        ediagnostic!(context, Error, ProtoMissingField, field_name);
    }
    push_proto_repeated_field(
        context,
        field,
        field_name,
        unknown_subtree,
        parser,
        validator,
    )
}

//=============================================================================
// Protobuf root message handling
//=============================================================================

/// Parses a serialized protobuf message using the given root parse function,
/// initial state, and configuration.
pub fn parse_proto<T, F, B>(
    buffer: B,
    root_name: &'static str,
    root_parser: F,
    state: &mut context::State,
    config: &config::Config,
) -> diagnostic::Result<parse_result::ParseResult>
where
    T: prost::Message + InputNode + Default,
    F: FnOnce(&T, &mut context::Context) -> diagnostic::Result<()>,
    B: prost::bytes::Buf,
{
    // Run protobuf deserialization.
    let input = T::decode(buffer).map_err(|e| ecause!(ProtoParseFailed, e))?;

    // Create the root node.
    let mut root = input.data_to_node();

    // Create the root context.
    let mut context = context::Context::new(root_name, &mut root, state, config);

    // Call the provided parser function.
    let success = root_parser(&input, &mut context)
        .map_err(|cause| {
            diagnostic!(&mut context, Error, cause);
        })
        .is_ok();

    // Handle any fields not handled by the provided parse function.
    // Only generate a warning diagnostic for unhandled children if the
    // parse function succeeded.
    handle_unknown_children(&input, &mut context, success);

    Ok(parse_result::ParseResult { root })
}

//=============================================================================
// YAML object handling
//=============================================================================

/// Convenience/shorthand macro for parsing optional YAML fields.
macro_rules! yaml_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        crate::parse::traversal::push_yaml_field($input, $context, $field, false, $parser)
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $($args:expr),*) => {
        yaml_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push an optional YAML field.
pub fn push_yaml_field<TS, TR, FP>(
    input: &yaml::Value,
    context: &mut context::Context,
    field_name: TS,
    unknown_subtree: bool,
    parser: FP,
) -> diagnostic::Result<OptionalResult<TR>>
where
    TS: AsRef<str>,
    FP: FnOnce(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let serde_json::Value::Object(input) = input {
        let field_name = field_name.as_ref();
        if !context.set_field_parsed(field_name) {
            panic!("field {field_name} was parsed multiple times");
        }

        if let Some(child) = input.get(field_name) {
            let (field_output, result) = push_child(
                context,
                child,
                path::PathElement::Field(field_name.to_string()),
                unknown_subtree,
                parser,
            );
            Ok((Some(field_output), result))
        } else {
            Ok((None, None))
        }
    } else {
        Err(cause!(YamlInvalidType, "object expected"))
    }
}

/// Convenience/shorthand macro for parsing required YAML fields.
macro_rules! yaml_required_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_required_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        crate::parse::traversal::push_yaml_required_field($input, $context, $field, false, $parser)
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $($args:expr),*) => {
        yaml_required_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*))
    };
}

/// Parse and push a required field of a YAML object. If the field does not
/// exist, a MissingField diagnostic is pushed automatically, and an empty node
/// is returned as an error recovery placeholder.
pub fn push_yaml_required_field<TS, TR, FP>(
    input: &yaml::Value,
    context: &mut context::Context,
    field_name: TS,
    unknown_subtree: bool,
    parser: FP,
) -> diagnostic::Result<RequiredResult<TR>>
where
    TS: AsRef<str>,
    FP: FnOnce(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    let field_name = field_name.as_ref();
    if let (Some(node), result) =
        push_yaml_field(input, context, field_name, unknown_subtree, parser)?
    {
        Ok((node, result))
    } else {
        ediagnostic!(context, Error, YamlMissingKey, field_name);
        Ok((
            Arc::new(tree::NodeType::YamlPrimitive(primitive_data::PrimitiveData::Null).into()),
            None,
        ))
    }
}

//=============================================================================
// YAML array handling
//=============================================================================

/// Convenience/shorthand macro for parsing a YAML array that may be empty.
macro_rules! yaml_array {
    ($input:expr, $context:expr) => {
        yaml_array!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $parser:expr) => {
        yaml_array!($input, $context, $field, $parser, 0)
    };
    ($input:expr, $context:expr, $parser:expr, $min_size:expr) => {
        crate::parse::traversal::push_yaml_array($input, $context, $min_size, false, $parser)
    };
    ($input:expr, $context:expr, $parser:expr, $min_size:expr, $($args:expr),*) => {
        yaml_array!($input, $context, |x, y| $parser(x, y, $($args),*), $min_size)
    };
}

/// Convenience/shorthand macro for parsing a YAML array that must have at
/// least one value.
macro_rules! yaml_required_array {
    ($input:expr, $context:expr) => {
        yaml_required_array!($input, $context, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $parser:expr) => {
        yaml_array!($input, $context, $parser, 1)
    };
    ($input:expr, $context:expr, $parser:expr, $($args:expr),*) => {
        yaml_array!($input, $context, $parser, 1, $($args:expr),*)
    };
}

/// Parse and push an optional YAML array element.
pub fn push_yaml_element<TR, FP>(
    input: &yaml::Array,
    context: &mut context::Context,
    index: usize,
    unknown_subtree: bool,
    parser: FP,
) -> OptionalResult<TR>
where
    FP: FnOnce(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    if !context.set_field_parsed(index) {
        panic!("element {index} was parsed multiple times");
    }

    if let Some(child) = input.get(index) {
        let (field_output, result) = push_child(
            context,
            child,
            path::PathElement::Index(index),
            unknown_subtree,
            parser,
        );
        (Some(field_output), result)
    } else {
        (None, None)
    }
}

/// Parse and push a required element of a YAML array. If the element does not
/// exist, a MissingElement diagnostic is pushed automatically, and an empty node
/// is returned as an error recovery placeholder.
pub fn push_yaml_required_element<TR, FP>(
    input: &yaml::Array,
    context: &mut context::Context,
    index: usize,
    unknown_subtree: bool,
    parser: FP,
) -> RequiredResult<TR>
where
    FP: FnOnce(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let (Some(node), result) = push_yaml_element(input, context, index, unknown_subtree, parser)
    {
        (node, result)
    } else {
        diagnostic!(context, Error, YamlMissingElement, "index {index}");
        (
            Arc::new(tree::NodeType::YamlPrimitive(primitive_data::PrimitiveData::Null).into()),
            None,
        )
    }
}

/// Parse and push a complete YAML array. If a required element does not exist,
/// a MissingElement diagnostic is pushed automatically, and an empty node is
/// returned as an error recovery placeholder.
pub fn push_yaml_array<TR, FP>(
    input: &yaml::Value,
    context: &mut context::Context,
    min_size: usize,
    unknown_subtree: bool,
    mut parser: FP,
) -> diagnostic::Result<RepeatedResult<TR>>
where
    FP: FnMut(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    if let serde_json::Value::Array(input) = input {
        let size = std::cmp::max(min_size, input.len());
        Ok((0..size)
            .into_iter()
            .map(|index| {
                push_yaml_required_element(input, context, index, unknown_subtree, &mut parser)
            })
            .unzip())
    } else {
        Err(cause!(YamlInvalidType, "array expected"))
    }
}

/// Shorthand for fields that must be arrays if specified.
macro_rules! yaml_repeated_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_repeated_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        yaml_repeated_field!($input, $context, $field, $parser, 0)
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $min_size:expr) => {
        crate::parse::traversal::push_yaml_repeated_field(
            $input, $context, $field, false, $min_size, false, $parser,
        )
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $min_size:expr, $($args:expr),*) => {
        yaml_repeated_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*), $min_size)
    };
}

/// Shorthand for fields that must be arrays.
macro_rules! yaml_required_repeated_field {
    ($input:expr, $context:expr, $field:expr) => {
        yaml_required_repeated_field!($input, $context, $field, |_, _| Ok(()))
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr) => {
        yaml_required_repeated_field!($input, $context, $field, $parser, 1)
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $min_size:expr) => {
        crate::parse::traversal::push_yaml_repeated_field(
            $input, $context, $field, true, $min_size, false, $parser,
        )
    };
    ($input:expr, $context:expr, $field:expr, $parser:expr, $min_size:expr, $($args:expr),*) => {
        yaml_required_repeated_field!($input, $context, $field, |x, y| $parser(x, y, $($args),*), $min_size)
    };
}

/// Parse and push a complete YAML array. If a required element does not exist,
/// a MissingElement diagnostic is pushed automatically, and an empty node is
/// returned as an error recovery placeholder.
pub fn push_yaml_repeated_field<TR, FP>(
    input: &yaml::Value,
    context: &mut context::Context,
    field_name: &'static str,
    field_required: bool,
    min_size: usize,
    unknown_subtree: bool,
    parser: FP,
) -> diagnostic::Result<RepeatedResult<TR>>
where
    FP: FnMut(&yaml::Value, &mut context::Context) -> diagnostic::Result<TR>,
{
    Ok(if field_required {
        push_yaml_required_field(input, context, field_name, unknown_subtree, |x, y| {
            yaml_array!(x, y, parser, min_size)
        })?
        .1
    } else {
        push_yaml_field(input, context, field_name, unknown_subtree, |x, y| {
            yaml_array!(x, y, parser, min_size)
        })?
        .1
    }
    .unwrap_or_else(|| (vec![], vec![])))
}

//=============================================================================
// YAML primitive handling
//=============================================================================

/// Convenience/shorthand macro for parsing optional YAML fields.
macro_rules! yaml_prim {
    ($typ:ident) => {
        |x, y| crate::parse::traversal::yaml_primitive_parsers::$typ(x, y, |x, _| Ok(x.to_owned()))
    };
    ($typ:ident, $parser:expr) => {
        |x, y| crate::parse::traversal::yaml_primitive_parsers::$typ(x, y, $parser)
    };
}

pub mod yaml_primitive_parsers {
    use super::*;

    /// Boolean primitive helper.
    pub fn bool<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&bool, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::Bool(x) = x {
            parser(x, y)
        } else {
            Err(cause!(YamlInvalidType, "string expected"))
        }
    }

    /// Signed integer primitive helper.
    pub fn i64<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&i64, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::Number(x) = x {
            if let Some(x) = x.as_i64() {
                return parser(&x, y);
            }
        }
        Err(cause!(YamlInvalidType, "signed integer expected"))
    }

    /// Unsigned integer primitive helper.
    pub fn u64<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&u64, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::Number(x) = x {
            if let Some(x) = x.as_u64() {
                return parser(&x, y);
            }
        }
        Err(cause!(YamlInvalidType, "unsigned integer expected"))
    }

    /// Float primitive helper.
    pub fn f64<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&f64, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::Number(x) = x {
            if let Some(x) = x.as_f64() {
                return parser(&x, y);
            }
        }
        Err(cause!(YamlInvalidType, "floating point number expected"))
    }

    /// String primitive helper.
    pub fn str<TR, FP>(
        x: &yaml::Value,
        y: &mut context::Context,
        parser: FP,
    ) -> diagnostic::Result<TR>
    where
        FP: FnOnce(&str, &mut context::Context) -> diagnostic::Result<TR>,
    {
        if let serde_json::Value::String(x) = x {
            parser(x, y)
        } else {
            Err(cause!(YamlInvalidType, "string expected"))
        }
    }
}

//=============================================================================
// URI resolution and YAML root handling
//=============================================================================

/// Worker for resolve_uri().
fn resolve_uri(uri: &str, context: &mut context::Context) -> Option<config::BinaryData> {
    // Check for cyclic dependencies.
    let uri_stack = context.uri_stack();
    if let Some((index, _)) = uri_stack.iter().enumerate().find(|(_, x)| &x[..] == uri) {
        let cycle = itertools::Itertools::intersperse(
            uri_stack
                .iter()
                .map(|x| &x[index..])
                .chain(std::iter::once(uri)),
            " -> ",
        )
        .collect::<String>();
        diagnostic!(context, Error, YamlCyclicDependency, "{cycle}");
        return None;
    }

    // Check for recursion limit.
    if let Some(max_depth) = context.config.max_uri_resolution_depth {
        if context.uri_stack().len() >= max_depth {
            diagnostic!(
                context,
                Warning,
                YamlResolutionDisabled,
                "configured recursion limit for URI resolution has been reached"
            );
            return None;
        }
    }

    // Apply yaml_uri_overrides configuration.
    let remapped_uri = context
        .config
        .uri_overrides
        .iter()
        .find_map(|(pattern, mapping)| {
            if pattern.matches(uri) {
                Some(mapping.as_ref().map(|x| &x[..]))
            } else {
                None
            }
        });
    let is_remapped = remapped_uri.is_some();
    let remapped_uri = remapped_uri.unwrap_or(Some(uri));

    let remapped_uri = if let Some(remapped_uri) = remapped_uri {
        remapped_uri.to_owned()
    } else {
        diagnostic!(
            context,
            Warning,
            YamlResolutionDisabled,
            "YAML resolution for {uri} was disabled"
        );
        return None;
    };
    if is_remapped {
        diagnostic!(context, Info, Yaml, "URI was remapped to {remapped_uri}");
    }

    // If a custom download function is specified, use it to resolve.
    if let Some(ref resolver) = context.config.uri_resolver {
        return match resolver(&remapped_uri) {
            Ok(x) => Some(x),
            Err(e) => {
                diagnostic!(context, Warning, YamlResolutionFailed, "{e}");
                None
            }
        };
    }

    // Parse as a URL.
    let url = match url::Url::parse(&remapped_uri) {
        Ok(url) => url,
        Err(e) => {
            if is_remapped {
                diagnostic!(
                    context,
                    Warning,
                    YamlResolutionFailed,
                    "configured URI remapping ({remapped_uri}) did not parse as URL: {e}"
                )
            } else {
                diagnostic!(
                    context,
                    Warning,
                    YamlResolutionFailed,
                    "failed to parse {remapped_uri} as URL: {e}"
                )
            };
            return None;
        }
    };

    // Reject anything that isn't file://-based.
    if url.scheme() != "file" {
        if is_remapped {
            diagnostic!(
                context,
                Warning,
                YamlResolutionFailed,
                "configured URI remapping ({remapped_uri}) does not use file:// scheme"
            )
        } else {
            diagnostic!(
                context,
                Warning,
                YamlResolutionFailed,
                "URI does not use file:// scheme"
            )
        };
        return None;
    }

    // Convert to path.
    let path =
        match url.to_file_path() {
            Ok(path) => path,
            Err(_) => {
                if is_remapped {
                    diagnostic!(context, Warning,
                    YamlResolutionFailed,
                    "configured URI remapping ({remapped_uri}) could not be converted to file path"
                )
                } else {
                    diagnostic!(
                        context,
                        Warning,
                        YamlResolutionFailed,
                        "URI could not be converted to file path"
                    )
                };
                return None;
            }
        };

    // Read the file.
    match std::fs::read(path) {
        Ok(data) => Some(Box::new(data)),
        Err(e) => {
            if is_remapped {
                diagnostic!(
                    context,
                    Warning,
                    YamlResolutionFailed,
                    "failed to file remapping for URI ({remapped_uri}): {e}"
                );
            } else {
                ediagnostic!(context, Warning, YamlResolutionFailed, e);
            }
            None
        }
    }
}

/// Attempts to resolve a URI. If resolution fails or is disabled, None is
/// returned and an appropriate diagnostic may be emitted. This function
/// includes detection and prevention of recursive resolution. The reader
/// function will first be called to convert the binary data from the
/// resolution to a traversable tree (something that satisfied InputNode),
/// then the parser will be called on the root of that tree.
pub fn parse_uri<FP, FR, TF, TR>(
    uri: &str,
    context: &mut context::Context,
    reader: FR,
    parser: FP,
) -> OptionalResult<TR>
where
    TF: InputNode,
    FR: FnOnce(config::BinaryData, &mut context::Context) -> Option<TF>,
    FP: FnOnce(&TF, &mut context::Context) -> diagnostic::Result<TR>,
{
    // Try resolving the URI.
    if let Some(data) = resolve_uri(uri, context) {
        // Parse the flat file to a traversable tree.
        if let Some(root) = reader(data, context) {
            // Update recursion stack.
            context.uri_stack().push(uri.to_string());

            // Defer to the provided parser to handle parsing the resolved
            // data.
            let (field_output, result) = push_child(
                context,
                &root,
                path::PathElement::Field("data".to_string()),
                false,
                parser,
            );

            // Revert recursion stack update.
            context.uri_stack().pop();

            // Replace node type to make clear what the child node we just
            // added signifies.
            context.replace_node_type(tree::NodeType::ResolvedUri(uri.to_string()));

            // Success, at least to the point that a tree was formed. The
            // tree itself might still be invalid.
            return (Some(field_output), result);
        }
    }

    (None, None)
}

/// Read function for YAML files, to be used with [parse_uri()].
pub fn read_yaml(
    binary_data: config::BinaryData,
    context: &mut context::Context,
    schema: Option<&jsonschema::JSONSchema>,
) -> Option<yaml::Value> {
    // Parse as UTF-8.
    let string_data = match std::str::from_utf8(binary_data.as_ref().as_ref()) {
        Err(e) => {
            ediagnostic!(context, Error, YamlParseFailed, e);
            return None;
        }
        Ok(x) => x,
    };

    // Parse as YAML.
    let yaml_data = match serde_yaml::from_str::<serde_yaml::Value>(string_data) {
        Err(e) => {
            ediagnostic!(context, Error, YamlParseFailed, e);
            return None;
        }
        Ok(x) => x,
    };

    // Convert to JSON DOM.
    let json_data = match yaml::yaml_to_json(yaml_data, context.path()) {
        Err(e) => {
            diagnostic!(context, e);
            return None;
        }
        Ok(x) => x,
    };

    // Validate with schema.
    if let Some(schema) = schema {
        if let Err(es) = schema.validate(&json_data) {
            for e in es {
                ediagnostic!(context, Error, YamlSchemaValidationFailed, e);
            }
            return None;
        }
    }

    Some(json_data)
}

//=============================================================================
// ANTLR syntax tree node handling
//=============================================================================

/// Wrapper type to satisfy push_child()'s InputNode trait bound on the input
/// node type.
struct AntlrContextWrapper<'a, T>(&'a T);

impl<'a, T> InputNode for AntlrContextWrapper<'a, T> {
    fn type_to_node() -> tree::Node {
        tree::NodeType::AstNode.into()
    }

    fn data_to_node(&self) -> tree::Node {
        tree::NodeType::AstNode.into()
    }

    fn oneof_variant(&self) -> Option<&'static str> {
        None
    }

    fn parse_unknown(&self, _: &mut context::Context<'_>) -> bool {
        false
    }
}

/// Convenience/shorthand macro for traversing into a syntax tree node by node.
macro_rules! antlr_child {
    ($input:expr, $context:expr, $field:ident, $analyzer:expr) => {
        antlr_child!($input, $context, $field, 0, $analyzer)
    };
    ($input:expr, $context:expr, $field:ident, $index:expr, $analyzer:expr) => {
        crate::parse::traversal::push_antlr_child(
            $context,
            $input,
            $index,
            stringify!($field),
            $analyzer,
        )
    };
    ($input:expr, $context:expr, $field:ident, $index:expr, $analyzer:expr, $($args:expr),*) => {
        antlr_child!($input, $context, $field, $index, |x, y| $analyzer(x, y, $($args),*))
    };
}

/// Parse and push a child of an ANTLR syntax tree node.
pub fn push_antlr_child<'input, TP, TC, TR, FA>(
    context: &mut context::Context,
    parent: &TP,
    index: usize,
    field: &'static str,
    analyzer: FA,
) -> OptionalResult<TR>
where
    TP: antlr_rust::parser_rule_context::ParserRuleContext<'input>,
    FA: FnOnce(&TC, &mut context::Context) -> diagnostic::Result<TR>,
    TC: antlr_rust::parser_rule_context::ParserRuleContext<'input, TF = TP::TF, Ctx = TP::Ctx>
        + 'input,
{
    if let Some(child) = parent.child_of_type::<TC>(index) {
        let (field_output, result) = push_child(
            context,
            &AntlrContextWrapper(child.as_ref()),
            path::PathElement::Field(field.to_string()),
            false,
            |x: &AntlrContextWrapper<TC>, y| analyzer(x.0, y),
        );
        (Some(field_output), result)
    } else {
        (None, None)
    }
}

/// Convenience/shorthand macro for traversing into a syntax tree node by node.
/// Contrary to antlr_child! and most other traversal macros, this does NOT
/// make a child node in the resulting tree. It can be used to hide unobvious
/// grammar constructs, such rules related to avoiding left recursion.
macro_rules! antlr_hidden_child {
    ($input:expr, $context:expr, $analyzer:expr) => {
        antlr_hidden_child!($input, $context, 0, $analyzer)
    };
    ($input:expr, $context:expr, $index:expr, $analyzer:expr) => {
        crate::parse::traversal::push_antlr_hidden_child(
            $context,
            $input,
            $index,
            $analyzer,
        )
    };
    ($input:expr, $context:expr, $index:expr, $analyzer:expr, $($args:expr),*) => {
        antlr_hidden_child!($input, $context, $index, |x, y| $analyzer(x, y, $($args),*))
    };
}

/// Parse and push a child of an ANTLR syntax tree node, without making a
/// corresponding child node in the output tree.
pub fn push_antlr_hidden_child<'input, TP, TC, TR, FA>(
    context: &mut context::Context,
    parent: &TP,
    index: usize,
    analyzer: FA,
) -> Option<TR>
where
    TP: antlr_rust::parser_rule_context::ParserRuleContext<'input>,
    FA: FnOnce(&TC, &mut context::Context) -> diagnostic::Result<TR>,
    TC: antlr_rust::parser_rule_context::ParserRuleContext<'input, TF = TP::TF, Ctx = TP::Ctx>
        + 'input,
{
    parent.child_of_type::<TC>(index).and_then(|child| {
        analyzer(child.as_ref(), context)
            .map_err(|cause| {
                diagnostic!(context, Error, cause);
            })
            .ok()
    })
}

/// This does more or less the opposite of pushing a hidden child: it creates a
/// child node in the output tree without traversing deeper into the input
/// tree. It can be used to hide unobvious grammar constructs, such rules
/// related to avoiding left recursion.
macro_rules! antlr_recurse {
    ($input:expr, $context:expr, $field:ident, $analyzer:expr) => {
        crate::parse::traversal::push_antlr_recurse(
            $context,
            $input,
            stringify!($field),
            $analyzer,
        )
    };
    ($input:expr, $context:expr, $field:ident, $analyzer:expr, $($args:expr),*) => {
        antlr_recurse!($input, $context, $field, |x, y| $analyzer(x, y, $($args),*))
    };
}

/// Parse and push a child of an ANTLR syntax tree node.
pub fn push_antlr_recurse<'input, TP, TR, FA>(
    context: &mut context::Context,
    parent: &TP,
    field: &'static str,
    analyzer: FA,
) -> RequiredResult<TR>
where
    TP: antlr_rust::parser_rule_context::ParserRuleContext<'input>,
    FA: FnOnce(&TP, &mut context::Context) -> diagnostic::Result<TR>,
{
    push_child(
        context,
        &AntlrContextWrapper(parent),
        path::PathElement::Field(field.to_string()),
        false,
        |x: &AntlrContextWrapper<TP>, y| analyzer(x.0, y),
    )
}

/// Convenience/shorthand macro for traversing into all children of a certain
/// type in a syntax tree.
macro_rules! antlr_children {
    ($input:expr, $context:expr, $rule:ident, $analyzer:expr) => {
        crate::parse::traversal::push_antlr_children(
            $context,
            $input,
            stringify!($rule),
            $analyzer,
        )
    };
    ($input:expr, $context:expr, $rule:ident, $analyzer:expr, $($args:expr),*) => {
        antlr_children!($input, $context, $rule, |x, y| $analyzer(x, y, $($args),*))
    };
}

/// Parse and push a child of an ANTLR syntax tree node.
pub fn push_antlr_children<'input, TP, TC, TR, FA>(
    context: &mut context::Context,
    parent: &TP,
    field: &'static str,
    mut analyzer: FA,
) -> RepeatedResult<TR>
where
    TP: antlr_rust::parser_rule_context::ParserRuleContext<'input>,
    FA: FnMut(&TC, &mut context::Context) -> diagnostic::Result<TR>,
    TC: antlr_rust::parser_rule_context::ParserRuleContext<'input, TF = TP::TF, Ctx = TP::Ctx>
        + 'input,
{
    parent
        .children_of_type::<TC>()
        .into_iter()
        .enumerate()
        .map(|(index, child)| {
            push_child(
                context,
                &AntlrContextWrapper(child.as_ref()),
                path::PathElement::Repeated(field.to_string(), index),
                false,
                |x: &AntlrContextWrapper<TC>, y| analyzer(x.0, y),
            )
        })
        .unzip()
}
