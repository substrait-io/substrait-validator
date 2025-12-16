// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for fetch relations.
//!
//! The fetch operation eliminates records outside a desired window. Typically
//! corresponds to a fetch/offset SQL clause.
//!
//! See <https://substrait.io/relations/logical_relations/#fetch-operation>

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::parse::context;
use crate::parse::expressions;
use crate::util;

/// Parse fetch relation.
pub fn parse_fetch_rel(
    x: &substrait::FetchRel,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    use substrait::fetch_rel::CountMode;
    use substrait::fetch_rel::OffsetMode;

    // Parse input.
    let in_type = handle_rel_input!(x, y);

    // Filters pass through their input schema unchanged.
    y.set_schema(in_type);

    // Parse offset and count.
    // proto_primitive_field!(x, y, offset, |x, y| {
    //     if *x < 0 {
    //         diagnostic!(y, Error, IllegalValue, "offsets cannot be negative");
    //     }
    //     Ok(())
    // });
    // proto_primitive_field!(x, y, count, |x, y| {
    //     if *x < 0 {
    //         diagnostic!(y, Error, IllegalValue, "count cannot be negative");
    //     }
    //     Ok(())
    // });

    proto_field!(x, y, offset_mode); // TODO add the check for negative values
    let offset = match &x.offset_mode {
        Some(OffsetMode::Offset(n)) => *n,
        Some(OffsetMode::OffsetExpr(expr)) => {
            let ex: expressions::Expression = expressions::parse_expression(expr, y)?;
            match ex {
                expressions::Expression::Literal(literal) => match literal.data_type().class() {
                    data::Class::Simple(data::class::Simple::I64) => todo!(),
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }
        None => 0,
    };

    proto_field!(x, y, count_mode); // TODO add the check for negative values
    let count = match &x.count_mode {
        Some(CountMode::Count(n)) => *n,
        Some(CountMode::CountExpr(expr)) => {
            let ex: expressions::Expression = expressions::parse_expression(expr, y)?;
            match ex {
                expressions::Expression::Literal(literal) => match literal.data_type().class() {
                    data::Class::Simple(data::class::Simple::I64) => todo!(),
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }
        None => 0,
    };

    // Describe the relation.
    if count == 1 {
        describe!(
            y,
            Relation,
            "Propagate only the {} row",
            (offset + 1)
                .try_into()
                .map(util::string::describe_nth)
                .unwrap_or_else(|_| String::from("?"))
        );
    } else if count > 1 {
        if offset > 1 {
            describe!(
                y,
                Relation,
                "Propagate only {} rows, starting from the {}",
                count,
                (offset + 1)
                    .try_into()
                    .map(util::string::describe_nth)
                    .unwrap_or_else(|_| String::from("?"))
            );
        } else {
            describe!(y, Relation, "Propagate only the first {} rows", count);
        }
    } else if offset == 0 {
        describe!(y, Relation, "Fetch all rows");
    } else if offset == 1 {
        describe!(y, Relation, "Discard the first row");
    } else if offset > 1 {
        describe!(y, Relation, "Discard the first {} rows", offset);
    } else {
        describe!(y, Relation, "Invalid fetch relation");
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
