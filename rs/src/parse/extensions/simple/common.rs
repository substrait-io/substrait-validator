// SPDX-License-Identifier: Apache-2.0

//! Module providing private helper functions that are not specific to any
//! particular simple extension construct.

use crate::output::diagnostic::Result;
use crate::parse::context;

/// Parser for names given to things.
pub fn parse_name(x: &str, y: &mut context::Context, construct: &str) -> Result<String> {
    static IDENTIFIER_RE: once_cell::sync::Lazy<regex::Regex> =
        once_cell::sync::Lazy::new(|| regex::Regex::new("[a-zA-Z_][a-zA-Z0-9_\\.]*").unwrap());

    if x.is_empty() {
        diagnostic!(
            y,
            Info,
            LinkDiscouragedName,
            "using the empty string as a {construct} name is not explicitly \
            illegal, but probably not a good idea"
        );
    } else if !IDENTIFIER_RE.is_match(x) {
        diagnostic!(
            y,
            Info,
            LinkDiscouragedName,
            "it is recommended for {construct} names to case-insensitively \
            match [a-z_][a-z0-9_]* for maximum compatibility"
        );
    } else if x.contains('.') {
        diagnostic!(
            y,
            Info,
            LinkDiscouragedName,
            "using periods within a {construct} name is not explicitly \
            illegal, but probably not a good idea, as they are also used as \
            namespace separators for dependencies"
        );
    }
    Ok(x.to_owned())
}
