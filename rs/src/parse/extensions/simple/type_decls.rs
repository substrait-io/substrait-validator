// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for parsing YAML type
//! declarations.

use crate::input::yaml;
use crate::output::diagnostic::Result;
use crate::parse::context;
use crate::parse::extensions::simple::builder;

/// Parse a type declaration.
pub fn parse_type(
    _x: &yaml::Value,
    _y: &mut context::Context,
    _z: &mut builder::Builder,
) -> Result<()> {
    // TODO
    Ok(())
}
