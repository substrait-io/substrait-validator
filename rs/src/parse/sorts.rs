// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for sort fields.

use crate::input::proto::substrait;
use crate::input::traits::ProtoEnum;
use crate::output::comment;
use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::extensions;

/// Parse a sort direction.
fn parse_sort_direction(x: &i32, y: &mut context::Context) -> diagnostic::Result<&'static str> {
    use substrait::sort_field::SortDirection;
    match SortDirection::proto_enum_from_i32(*x) {
        None => {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "unknown value {x} for {}",
                SortDirection::proto_enum_type()
            );
            Ok("Invalid sort by")
        }
        Some(SortDirection::Unspecified) => {
            diagnostic!(y, Error, ProtoMissingField, "direction");
            Ok("Invalid sort by")
        }
        Some(SortDirection::AscNullsFirst) => {
            describe!(y, Misc, "Sort ascending, nulls first");
            Ok("Ascending sort by")
        }
        Some(SortDirection::AscNullsLast) => {
            describe!(y, Misc, "Sort ascending, nulls last");
            Ok("Ascending sort by")
        }
        Some(SortDirection::DescNullsFirst) => {
            describe!(y, Misc, "Sort descending, nulls first");
            Ok("Descending sort by")
        }
        Some(SortDirection::DescNullsLast) => {
            describe!(y, Misc, "Sort descending, nulls last");
            Ok("Descending sort by")
        }
        Some(SortDirection::Clustered) => {
            describe!(y, Misc, "Coalesce equal values");
            summary!(
                y,
                "Equal values are grouped together, but no ordering is defined between clusters."
            );
            Ok("Coalesce")
        }
    }
}

/// Parse a function reference that should resolve to a comparison function
/// (i.e. one usable for sorts) for the given type.
fn parse_comparison_function_reference(
    x: &u32,
    y: &mut context::Context,
    data_type: &data::Type,
) -> diagnostic::Result<&'static str> {
    // Resolve the reference as normal.
    let functions = extensions::simple::parse_function_reference(x, y)?;

    // Try to bind the function.
    let argument =
        expressions::functions::FunctionArgument::Value(data_type.clone(), Default::default());
    let context = expressions::functions::FunctionContext {
        function_type: expressions::functions::FunctionType::Scalar,
        arguments: vec![argument.clone(), argument],
        return_type: data::new_unresolved_type(),
    };
    let binding = expressions::functions::FunctionBinding::new(Some(&functions), &context, y);

    // Describe how the function is to be interpreted.
    let comment = comment::Comment::new()
        .plain("Comparison function for sorting. Taking two elements as input,")
        .plain("it must determine the correct sort order. The return value is");
    let comment = match binding.return_type.class() {
        data::Class::Simple(data::class::Simple::Boolean) => {
            let comment = comment
                .plain("interpreted as the result of a < b, so:")
                .lo()
                .plain("f(a, b) => true: a sorts before b.")
                .li()
                .plain("f(a, b) => false: b sorts before a.");
            if binding.return_type.nullable() {
                comment
                    .li()
                    .plain("f(a, b) => null: a and b have no defined sort order.")
            } else {
                comment
            }
        }
        data::Class::Simple(data::class::Simple::I8)
        | data::Class::Simple(data::class::Simple::I16)
        | data::Class::Simple(data::class::Simple::I32)
        | data::Class::Simple(data::class::Simple::I64) => {
            let comment = comment
                .plain("interpreted as follows:")
                .lo()
                .plain("f(a, b) => negative: a sorts before b.")
                .li()
                .plain("f(a, b) => positive: b sorts before a.");
            if binding.return_type.nullable() {
                comment
                    .li()
                    .plain("f(a, b) => zero or null: a and b have no defined sort order.")
            } else {
                comment
                    .li()
                    .plain("f(a, b) => null: a and b have no defined sort order.")
            }
        }
        _ => {
            if !binding.return_type.is_unresolved() {
                diagnostic!(
                    y,
                    Error,
                    TypeMismatch,
                    "comparison functions must yield booleans (a < b) or integers (a ?= b), but found {}",
                    binding.return_type
                );
            }
            comment
                .plain("interpreted as follows:")
                .lo()
                .plain("f(a, b) => true or negative: a sorts before b;")
                .li()
                .plain("f(a, b) => false or positive: b sorts before a;")
                .li()
                .plain("f(a, b) => 0 or null: a and b have no defined sort order.")
                .lc()
        }
    };
    y.push_summary(comment);

    Ok("Custom sort")
}

/// Parse a sort kind, applicable to elements of the given data type.
fn parse_sort_kind(
    x: &substrait::sort_field::SortKind,
    y: &mut context::Context,
    data_type: &data::Type,
) -> diagnostic::Result<&'static str> {
    match x {
        substrait::sort_field::SortKind::Direction(x) => parse_sort_direction(x, y),
        substrait::sort_field::SortKind::ComparisonFunctionReference(x) => {
            parse_comparison_function_reference(x, y, data_type)
        }
    }
}

/// Parse a sort field.
pub fn parse_sort_field(
    x: &substrait::SortField,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse fields.
    let (n, e) = proto_required_field!(x, y, expr, expressions::parse_expression);
    let expression = e.unwrap_or_default();
    let method = proto_required_field!(x, y, sort_kind, parse_sort_kind, &n.data_type())
        .1
        .unwrap_or("Invalid sort by");

    // Describe node.
    describe!(y, Misc, "{method} {expression}");
    summary!(y, "{method} {expression:#}.");
    Ok(expression)
}
