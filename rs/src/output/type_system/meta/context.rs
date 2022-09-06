// SPDX-License-Identifier: Apache-2.0

//! Module for tracking [`Context`] and other state information when evaluating
//! derivation expressions and matching patterns.

use crate::output::type_system::meta;

/// Context for evaluating patterns and expressions.
#[derive(Clone, Debug, Default)]
pub struct Context {
    /// Named bindings that have been previously assigned or matched via
    /// patterns. The keys are stored in lower-case for case-insensitive
    /// matching.
    pub bindings: std::collections::HashMap<String, meta::Value>,
}
