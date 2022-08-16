// SPDX-License-Identifier: Apache-2.0

//! Module for builtin [`Function`]s that operate on [`meta::Value`]s.

use crate::output::diagnostic;
use crate::output::type_system::meta;
use crate::util;
use crate::util::string::Describe;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Function {
    Todo,
}

impl Describe for Function {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            Function::Todo => write!(f, "TODO"),
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Function {
    /// Evaluates this pattern with a provided context.
    pub fn evaluate_with_context(
        &self,
        _context: &mut meta::Context,
        _args: &[meta::Pattern],
    ) -> diagnostic::Result<meta::Value> {
        todo!()
    }
}
