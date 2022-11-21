// SPDX-License-Identifier: Apache-2.0

//! Module providing toplevel parse/validation functions for plans.

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::parse::context;
use crate::parse::extensions;
use crate::parse::relations;

// Parse a relation root, i.e. a toplevel relation that includes field name
// information.
fn parse_rel_root(x: &substrait::RelRoot, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse the fields.
    let schema = proto_required_field!(x, y, input, relations::parse_rel)
        .0
        .data_type();
    proto_repeated_field!(x, y, names);

    // Relate the names to the schema.
    let schema = schema
        .apply_field_names(&x.names)
        .map_err(|x| diagnostic!(y, Error, x))
        .unwrap_or_default();
    y.set_schema(schema);

    // Describe the node.
    describe!(y, Misc, "Named relation root");
    summary!(y, "Attaches names to result schema");
    Ok(())
}

// Parse a relation type.
fn parse_rel_type(
    x: &substrait::plan_rel::RelType,
    y: &mut context::Context,
) -> diagnostic::Result<data::Type> {
    match x {
        substrait::plan_rel::RelType::Rel(x) => {
            relations::parse_rel(x, y)?;
            Ok(y.data_type().strip_field_names())
        }
        substrait::plan_rel::RelType::Root(x) => {
            parse_rel_root(x, y)?;
            Ok(y.data_type())
        }
    }
}

/// Parse a PlanRel node.
fn parse_plan_rel(x: &substrait::PlanRel, y: &mut context::Context) -> diagnostic::Result<()> {
    let data_type = y.enter_relation_root(|y| {
        proto_required_field!(x, y, rel_type, parse_rel_type)
            .1
            .unwrap_or_default()
    });

    // Describe the node.
    y.set_data_type(data_type);
    describe!(y, Misc, "Relation root");
    Ok(())
}

/// Parse a git hash string.
fn parse_git_hash(x: &String, y: &mut context::Context) -> diagnostic::Result<()> {
    if !x.is_empty() {
        static GIT_HASH_RE: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new("[0-9a-f]{40}").unwrap());
        if !GIT_HASH_RE.is_match(x) {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "git hash must be a 40-character lowercase hexadecimal string \
                if specified."
            );
        }
        diagnostic!(
            y,
            Warning,
            Versioning,
            "a git hash was specified for the Substrait version, indicating \
            use of nonstandard features. The validation result may not be \
            accurate."
        );
    }
    Ok(())
}

/// Parse a producer identification string.
fn parse_producer_id(x: &String, y: &mut context::Context) -> diagnostic::Result<()> {
    if x.is_empty() {
        diagnostic!(
            y,
            Info,
            Versioning,
            "producer identifier is missing. While not strictly necessary, \
            especially for hand-written plans, it is strongly recommended to \
            include one. This allows consumers to work around unforeseen \
            problems specific to your producer."
        );
    }
    Ok(())
}

/// Parse a version node.
fn parse_version(x: &substrait::Version, y: &mut context::Context) -> diagnostic::Result<()> {
    // Parse the version information.
    let major = proto_primitive_field!(x, y, major_number)
        .1
        .unwrap_or_default() as u64;
    let minor = proto_primitive_field!(x, y, minor_number)
        .1
        .unwrap_or_default() as u64;
    let patch = proto_primitive_field!(x, y, patch_number)
        .1
        .unwrap_or_default() as u64;
    let version = semver::Version::new(major, minor, patch);
    if version == semver::Version::new(0, 0, 0) {
        diagnostic!(y, Error, Versioning, "invalid plan version (0.0.0)");
    } else if !crate::substrait_version_req_loose().matches(&version) {
        diagnostic!(
            y,
            Warning,
            Versioning,
            "plan version ({}) is not compatible with the Substrait \
            version that this version of the validator validates ({}).",
            version,
            crate::substrait_version()
        );
    } else if !crate::substrait_version_req().matches(&version) {
        diagnostic!(
            y,
            Warning,
            Versioning,
            "cannot automatically determine whether plan version ({}) is \
            compatible with the Substrait version that this version of \
            the validator validates ({}). Please check the release notes \
            between these versions, or install the correct version of the \
            validator. See also \
            https://github.com/substrait-io/substrait/pull/210#discussion_r881965837",
            version,
            crate::substrait_version()
        );
    };

    // Check hash.
    proto_primitive_field!(x, y, git_hash, parse_git_hash);

    // Check producer information.
    proto_primitive_field!(x, y, producer, parse_producer_id);

    Ok(())
}

/// Report the "validator is experimental" diagnostic.
fn mark_experimental(y: &mut context::Context) {
    diagnostic!(
        y,
        Info,
        Experimental,
        "this version of the validator is EXPERIMENTAL. Please report issues \
        via https://github.com/substrait-io/substrait-validator/issues/new"
    );
}

/// Toplevel parse function for a plan.
pub fn parse_plan(x: &substrait::Plan, y: &mut context::Context) -> diagnostic::Result<()> {
    mark_experimental(y);

    // Parse the version.
    proto_required_field!(x, y, version, parse_version);

    // Handle extensions first, because we'll need their declarations to
    // correctly interpret the relations.
    extensions::parse_plan(x, y);

    // Handle the relations.
    let num_relations = proto_repeated_field!(x, y, relations, parse_plan_rel)
        .0
        .len();
    if num_relations == 0 {
        diagnostic!(
            y,
            Error,
            RelationRootMissing,
            "a plan must have at least one relation"
        );
    }

    // Generate an Info diagnostic for every extension definition that wasn't
    // used at any point, and can thus be safely removed.
    extensions::check_unused_definitions(y);

    Ok(())
}

/// Toplevel parse function for a plan.
pub fn parse_plan_version(
    x: &substrait::PlanVersion,
    y: &mut context::Context,
    e: diagnostic::Cause,
) -> diagnostic::Result<()> {
    mark_experimental(y);

    // Push the diagnostic that the caller got while parsing as a complete Plan
    // before.
    diagnostic!(y, Error, e);

    // Parse the version.
    proto_required_field!(x, y, version, parse_version);

    Ok(())
}
