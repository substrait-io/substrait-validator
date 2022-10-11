// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for read relations.
//!
//! The read operator is an operator that produces one output. A simple example
//! would be the reading of a Parquet file.
//!
//! See <https://substrait.io/relations/logical_relations/#read-operator>

use std::sync::Arc;

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::expressions::literals;
use crate::parse::expressions::references::mask;
use crate::parse::extensions;
use crate::parse::types;
use crate::util;

/// Information about a data source.
struct SourceInfo {
    /// Short description of the data source, used in the brief of the read
    /// relation.
    pub name: String,

    /// The schema of the data, if not context-sensitive.
    pub data_type: Option<data::Type>,
}

/// Parse virtual table.
fn parse_virtual_table(
    x: &substrait::read_rel::VirtualTable,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    let mut data_type: data::Type = Arc::default();

    // Parse rows, ensuring that they all have the same type.
    proto_repeated_field!(x, y, values, |x, y| {
        let result = literals::parse_struct(x, y, false, None);
        data_type = types::assert_equal(
            y,
            &y.data_type(),
            &data_type,
            "virtual table rows must have the same type",
        );
        result
    });

    // Describe the node.
    describe!(y, Misc, "Virtual table");
    Ok(SourceInfo {
        name: String::from("virtual table"),
        data_type: Some(data_type),
    })
}

/// Parse file entry. Returns whether this matches multiple files.
fn parse_path_type(
    x: &substrait::read_rel::local_files::file_or_files::PathType,
    y: &mut context::Context,
) -> diagnostic::Result<bool> {
    // FIXME: I'm not sure these paths should even be URIs. These are supposed
    // to be local files after all, so shouldn't they just be paths? But they
    // really shouldn't be called URIs if they're not going to conform to the
    // standard governing them, and if they're paths, there should still be
    // some specification about what kind of paths they can be (POSIX? Windows
    // with slashes? UNC? etc).
    //
    // Note that the diagnostics for this have their own code, so if a user
    // disagrees with the syntax they can just downgrade these warnings to
    // infos.
    use substrait::read_rel::local_files::file_or_files::PathType;
    match x {
        PathType::UriPath(x) => {
            if let Err(e) = util::string::check_uri(x) {
                diagnostic!(y, Error, e);
            }
            Ok(false)
        }
        PathType::UriPathGlob(x) => {
            if let Err(e) = util::string::check_uri_glob(x) {
                diagnostic!(y, Error, e);
            }
            Ok(true)
        }
        PathType::UriFile(x) => {
            if let Err(e) = util::string::check_uri(x) {
                diagnostic!(y, Error, e);
            }
            Ok(false)
        }
        PathType::UriFolder(x) => {
            if let Err(e) = util::string::check_uri(x) {
                diagnostic!(y, Error, e);
            }
            Ok(true)
        }
    }
}

/// Parse file format, yielding a string description of the format if known.
fn parse_file_format(
    x: &substrait::read_rel::local_files::file_or_files::FileFormat,
    y: &mut context::Context,
) -> diagnostic::Result<Option<String>> {
    Ok(match x {
        substrait::read_rel::local_files::file_or_files::FileFormat::Parquet(_) => {
            Some(String::from("Parquet"))
        }
        substrait::read_rel::local_files::file_or_files::FileFormat::Arrow(_) => {
            Some(String::from("Arrow IPC"))
        }
        substrait::read_rel::local_files::file_or_files::FileFormat::Orc(_) => {
            Some(String::from("Orc"))
        }
        substrait::read_rel::local_files::file_or_files::FileFormat::Dwrf(_) => {
            Some(String::from("Dwrf"))
        }
        substrait::read_rel::local_files::file_or_files::FileFormat::Extension(x) => {
            extensions::advanced::parse_functional_any(x, y)?;
            None
        }
    })
}

/// Parse file entry.
fn parse_file_or_files(
    x: &substrait::read_rel::local_files::FileOrFiles,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // Parse path.
    let multiple = proto_required_field!(x, y, path_type, parse_path_type)
        .1
        .unwrap_or_default();

    // Parse read configuration.
    let format = proto_required_field!(x, y, file_format, parse_file_format)
        .1
        .flatten();
    proto_primitive_field!(x, y, partition_index);
    proto_primitive_field!(x, y, start);
    proto_primitive_field!(x, y, length);

    // Having nonzero file offsets makes no sense when this entry refers to
    // multiple files.
    if multiple && (x.start > 0 || x.length > 0) {
        diagnostic!(
            y,
            Error,
            IllegalValue,
            "file offsets are not allowed in conjunction with multiple files"
        );
    }

    // Describe the node.
    if multiple {
        describe!(y, Misc, "Multiple files");
    } else {
        describe!(y, Misc, "Single file");
    }
    summary!(y, "Read");
    if x.partition_index != 0 {
        summary!(y, "partition {}", x.partition_index);
    }
    summary!(y, "from");
    if multiple {
        summary!(y, "multiple");
    } else {
        if x.start > 0 {
            if x.length > 0 {
                summary!(y, "byte offset {} to {} of", x.start, x.start + x.length);
            } else {
                summary!(y, "byte offset {} to the end of", x.start);
            }
        } else if x.length > 0 {
            summary!(y, "the first {} byte(s) of", x.length);
        }
        summary!(y, "a single");
    }
    if let Some(format) = format {
        summary!(y, "{}", format);
    }
    if multiple {
        summary!(y, "files");
    } else {
        summary!(y, "file");
    }

    Ok(())
}

/// Parse local files.
fn parse_local_files(
    x: &substrait::read_rel::LocalFiles,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    // Parse fields.
    proto_required_repeated_field!(x, y, items, parse_file_or_files, |_, _, _, _, _| (),);
    proto_field!(
        x,
        y,
        advanced_extension,
        extensions::advanced::parse_advanced_extension
    );

    // Describe the node.
    describe!(y, Misc, "Table from file(s)");
    Ok(SourceInfo {
        name: String::from("local files"),
        data_type: None,
    })
}

/// Parse named table.
fn parse_named_table(
    x: &substrait::read_rel::NamedTable,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    // Parse fields.
    proto_required_repeated_field!(x, y, names);
    proto_field!(
        x,
        y,
        advanced_extension,
        extensions::advanced::parse_advanced_extension
    );

    // Determine and check consistency of the table name.
    let name = if x.names.is_empty() {
        String::from("?")
    } else {
        if x.names.len() > 1 {
            // FIXME: what does this mean?
            diagnostic!(
                y,
                Warning,
                NotYetImplemented,
                "named tables with multiple names"
            );
        }
        util::string::as_ident_or_string(x.names.first().unwrap())
    };

    // Describe the node.
    describe!(
        y,
        Misc,
        "Named table {}",
        util::string::as_ident_or_string(&name)
    );
    Ok(SourceInfo {
        name,
        data_type: None,
    })
}

/// Parse extension table.
fn parse_extension_table(
    x: &substrait::read_rel::ExtensionTable,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    proto_required_field!(x, y, detail, extensions::advanced::parse_functional_any);

    // Describe the node.
    describe!(
        y,
        Misc,
        "{} extension",
        x.detail
            .as_ref()
            .map(|x| x.type_url.clone())
            .unwrap_or_else(|| String::from("Unknown"))
    );
    Ok(SourceInfo {
        name: x
            .detail
            .as_ref()
            .map(|x| x.type_url.to_string())
            .unwrap_or_else(|| String::from("extension")),
        data_type: None,
    })
}

/// Parse read type.
fn parse_read_type(
    x: &substrait::read_rel::ReadType,
    y: &mut context::Context,
) -> diagnostic::Result<SourceInfo> {
    match x {
        substrait::read_rel::ReadType::VirtualTable(x) => parse_virtual_table(x, y),
        substrait::read_rel::ReadType::LocalFiles(x) => parse_local_files(x, y),
        substrait::read_rel::ReadType::NamedTable(x) => parse_named_table(x, y),
        substrait::read_rel::ReadType::ExtensionTable(x) => parse_extension_table(x, y),
    }
}

/// Parse read relation.
pub fn parse_read_rel(x: &substrait::ReadRel, y: &mut context::Context) -> diagnostic::Result<()> {
    // Handle read type field.
    let source = proto_required_field!(x, y, read_type, parse_read_type)
        .1
        .unwrap_or(SourceInfo {
            name: String::from("unknown source"),
            data_type: None,
        });

    // Handle schema field.
    let schema = proto_required_field!(x, y, base_schema, types::parse_named_struct)
        .0
        .data_type
        .clone();

    // If both data_type and schema are known, verify that they are the same.
    let mut schema = match (source.data_type, schema) {
        (Some(data_type), Some(schema)) => {
            types::assert_equal(y, &schema, &data_type, "data differs from schema")
        }
        (Some(data_type), None) => data_type,
        (None, Some(schema)) => schema,
        (None, None) => Arc::default(),
    };

    // The outer struct of a schema should not be nullable.
    if !schema.is_unresolved() && schema.nullable() {
        diagnostic!(
            y,
            Error,
            TypeMismatchedNullability,
            "the outer struct representing a schema must not be nullable"
        );
    }

    // Set the schema to the merged data type.
    y.set_schema(schema.clone());

    // Handle filter.
    let filter = proto_boxed_field!(x, y, filter, expressions::parse_predicate);

    // Handle projection.
    if x.projection.is_some() {
        schema =
            proto_required_field!(x, y, projection, mask::parse_mask_expression, &schema, true)
                .0
                .data_type();
        y.set_schema(schema.clone());
    }

    // Describe the relation.
    match (x.filter.is_some(), x.projection.is_some()) {
        (false, false) => describe!(y, Relation, "Read from {}", source.name),
        (false, true) => describe!(y, Relation, "Partial read from {}", source.name),
        (true, false) => describe!(y, Relation, "Filtered read from {}", source.name),
        (true, true) => describe!(y, Relation, "Filtered partial read from {}", source.name),
    }

    // Add filter summary.
    if let (Some(filter_node), Some(filter_expr)) = filter {
        let nullable = filter_node.data_type().nullable();
        summary!(
            y,
            "This relation discards all rows for which the expression {} yields {}.",
            filter_expr,
            if nullable { "false or null" } else { "false" }
        );
    }

    // Handle the common field.
    handle_rel_common!(x, y);

    // Handle the advanced extension field.
    handle_advanced_extension!(x, y);

    Ok(())
}
