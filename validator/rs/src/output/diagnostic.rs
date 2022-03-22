// SPDX-License-Identifier: Apache-2.0

//! Module for diagnostic message types.
//!
//! Since diagnostic messages are rather important for a validator (after all,
//! getting a diagnostic message is hardly an exceptional case), they have
//! quite a bit of metadata attached to them. Ultimately, the diagnostic
//! messages attached to the tree ([`Diagnostic`]) have the following
//! parameters:
//!
//!  - cause.message: an enumeration of various types of error messages, in
//!    the usual Rust way. Messages generated by this crate are usually
//!    untyped (they just use String), but error information from other
//!    crates is retained as much as possible.
//!  - cause.classification: an enumeration of various bits of the validation
//!    process where diagnostics might occur. Each [`Classification`] enum
//!    variant can be converted to a unique number, known as the diagnostic
//!    code, which the user of the crate may use to easily programmatically
//!    determine what caused a diagnostic in a language-agnostic way. The user
//!    may also configure the validator in advance to promote or reduce the
//!    severity of diagnostics, indexed by their code. The codes are
//!    furthermore organized into groups, with up to 999 classes per group: the
//!    thousands digit and up is the group identifier, and the less-significant
//!    digits form the sub-code. Sub-code 0 is reserved to refer to the group
//!    as a whole.
//!  - original_level: the error [`Level`] that the validation code assigned to
//!    the message. This can be `Error`, `Warning`, or `Info`, which correspond
//!    directly to "this is definitely wrong," "this may or may not be wrong,"
//!    and "this conforms to the Substrait specification, but it's worth noting
//!    anyway" respectively.
//!  - adjusted_level: the error [`Level`] after configuration-based adjustment.
//!    This level is what's used by the high-level APIs to determine the
//!    validity of a plan. Thus, a user can choose to ignore a particular error
//!    if their consumer implementation can deal with it anyway, or they can
//!    assert whether a particular type of warning is actually an error or not.
//!  - path: a path into the substrait.Plan message. This is *usually* just a
//!    copy of the path to the node that was being validated when the
//!    diagnostic was created, but in some cases diagnostics may be placed in a
//!    parent node (for instance to refer to a node that should exist but
//!    doesn't), or refer to a different location altogether (for instance to
//!    point the user to the previous definition in a note following a
//!    duplicate definition error).

use crate::output::path;
use num_traits::cast::FromPrimitive;
use std::sync::Arc;
use strum::EnumProperty;

/// Owned variant of jsonschema::error::ValidationError<'a>. Instead of a
/// reference to the YAML tree node that caused the error, this just contains
/// the formatted error message. The validation error kind and paths are
/// however retained.
#[derive(Debug, thiserror::Error)]
pub struct JsonSchemaValidationError {
    pub message: String,
    pub kind: jsonschema::error::ValidationErrorKind,
    pub instance_path: jsonschema::paths::JSONPointer,
    pub schema_path: jsonschema::paths::JSONPointer,
}

impl std::fmt::Display for JsonSchemaValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl From<jsonschema::error::ValidationError<'_>> for JsonSchemaValidationError {
    fn from(v: jsonschema::error::ValidationError) -> Self {
        JsonSchemaValidationError {
            message: v.to_string(),
            kind: v.kind,
            instance_path: v.instance_path,
            schema_path: v.schema_path,
        }
    }
}

/// Enumeration for error message data we might encounter.
#[derive(Debug, thiserror::Error)]
pub enum Message {
    #[error("{0}")]
    Untyped(String),

    #[error("{0}")]
    ProstDecodeError(#[from] prost::DecodeError),

    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    UtfError(#[from] std::str::Utf8Error),

    #[error("{0}")]
    YamlScanError(#[from] yaml_rust::ScanError),

    #[error("{0}")]
    JsonSchemaValidationError(#[from] JsonSchemaValidationError),
}

impl From<&str> for Message {
    fn from(s: &str) -> Self {
        Message::Untyped(s.to_string())
    }
}

impl From<String> for Message {
    fn from(s: String) -> Self {
        Message::Untyped(s)
    }
}

impl From<jsonschema::error::ValidationError<'_>> for Message {
    fn from(v: jsonschema::error::ValidationError<'_>) -> Self {
        JsonSchemaValidationError::from(v).into()
    }
}

/// Enumeration for the particular types of diagnostics we might encounter.
///
/// Numbers must be assigned as follows:
///  - the group identifier is represented by the thousands digit and up;
///  - the first classification for each group (i.e. divisible by 1000) is
///    reserved for diagnostics that have no more specific information
///    attached to them: their description must be hidden and related to
///    the group name;
///  - group 0 is a sort of null group, where no group information is known;
///  - all enum variant names for classifications belonging to a group (except
///    the null group) must be prefixed by the group name;
///  - for backward/forward-compatibility, numbers should not be reassigned.
///
/// The Description and HiddenDescription enum properties define a description
/// of the class. When Description is used, the description is prefixed before
/// the error message; when HiddenDescription is used, the message is not
/// prefixed, and should thus be sufficiently specific to not need it. The
/// latter is useful to reduce the amount of redundant information in a
/// message.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumIter,
    strum_macros::EnumProperty,
    num_derive::FromPrimitive,
)]
pub enum Classification {
    // Unclassified diagnostics (group 0).
    #[strum(props(HiddenDescription = "unclassified diagnostic"))]
    Unclassified = 0,

    #[strum(props(Description = "not yet implemented"))]
    NotYetImplemented = 1,

    #[strum(props(Description = "illegal value"))]
    IllegalValue = 2,

    #[strum(props(Description = "illegal value in hint"))]
    IllegalValueInHint = 3,

    #[strum(props(Description = "illegal URI"))]
    IllegalUri = 4,

    // Protobuf-related diagnostics (group 1).
    #[strum(props(HiddenDescription = "protobuf-related diagnostic"))]
    Proto = 1000,

    #[strum(props(HiddenDescription = "protobuf parsing failed"))]
    ProtoParseFailed = 1001,

    #[strum(props(Description = "missing required protobuf field"))]
    ProtoMissingField = 1002,

    #[strum(props(Description = "encountered a protobuf \"any\""))]
    ProtoAny = 1004,

    #[strum(props(Description = "missing protobuf \"any\" declaration"))]
    ProtoMissingAnyDeclaration = 1006,

    // YAML-reated diagnostics (group 2).
    #[strum(props(HiddenDescription = "YAML-related diagnostic"))]
    Yaml = 2000,

    #[strum(props(Description = "did not attempt to resolve YAML"))]
    YamlResolutionDisabled = 2001,

    #[strum(props(Description = "failed to resolve YAML"))]
    YamlResolutionFailed = 2002,

    #[strum(props(Description = "failed to parse YAML"))]
    YamlParseFailed = 2003,

    #[strum(props(Description = "YAML does not conform to schema"))]
    YamlSchemaValidationFailed = 2004,

    #[strum(props(Description = "missing required YAML key"))]
    YamlMissingKey = 2005,

    #[strum(props(Description = "missing required YAML array element"))]
    YamlMissingElement = 2007,

    // Link resolution diagnostics (group 3).
    #[strum(props(HiddenDescription = "link resolution diagnostic"))]
    Link = 3000,

    #[strum(props(Description = "failed to resolve anchor"))]
    LinkMissingAnchor = 3001,

    #[strum(props(Description = "failed to resolve function name"))]
    LinkMissingFunctionName = 3002,

    #[strum(props(Description = "failed to resolve type name"))]
    LinkMissingTypeName = 3003,

    #[strum(props(Description = "failed to resolve type variation name"))]
    LinkMissingTypeVariationName = 3004,

    // Type-related diagnostics (group 4).
    #[strum(props(HiddenDescription = "type-related diagnostics"))]
    Type = 4000,

    #[strum(props(Description = "unknown type"))]
    TypeUnknown = 4001,

    #[strum(props(Description = "mismatched type parameters"))]
    TypeMismatchedParameters = 4002,

    #[strum(props(Description = "mismatched field name associations"))]
    TypeMismatchedFieldNameAssociations = 4003,

    #[strum(props(Description = "invalid swizzle operation"))]
    TypeInvalidSwizzle = 4004,

    #[strum(props(Description = "mismatched types"))]
    TypeMismatch = 4005,

    #[strum(props(Description = "struct type is required"))]
    TypeStructRequired = 4006,

    // Relation-related diagnostics (group 5).
    #[strum(props(HiddenDescription = "relation-related diagnostics"))]
    Relation = 5000,

    #[strum(props(Description = "missing root relation"))]
    RelationRootMissing = 5001,

    // Expression-related diagnostics (group 6).
    #[strum(props(HiddenDescription = "expression-related diagnostics"))]
    Expression = 6000,

    #[strum(props(Description = "field reference into non-existent stream"))]
    ExpressionFieldRefMissingStream = 6001,

    #[strum(props(Description = "illegal literal value"))]
    ExpressionIllegalLiteralValue = 6002,

    #[strum(props(Description = "function definition unavailable"))]
    ExpressionFunctionDefinitionUnavailable = 6003,

    // Redundant declarations (group 7).
    #[strum(props(
        HiddenDescription = "diagnostics for pointing out parts of the plan that can be removed without changing its semantics"
    ))]
    Redundant = 7000,

    #[strum(props(Description = "redundant protobuf \"any\" declaration"))]
    RedundantProtoAnyDeclaration = 7001,

    #[strum(props(Description = "redundant extension URI definition"))]
    RedundantExtensionDefition = 7002,

    #[strum(props(Description = "redundant function declaration"))]
    RedundantFunctionDeclaration = 7003,

    #[strum(props(Description = "redundant type declaration"))]
    RedundantTypeDeclaration = 7004,

    #[strum(props(Description = "redundant type variation declaration"))]
    RedundantTypeVariationDeclaration = 7005,

    #[strum(props(Description = "redundant list slice"))]
    RedundantListSlice = 7006,

    #[strum(props(Description = "redundant field"))]
    RedundantField = 7007,
}

impl Classification {
    /// Returns the complete code for this classification.
    pub fn code(&self) -> u32 {
        *self as u32
    }

    /// Returns the name of the classiciation.
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    /// Returns the group code for this classification.
    pub fn group_code(&self) -> u32 {
        (*self as u32) / 1000
    }

    /// Returns the group variant for this classification.
    pub fn group(&self) -> Classification {
        Self::from_group(self.group_code())
            .unwrap_or_else(|| panic!("missing group for {:?}", self))
    }

    /// Returns the code for this classification within its group.
    pub fn sub_code(&self) -> u32 {
        (*self as u32) % 1000
    }

    /// Returns the description of this classification.
    pub fn description(&self) -> &str {
        self.get_str("Description")
            .or_else(|| self.get_str("HiddenDescription"))
            .unwrap_or_else(|| {
                panic!(
                    "missing Description or HiddenDescription property for {:?}",
                    self
                )
            })
    }

    /// Returns the classification associated with the given code, if any.
    pub fn from_code(code: u32) -> Option<Self> {
        Self::from_u32(code)
    }

    /// Returns the group classification associated with the given code, if
    /// any.
    pub fn group_from_code(code: u32) -> Option<Self> {
        Self::from_group(code / 1000)
    }

    /// Returns the group classification associated with the given group.
    pub fn from_group(group: u32) -> Option<Self> {
        Self::from_u32(group * 1000)
    }

    /// Formats a Message with this classification.
    pub fn format_message(
        &self,
        message: &Message,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        if let Some(description) = self.get_str("Description") {
            write!(f, "{description}: ")?;
        }
        write!(f, "{message} (code {:04})", self.code())
    }
}

impl From<Classification> for u32 {
    /// Converts a Classification into its error code.
    fn from(classification: Classification) -> Self {
        classification.code()
    }
}

/// Description of the cause of a diagnostic.
#[derive(Clone, Debug, thiserror::Error)]
pub struct Cause {
    /// The error message. Within this crate we don't bother typing these
    /// beyond the Classification enum, but we do retain typing information for
    /// messages from other crates.
    pub message: Arc<Message>,

    /// Classification of this cause. This attaches an error code and generic
    /// message for said code to the diagnostic message. The user can use these
    /// codes to for instance always promote a particular type of diagnostic to
    /// an error (like gcc -Werror).
    pub classification: Classification,
}

impl PartialEq for Cause {
    fn eq(&self, other: &Self) -> bool {
        self.message.to_string() == other.message.to_string()
            && self.classification == other.classification
    }
}

impl std::fmt::Display for Cause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.classification.format_message(&self.message, f)
    }
}

/// Convenience/shorthand macro for creating error diagnostics. Use this
/// variant when you have something that can be cast into a Message via into(),
/// like a pre-formatted string or a compatible Error type from a dependency.
macro_rules! ecause {
    ($class:ident, $message:expr) => {
        crate::output::diagnostic::Cause {
            message: std::sync::Arc::new($message.into()),
            classification: crate::output::diagnostic::Classification::$class,
        }
    };
}

/// Convenience/shorthand macro for creating error diagnostics. Use this
/// variant when you want to format a string. The argument list beyond the
/// diagnostic class identifier is passed straight to [`format!`].
macro_rules! cause {
    ($class:ident, $($args:expr),*) => {
        ecause!($class, format!($($args),*))
    };
}

/// Result type for diagnostic causes.
pub type Result<T> = std::result::Result<T, Cause>;

/// Error level for a diagnostic message.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Level used for diagnostics that don't point out anything wrong with
    /// the plan, and merely provide additional information.
    Info,

    /// Level used for diagnostics that may or may not indicate that there
    /// is something wrong with the plan, i.e. the plan *could* be valid,
    /// but the validator isn't sure.
    Warning,

    /// Level used for diagnostics that indicate that there is definitely
    /// something wrong with the plan.
    Error,
}

/// A diagnostic message, without configuration-based level override.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub struct RawDiagnostic {
    /// The cause of the diagnostic.
    pub cause: Cause,

    /// The severity of the diagnostic.
    pub level: Level,

    /// The path within the protobuf message where the diagnostic occurred.
    pub path: path::PathBuf,
}

impl std::fmt::Display for RawDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.level)?;
        if !f.alternate() {
            write!(f, " at {}", self.path)?;
        }
        write!(f, ": {}", self.cause)
    }
}

/// A diagnostic message, including configuration-based level override.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub struct Diagnostic {
    /// The cause of the diagnostic.
    pub cause: Cause,

    /// The original severity of the diagnostic.
    pub original_level: Level,

    /// The severity of the diagnostic after application of configuration.
    pub adjusted_level: Level,

    /// The path within the protobuf message where the diagnostic occurred.
    pub path: path::PathBuf,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.adjusted_level)?;
        match self.original_level.cmp(&self.adjusted_level) {
            std::cmp::Ordering::Less => write!(f, " (upgraded from {:?})", self.original_level)?,
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => {
                write!(f, " (downgraded from {:?})", self.original_level)?
            }
        }
        if !f.alternate() {
            write!(f, " at {}", self.path)?;
        }
        write!(f, ": {}", self.cause)
    }
}

impl RawDiagnostic {
    /// Converts to an AdjustedDiagnostic by adding an adjusted level.
    pub fn adjust_level(self, adjusted_level: Level) -> Diagnostic {
        Diagnostic {
            cause: self.cause,
            original_level: self.level,
            adjusted_level,
            path: self.path,
        }
    }
}

/// Convenience/shorthand macro for creating error diagnostics.
macro_rules! diag {
    ($path:expr, $level:ident, $class:ident, $($args:expr),*) => {
        diag!($path, $level, cause!($class, $($args),*))
    };
    ($path:expr, $level:ident, $cause:expr) => {
        crate::output::diagnostic::RawDiagnostic {
            cause: $cause,
            level: crate::output::diagnostic::Level::$level,
            path: $path
        }
    };
}
/*macro_rules! ediag {
    ($path:expr, $level:ident, $class:ident, $err:expr) => {
        diag!($path, $level, ecause!($class, $err))
    };
}*/

/// Result type for complete diagnostics, including path.
pub type DiagResult<T> = std::result::Result<T, RawDiagnostic>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use strum::IntoEnumIterator;

    #[test]
    fn test_diagnostic_classifications() {
        // Check validity of the classifications definitions.
        let mut descriptions = HashSet::new();
        for class in Classification::iter() {
            let group = class.group();
            if group != Classification::Unclassified {
                assert!(
                    class.name().starts_with(&group.name()),
                    "incorrect group prefix for {:?}, should start with {:?}",
                    class,
                    group
                );
            }
            assert!(
                descriptions.insert(class.description().to_string()),
                "duplicate description for {:?}",
                class
            );
        }
    }
}
