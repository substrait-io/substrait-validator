// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for fetch relations.
//!
//! The fetch operation eliminates records outside a desired window. Typically
//! corresponds to a fetch/offset SQL clause.
//!
//! See <https://substrait.io/relations/logical_relations/#fetch-operation>

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;
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

    // Parse offset and count from the new oneof fields.
    // Extract offset value (default to 0 if not set)
    proto_field!(x, y, offset_mode); // TODO add the check for negative values
    let offset = match &x.offset_mode {
        Some(OffsetMode::Offset(val)) => {
            if *val < 0 {
                diagnostic!(y, Error, IllegalValue, "offsets cannot be negative");
            }
            *val
        }
        Some(OffsetMode::OffsetExpr(_)) => {
            // For now, we can't evaluate expressions, so we'll just note it
            diagnostic!(
                y,
                Warning,
                NotYetImplemented,
                "offset_expr evaluation not yet implemented"
            );
            0
        }
        None => 0,
    };

    // Extract count value (default to 0 if not set)
    proto_field!(x, y, count_mode); // TODO add the check for negative values
    let count = match &x.count_mode {
        Some(CountMode::Count(val)) => {
            if *val < 0 {
                diagnostic!(y, Error, IllegalValue, "count cannot be negative");
            }
            *val
        }
        Some(CountMode::CountExpr(_)) => {
            // For now, we can't evaluate expressions, so we'll just note it
            diagnostic!(
                y,
                Warning,
                NotYetImplemented,
                "count_expr evaluation not yet implemented"
            );
            0
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
