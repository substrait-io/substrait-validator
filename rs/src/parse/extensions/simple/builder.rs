// SPDX-License-Identifier: Apache-2.0

//! Module providing a builder structure to be used while parsing a simple
//! extension file.

use crate::output::extension;

#[derive(Clone, Debug, Default)]
pub struct Builder {
    // TODO
}

impl From<Builder> for extension::simple::module::Definition {
    fn from(_: Builder) -> Self {
        // TODO
        extension::simple::module::Definition::default()
    }
}
