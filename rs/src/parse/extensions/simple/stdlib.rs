// SPDX-License-Identifier: Apache-2.0

//! Registry of the standard `extension:io.substrait:*` extension files, bundled
//! into the validator at build time.
//!
//! Unlike the old URI-based model, a URN is an *identifier*, not a location:
//! it cannot be downloaded. To still allow plans that reference the standard
//! Substrait extensions to be validated out of the box (and offline), the YAML
//! files are embedded into the binary (via the [`substrait-extensions`]
//! [`substrait_extensions`] crate) and resolved by their URN here. Custom
//! extensions can be resolved by registering a resolver or override on the
//! [`Config`](crate::Config).

use substrait_extensions::extensions::SIMPLE_EXTENSIONS;

/// Resolves a standard extension URN to the bytes of its bundled YAML file, if
/// the URN refers to one of the standard Substrait extensions.
///
/// The standard extension files and their URNs are provided by the
/// [`substrait-extensions`](substrait_extensions) crate, generated and
/// published from the Substrait specification by substrait-packaging.
pub fn resolve(urn: &str) -> Option<&'static [u8]> {
    SIMPLE_EXTENSIONS
        .iter()
        .find(|(known, _)| *known == urn)
        .map(|(_, yaml)| yaml.as_bytes())
}
