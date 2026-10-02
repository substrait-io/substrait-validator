// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for join relations.
//!
//! The join operation will combine two separate inputs into a single output,
//! based on a join expression. A common subtype of joins is a equality join
//! where the join expression is constrained to a list of equality (or
//! equality + null equality) conditions between the two inputs of the join.
//!
//! See <https://substrait.io/relations/logical_relations/#join-operation>

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::comment;
use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::parse::context;
use crate::parse::expressions;

/// Parse join relation.
pub fn parse_join_rel(x: &substrait::JoinRel, y: &mut context::Context) -> diagnostic::Result<()> {
    use substrait::join_rel::JoinType;

    // Parse input.
    let left = handle_rel_input!(x, y, left);
    let right = handle_rel_input!(x, y, right);

    // Derive schema with which the join expression is evaluated.
    if let (Some(mut fields), Some(additional_fields)) =
        (left.unwrap_struct(), right.unwrap_struct())
    {
        fields.extend(additional_fields);
        let schema = data::new_struct(fields, false);
        y.set_schema(schema);
    } else {
        y.set_schema(Arc::default());
    }

    // Parse join expression.
    let (join_expression_node, opt_join_expression) =
        proto_boxed_required_field!(x, y, expression, expressions::parse_predicate);
    let join_expression = opt_join_expression.unwrap_or_default();

    // Parse join type.
    let join_type = proto_required_enum_field!(x, y, r#type, JoinType)
        .1
        .unwrap_or_default();

    // Determine which inputs the join returns, whether it can null them, and
    // whether it appends a mark column. A semi or anti join returns one side
    // only, and a mark join returns one side followed by a nullable boolean.
    let (left_nullable, right_nullable, mark) = match join_type {
        JoinType::Unspecified => (Some(false), Some(false), false),
        JoinType::Inner => (Some(false), Some(false), false),
        JoinType::Outer => (Some(true), Some(true), false),
        JoinType::Left => (Some(false), Some(true), false),
        JoinType::Right => (Some(true), Some(false), false),
        JoinType::LeftSemi => (Some(false), None, false),
        JoinType::RightSemi => (None, Some(false), false),
        JoinType::LeftAnti => (Some(false), None, false),
        JoinType::RightAnti => (None, Some(false), false),
        JoinType::LeftSingle => (Some(false), Some(true), false),
        JoinType::RightSingle => (Some(true), Some(false), false),
        JoinType::LeftMark => (Some(false), None, true),
        JoinType::RightMark => (None, Some(false), true),
    };

    // Derive final schema.
    if let (Some(left_fields), Some(right_fields)) = (left.unwrap_struct(), right.unwrap_struct()) {
        let mut fields = Vec::with_capacity(left_fields.len() + right_fields.len() + 1);
        for (side, nullable) in [(left_fields, left_nullable), (right_fields, right_nullable)] {
            match nullable {
                Some(true) => fields.extend(side.into_iter().map(|x| x.make_nullable())),
                Some(false) => fields.extend(side),
                None => {}
            }
        }
        if mark {
            fields.push(data::new_predicate_with_nullability(true));
        }
        let schema = data::new_struct(fields, false);
        y.set_schema(schema);
    } else {
        y.set_schema(Arc::default());
    }

    // Handle optional post-join filter.
    let filter_expression =
        proto_boxed_field!(x, y, post_join_filter, expressions::parse_predicate);

    // Describe the relation.
    let prefix = match (join_type, x.post_join_filter.is_some()) {
        (JoinType::Unspecified, _) => "Unknown",
        (JoinType::Inner, true) => "Filtered inner",
        (JoinType::Inner, false) => "Inner",
        (JoinType::Outer, true) => "Filtered outer",
        (JoinType::Outer, false) => "Outer",
        (JoinType::Left, true) => "Filtered left",
        (JoinType::Left, false) => "Left",
        (JoinType::Right, true) => "Filtered right",
        (JoinType::Right, false) => "Right",
        (JoinType::LeftSemi, true) => "Filtered left semi",
        (JoinType::LeftSemi, false) => "Left semi",
        (JoinType::LeftAnti, true) => "Filtered left anti",
        (JoinType::LeftAnti, false) => "Left anti",
        (JoinType::LeftSingle, true) => "Filtered left single",
        (JoinType::LeftSingle, false) => "Left single",
        (JoinType::RightSemi, true) => "Filtered right semi",
        (JoinType::RightSemi, false) => "Right semi",
        (JoinType::RightAnti, true) => "Filtered right anti",
        (JoinType::RightAnti, false) => "Right anti",
        (JoinType::RightSingle, true) => "Filtered right single",
        (JoinType::RightSingle, false) => "Right single",
        (JoinType::LeftMark, true) => "Filtered left mark",
        (JoinType::LeftMark, false) => "Left mark",
        (JoinType::RightMark, true) => "Filtered right mark",
        (JoinType::RightMark, false) => "Right mark",
    };
    describe!(y, Relation, "{prefix} join by {join_expression}");
    summary!(y, "{prefix} join by {join_expression:#}.");
    let nullable = if join_expression_node.data_type().nullable() {
        "false or null"
    } else {
        "false"
    };
    y.push_summary(
        comment::Comment::new().nl().plain(match join_type {
            JoinType::Unspecified => "".to_string(),
            JoinType::Inner => format!(
                "Returns rows combining the row from the left and right \
                input for each pair where the join expression yields true, \
                discarding rows where the join expression yields {}.",
                nullable
            ),
            JoinType::Outer => format!(
                "Returns rows combining the row from the left and right \
                input for each pair where the join expression yields true, \
                discarding rows where the join expression yields {}. \
                If the join expression never yields true for any left or \
                right row, this returns a row anyway, with the fields \
                corresponding to the other input set to null.",
                nullable
            ),
            JoinType::Left => format!(
                "Returns rows combining the row from the left and right \
                input for each pair where the join expression yields true, \
                discarding rows where the join expression yields {}. \
                If the join expression never yields true for a row from the \
                left, this returns a row anyway, with the fields corresponding \
                to the right input set to null.",
                nullable
            ),
            JoinType::Right => format!(
                "Returns rows combining the row from the left and right \
                input for each pair where the join expression yields true, \
                discarding rows where the join expression yields {}. \
                If the join expression never yields true for a row from the \
                right, this returns a row anyway, with the fields corresponding \
                to the left input set to null.",
                nullable
            ),
            JoinType::LeftSemi => "Filters rows from the left input, propagating a row only if \
                              the join expression yields true for that row combined with \
                              any row from the right input."
                .to_string(),
            JoinType::RightSemi => "Filters rows from the right input, propagating a row only if \
                                  the join expression yields true for that row combined with \
                                  any row from the left input."
                .to_string(),
            JoinType::LeftAnti => "Filters rows from the left input, propagating a row only if \
                                the join expression does not yield true for that row combined \
                                with any row from the right input."
                .to_string(),
            JoinType::RightAnti => "Filters rows from the right input, propagating a row only if \
                                the join expression does not yield true for that row combined \
                                with any row from the left input."
                .to_string(),
                JoinType::LeftSingle => {
                    "Returns a row for each row from the left input, concatenating \
                                    it with the row from the right input for which the join \
                                    expression yields true. If the expression never yields true for \
                                    a left input, the fields corresponding to the right input are \
                                    set to null. If the expression yields true for a left row and \
                                    multiple right rows, it is a runtime error."
                        .to_string()
                }
                JoinType::RightSingle => {
                    "Returns a row for each row from the right input, concatenating \
                                    it with the row from the left input for which the join \
                                    expression yields true. If the expression never yields true for \
                                    a right input, the fields corresponding to the left input are \
                                    set to null. If the expression yields true for a right row and \
                                    multiple left rows, it is a runtime error."
                        .to_string()
                }
                JoinType::LeftMark => "Returns one record for each record from the left input. \
                                    Appends one additional “mark” column to the output of the join. \
                                    The new column will be listed after all columns from the left \
                                    side and will be of type nullable boolean. If there is at least \
                                    one join partner in the right input where the join condition evaluates \
                                    to true then the mark column will be set to true. Otherwise, if \
                                    there is at least one join partner in the right input where the \
                                    join condition evaluates to NULL then the mark column will be set \
                                    to NULL. Otherwise the mark column will be set to false.".to_string(),
                JoinType::RightMark => "Returns one record for each record from the right input. \
                                    Appends one additional “mark” column to the output of the join. The new column will be \
                                    listed after all columns from the right side and will be of \
                                    type nullable boolean. If there is at least one join partner in the \
                                    left input where the join condition evaluates to true then the \
                                    mark column will be set to true. Otherwise, if there is at least \
                                    one join partner in the left input where the join condition \
                                    evaluates to NULL then the mark column will be set to NULL. \
                                    Otherwise the mark column will be set to false.".to_string(),
        }),
    );

    if let (Some(node), Some(filter_expression)) = filter_expression {
        let nullable = node.data_type().nullable();
        y.push_summary(comment::Comment::new().nl().plain(format!(
            "The result is filtered by the expression {filter_expression:#}, \
            discarding all rows for which the filter expression yields {}.",
            if nullable { "false or null" } else { "false" }
        )));
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
