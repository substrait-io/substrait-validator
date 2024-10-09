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

    // Determine whether the join can null the left and/or right side, and
    // whether the right side is returned at all.
    let (left_nullable, right_nullable) = match join_type {
        JoinType::Unspecified => (false, Some(false)),
        JoinType::Inner => (false, Some(false)),
        JoinType::Outer => (true, Some(true)),
        JoinType::Left => (false, Some(true)),
        JoinType::Right => (true, Some(false)),
        JoinType::LeftSemi => (false, None),
        JoinType::LeftAnti => (false, None),
        JoinType::LeftSingle => (false, Some(true)),
        // TODO: Implement the following join types. I don't understand this
        // code or these types well enough to do so.
        JoinType::RightSemi
        | JoinType::RightAnti
        | JoinType::RightSingle
        | JoinType::LeftMark
        | JoinType::RightMark => {
            diagnostic!(y, Warning, NotYetImplemented, "{:?} joins", join_type);
            handle_rel_common!(x, y);

            // Handle the advanced extension field.
            handle_advanced_extension!(x, y);

            // Keep going; this node is not correct, but we can continue to validate.
            return Ok(());
        }
    };

    // Derive final schema.
    if let (Some(left_fields), Some(right_fields)) = (left.unwrap_struct(), right.unwrap_struct()) {
        let mut fields = Vec::with_capacity(left_fields.len() + right_fields.len());
        if left_nullable {
            fields.extend(left_fields.into_iter().map(|x| x.make_nullable()))
        } else {
            fields.extend(left_fields)
        }
        if let Some(right_nullable) = right_nullable {
            if right_nullable {
                fields.extend(right_fields.into_iter().map(|x| x.make_nullable()))
            } else {
                fields.extend(right_fields)
            }
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
        // TODO: Implement the following join types. I don't understand these
        // types well enough to do so.
        (JoinType::RightSemi, _) => todo!(),
        (JoinType::RightAnti, _) => todo!(),
        (JoinType::RightSingle, _) => todo!(),
        (JoinType::LeftMark, _) => todo!(),
        (JoinType::RightMark, _) => todo!(),
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
                                    multiple right rows, this may return the first pair encountered \
                                    or throw an error."
                        .to_string()
                }
                JoinType::RightSingle => {
                    "Returns a row for each row from the right input, concatenating \
                                    it with the row from the left input for which the join \
                                    expression yields true. If the expression never yields true for \
                                    a right input, the fields corresponding to the left input are \
                                    set to null. If the expression yields true for a right row and \
                                    multiple left rows, this may return the first pair encountered \
                                    or throw an error."
                        .to_string()
                }
                JoinType::LeftMark => "Returns one record for each record from the left input. \
                                    Appends one additional “mark” column to the output of the join. \
                                    The new column will be listed after all columns from both sides \
                                    and will be of type nullable boolean. If there is at least one \
                                    join partner in the right input where the join condition evaluates \
                                    to true then the mark column will be set to true. Otherwise, if \
                                    there is at least one join partner in the right input where the \
                                    join condition evaluates to NULL then the mark column will be set \
                                    to NULL. Otherwise the mark column will be set to false.".to_string(),
                JoinType::RightMark => "Returns records from the right input. Appends one additional \
                                    “mark” column to the output of the join. The new column will be \
                                    listed after all columns from both sides and will be of type \
                                    nullable boolean. If there is at least one join partner in the \
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
