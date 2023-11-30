// SPDX-License-Identifier: Apache-2.0

//! Module providing parse/validation functions for parsing YAML type variation
//! declarations.

use crate::input::yaml;
use crate::output::diagnostic::Result;
use crate::parse::context;
use crate::parse::extensions::simple::modules;

/// Parse a type variation declaration.
pub fn parse_type_variation(
    _x: &yaml::Value,
    _y: &mut context::Context,
    _z: &mut modules::Builder,
) -> Result<()> {
    // TODO
    Ok(())
}
