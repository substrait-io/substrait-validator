// SPDX-License-Identifier: Apache-2.0

//! This module provides the configuration structure for the validator.
//!
//! This structure, [`Config`], is to be constructed by the application using
//! the validator to configure it. Alternatively, the default configuration can
//! be constructed by using the [`std::default::Default`] trait.

use crate::output::diagnostic;
pub use glob;
use std::collections::HashMap;

/// Trait object representing some immutable binary data.
pub type BinaryData = Box<dyn AsRef<[u8]>>;

/// Trait object representing some error data.
pub type ErrorData = Box<dyn std::error::Error>;

/// Callback function type for resolving the content of an extension URN.
///
/// A URN is an identifier, not a location, so resolution is a lookup that maps
/// the URN to the bytes of the corresponding YAML file (rather than a
/// download).
pub type UrnResolver =
    Box<dyn Fn(&str) -> std::result::Result<BinaryData, ErrorData> + Send + Sync>;

/// Configuration structure.
pub struct Config {
    /// When set, do not generate warnings for unknown protobuf fields that are
    /// set to their protobuf-defined default value.
    pub ignore_unknown_fields: bool,

    /// Protobuf message URLs that are explicitly allowed for use in "any"
    /// messages, i.e. that the caller warrants the existence of in the
    /// consumer that the plan is validated for.
    pub allowed_proto_any_urls: Vec<glob::Pattern>,

    /// Allows the level of diagnostic messages to be overridden based on their
    /// classification/code. The logic for this is as follows:
    ///
    ///  - if an entry exists for the classication of the incoming diagnostic,
    ///    override its error level to at most the second argument, and then to
    ///    at least the first argument. Otherwise,
    ///  - if an entry exists for the group of said classification, use its
    ///    level limits instead. Otherwise,
    ///  - if an entry exists for Unclassified (code 0), use its level limits
    ///    instead. Otherwise, do not adjust the level.
    ///
    /// Note that setting an entry to  (Info, Error) leaves the diagnostic
    /// level unchanged.
    pub diagnostic_level_overrides:
        HashMap<diagnostic::Classification, (diagnostic::Level, diagnostic::Level)>,

    /// Allows extension URNs from the plan to be remapped (Some(mapping)) or
    /// ignored (None). All resolution can effectively be disabled by just
    /// adding a rule that maps * to None. Furthermore, this can be used to
    /// remap a URN to a different URN that a custom resolver knows how to
    /// resolve.
    pub urn_overrides: Vec<(glob::Pattern, Option<String>)>,

    /// Optional callback function for resolving extension URNs. If specified,
    /// all URNs (after processing urn_overrides) are looked up using this
    /// function. The function takes the URN as its argument, and should either
    /// return the content of the corresponding YAML file as a `Vec<u8>` or
    /// return a String-based error. If no resolver is specified, only the
    /// standard extensions bundled into the validator can be resolved.
    pub urn_resolver: Option<UrnResolver>,

    /// Optional URN resolution depth. If specified, dependencies are only
    /// resolved this many levels deep. Setting this to zero effectively
    /// disables extension URN resolution altogether.
    pub max_urn_resolution_depth: Option<usize>,
}

// TODO: enable URN resolution by default once all that works. Then this can
// be derived again. Also still need to expose the depth option in extensions.
impl Default for Config {
    fn default() -> Self {
        Self {
            ignore_unknown_fields: Default::default(),
            allowed_proto_any_urls: Default::default(),
            diagnostic_level_overrides: Default::default(),
            urn_overrides: Default::default(),
            urn_resolver: Default::default(),
            max_urn_resolution_depth: Some(0),
        }
    }
}

impl Config {
    /// Creates a default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Instructs the validator to ignore protobuf fields that it doesn't know
    /// about yet (i.e., that have been added to the Substrait protobuf
    /// descriptions, but haven't yet been implemented in the validator) if the
    /// fields are set to their default value. If this option isn't set, or if
    /// an unknown field is not set to its default value, a warning is emitted.
    pub fn ignore_unknown_fields(&mut self) {
        self.ignore_unknown_fields = true;
    }

    /// Explicitly allows a protobuf message type to be used in advanced
    /// extensions, despite the fact that the validator can't validate it. If
    /// an advanced extension is encountered that isn't explicitly allowed, a
    /// warning is emitted.
    pub fn allow_proto_any_url(&mut self, pattern: glob::Pattern) {
        self.allowed_proto_any_urls.push(pattern);
    }

    /// Sets a minimum and/or maximum error level for the given class of
    /// diagnostic messages. Any previous settings for this class are
    /// overridden.
    pub fn override_diagnostic_level(
        &mut self,
        class: diagnostic::Classification,
        minimum: diagnostic::Level,
        maximum: diagnostic::Level,
    ) {
        self.diagnostic_level_overrides
            .insert(class, (minimum, maximum));
    }

    /// Overrides the resolution behavior for extension URNs matching the given
    /// pattern. If resolve_as is None, the URN will not be resolved; if it is
    /// Some(s), it will be resolved as if the URN in the plan had been s.
    pub fn override_urn<S: Into<String>>(&mut self, pattern: glob::Pattern, resolve_as: Option<S>) {
        self.urn_overrides
            .push((pattern, resolve_as.map(|s| s.into())));
    }

    /// Registers an extension URN resolution function with this configuration.
    /// If the given function fails, any previously registered function will be
    /// used as a fallback.
    pub fn add_urn_resolver<F, D, E>(&mut self, resolver: F)
    where
        F: Fn(&str) -> Result<D, E> + Send + Sync + 'static,
        D: AsRef<[u8]> + 'static,
        E: std::error::Error + 'static,
    {
        let previous = self.urn_resolver.take();
        self.urn_resolver = Some(Box::new(move |urn| match resolver(urn) {
            Ok(d) => Ok(Box::new(d)),
            Err(e) => match &previous {
                Some(f) => f.as_ref()(urn),
                None => Err(Box::new(e)),
            },
        }));
    }

    /// Sets the maximum recursion depth for URN resolution, in the presence of
    /// transitive dependencies. Setting this to None disables the limit,
    /// setting this to zero disables URN resolution entirely.
    pub fn set_max_urn_resolution_depth(&mut self, depth: Option<usize>) {
        self.max_urn_resolution_depth = depth;
    }
}
