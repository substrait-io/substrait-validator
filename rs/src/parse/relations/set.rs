// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for set relations.
//!
//! The set operation encompasses several set level operations that support
//! combining datasets based, possibly excluding records based on various
//! types of record level matching.
//!
//! See <https://substrait.io/relations/logical_relations/#set-operation>

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::input::traits::ProtoEnum;
use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::parse::context;
use crate::parse::types;

enum Operation {
    Invalid,
    Subtract,
    SubtractByUnion,
    SubtractByIntersection,
    Intersect,
    IntersectWithUnion,
    Union,
    Merge,
}

/// Changes the nullability of the fields in a schema, preserving nested types.
fn map_field_nullability(
    schema: &data::Type,
    mut nullable: impl FnMut(usize) -> bool,
) -> data::Type {
    if !schema.is_struct() {
        return schema.clone();
    }
    let parameters = schema
        .parameters()
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, parameter)| {
            parameter
                .map(|value| value.map_data_type(|field| field.override_nullable(nullable(index))))
        })
        .collect();
    data::new_type(
        schema.class().clone(),
        schema.nullable(),
        schema.variation().clone(),
        parameters,
    )
    .expect("changing field nullability must preserve a valid schema")
}

/// Parse set relation.
pub fn parse_set_rel(x: &substrait::SetRel, y: &mut context::Context) -> diagnostic::Result<()> {
    use substrait::set_rel::SetOp;

    // Parse inputs.
    let in_types: Vec<_> = handle_rel_inputs!(x, y)
        .map(|schema| schema.strip_field_names())
        .collect();

    // Check inputs and derive schema.
    if in_types.len() < 2 {
        diagnostic!(
            y,
            Error,
            RelationMissing,
            "set operations require at least two input relations"
        );
    }
    let mut schema = Arc::default();
    for in_type in in_types.iter() {
        schema = types::assert_equal(
            y,
            &map_field_nullability(in_type, |_| false),
            &schema,
            "all set inputs must have matching schemas",
        );
    }

    // Set inputs may differ in field nullability. Derive it according to the
    // operation, reading the operation before the field is parsed so that this
    // node's data keeps the order every other relation has.
    let derived = SetOp::proto_enum_from_i32(x.op).unwrap_or_default();
    let compared = schema.clone();
    schema = map_field_nullability(&schema, |index| {
        // A field votes only when it agrees with the schema the comparison
        // derived, ignoring nullability. An absent, unresolved or mismatched
        // field carries no nullability information, and counting it as nullable
        // would publish a type no input supports.
        let vote = |input: &data::Type| {
            let field = input.index_struct(index)?;
            let compared_field = compared.index_struct(index)?;
            (!field.is_unresolved()
                && !compared_field.is_unresolved()
                && field.override_nullable(false) == compared_field.override_nullable(false))
            .then(|| field.nullable())
        };
        let mut inputs = in_types.iter();
        let mut nullabilities = in_types.iter().skip(1).filter_map(vote).peekable();
        let primary = match inputs.next().and_then(vote) {
            Some(nullable) => nullable,
            // Every operation below takes the primary's nullability as its
            // starting point, so without one fall back to the informative
            // secondary inputs rather than claiming either way.
            None => return nullabilities.any(|nullable| nullable),
        };
        match derived {
            SetOp::Unspecified
            | SetOp::MinusPrimary
            | SetOp::MinusPrimaryAll
            | SetOp::MinusMultiset => primary,
            // The diagnostic for too few inputs is not fatal, so there may be no
            // informative secondary input: fall back to the primary rather than
            // narrowing the field to required on no evidence.
            SetOp::IntersectionPrimary => {
                primary
                    && (nullabilities.peek().is_none() || nullabilities.any(|nullable| nullable))
            }
            SetOp::IntersectionMultiset | SetOp::IntersectionMultisetAll => {
                primary && nullabilities.all(|nullable| nullable)
            }
            SetOp::UnionDistinct | SetOp::UnionAll => {
                primary || nullabilities.any(|nullable| nullable)
            }
        }
    });
    y.set_schema(schema);

    // Check set operation.
    let op = proto_required_enum_field!(x, y, op, SetOp)
        .1
        .unwrap_or_default();
    let op = match (op, in_types.len() > 2) {
        (SetOp::Unspecified, _) => Operation::Invalid,
        (SetOp::MinusPrimary, true) => Operation::SubtractByUnion,
        (SetOp::MinusPrimary, false) => Operation::Subtract,
        (SetOp::MinusMultiset, true) => Operation::SubtractByIntersection,
        (SetOp::MinusMultiset, false) => Operation::Subtract,
        (SetOp::IntersectionPrimary, true) => Operation::IntersectWithUnion,
        (SetOp::IntersectionPrimary, false) => Operation::Intersect,
        (SetOp::IntersectionMultiset, _) => Operation::Intersect,
        (SetOp::UnionDistinct, _) => Operation::Union,
        (SetOp::UnionAll, _) => Operation::Merge,
        (SetOp::MinusPrimaryAll, _) | (SetOp::IntersectionMultisetAll, _) => {
            diagnostic!(
                y,
                Warning,
                NotYetImplemented,
                "Set variant {:?} not yet supported",
                op
            );

            handle_rel_common!(x, y);
            handle_advanced_extension!(x, y);
            return Ok(());
        }
    };

    // Describe the relation.
    match op {
        Operation::Invalid => {
            describe!(y, Relation, "Invalid set operation");
        }
        Operation::Subtract => {
            describe!(y, Relation, "Set subtraction");
            summary!(
                y,
                "Yields all rows from the first dataset that do not exist \
                in the second dataset."
            );
        }
        Operation::SubtractByUnion => {
            describe!(y, Relation, "Set subtract by union");
            summary!(
                y,
                "Yields all rows from the first dataset that do not exist \
                in any of the other datasets."
            );
        }
        Operation::SubtractByIntersection => {
            describe!(y, Relation, "Set subtract by intersection");
            summary!(
                y,
                "Yields all rows from the first dataset that do not exist in \
                all of the other datasets."
            );
        }
        Operation::Intersect => {
            describe!(y, Relation, "Set intersection");
            summary!(
                y,
                "Yields all rows from the first dataset that exist in all \
                datasets."
            );
        }
        Operation::IntersectWithUnion => {
            describe!(y, Relation, "Set intersect with union");
            summary!(
                y,
                "Yields all rows from the first dataset that exist in any of \
                the other datasets."
            );
        }
        Operation::Union => {
            describe!(y, Relation, "Set union");
            summary!(
                y,
                "Yields all rows that exist in any dataset, removing duplicates."
            );
        }
        Operation::Merge => {
            describe!(y, Relation, "Merge");
            summary!(y, "Yields all rows from all incoming datasets.");
        }
    };

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
