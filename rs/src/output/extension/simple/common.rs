// SPDX-License-Identifier: Apache-2.0

//! Module for the common types involved with representing extension
//! definitions.

use crate::output;

/// Identifying information associated with an extension, that can be used to
/// refer to the extension from elsewhere.
#[derive(Clone, Debug)]
pub struct Identifier {
    /// The URI that the extension was declared with. Matched case-sensitively.
    pub uri: String,

    /// One or more aliases for this extension within the scope of the URI.
    /// Matched case-insensitively.
    pub names: Vec<String>,

    /// Unique number for the extension, generated during traversal. The
    /// number is only unique within the scope of a single run of the
    /// validator, and may change between runs.
    pub extension_id: u64,

    /// The path that the extension is defined in.
    pub definition_path: output::path::PathBuf,
}

/// Non-functional metadata common to all extension types.
#[derive(Clone, Debug, Default)]
pub struct Metadata {
    // Optional description of the extension. Only serves as documentation.
    pub description: String,
}
