// SPDX-License-Identifier: Apache-2.0

//! Module for [`Program`]s that perform a number of operations on a
//! [`meta::Context`] in order to yield a [`meta::Value`]. These are used
//! primarily to derive intermediate and return types, but they can also
//! check constraints on the arguments types passed to a function.

use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::util;
use crate::util::string::Describe;

use super::Pattern;

/// A program operating on a [`meta::Context`] to yield a [`meta::Value`].
/// The syntax is simply a multiline string, where each non-empty line except
/// the last maps to a statement, and the last non-empty line maps to the
/// expression. # can be used as an end-of-line comment, and semicolons may
/// be used in place of newlines (though these do not terminate a # comment).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Program {
    /// Zero or more evaluate-and-match statements to execute before
    /// evaluating the final expression.
    pub statements: Vec<Statement>,

    /// The expression that computes the final result of the program.
    pub expression: meta::pattern::Value,
}

impl Describe for Program {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        let expr_limit = if !self.statements.is_empty() {
            let (expr_limit, stmts_limit) = limit.split(50);
            util::string::describe_sequence_with_sep(
                f,
                &self.statements,
                stmts_limit,
                50,
                ';',
                |f, stmt, _, limit| stmt.describe(f, limit),
            )?;
            write!(f, "; ")?;
            expr_limit
        } else {
            limit
        };
        self.expression.describe(f, expr_limit)
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Program {
    /// Evaluates this program to a generalized metavalue.
    pub fn evaluate(&self, context: &mut meta::Context) -> diagnostic::Result<meta::Value> {
        for (index, statement) in self.statements.iter().enumerate() {
            statement
                .execute(context)
                .map_err(|e| e.prefix(format!("on line {}", index + 1)))?;
        }
        self.expression
            .evaluate_with_context(context)
            .map_err(|e| e.prefix("in final expression"))
    }

    /// Evaluates this program to a data type.
    pub fn evaluate_type(&self, context: &mut meta::Context) -> diagnostic::Result<data::Type> {
        self.evaluate(context)?.get_data_type().ok_or_else(|| {
            cause!(
                TypeDerivationInvalid,
                "type derivation program must yield a typename"
            )
        })
    }
}

/// An evaluate-and-match statement. This usually looks like an assignment
/// statement, with something that captures the result on the LHS, and
/// something expression-like on the RHS. However, it's more powerful than
/// that: you can also use it to check constraints by placing a constraint
/// pattern on the LHS, or for generalized assertions by placing `true` on
/// the LHS (in this case, only the value "true" will match, so the matching
/// process will fail if the RHS does not evaluate to true). The generalized
/// syntax for this is `lhs = rhs`, but `assert rhs matches lhs` and `assert rhs`
/// are syntactic sugar for aforementioned patterns to make those easier to
/// understand.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Statement {
    /// The pattern appearing on the left-hand side of the evaluate-and-match
    /// statement. This is what the result of the expression will be matched
    /// against.
    pub lhs_pattern: meta::pattern::Value,

    /// The expression appearing on the right-hand side of the
    /// evaluate-and-match statement. This is evaluated first, then its result
    /// is matched against lhs_pattern.
    pub rhs_expression: meta::pattern::Value,
}

impl Describe for Statement {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        let (lhs_limit, rhs_limit) = limit.split(limit.chars() / 2);
        self.lhs_pattern.describe(f, lhs_limit)?;
        write!(f, " = ")?;
        self.rhs_expression.describe(f, rhs_limit)
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Statement {
    /// Executes the statement within the given context.
    pub fn execute(&self, context: &mut meta::Context) -> diagnostic::Result<()> {
        let value = self
            .rhs_expression
            .evaluate_with_context(context)
            .map_err(|e| e.prefix("while evaluating RHS"))?;
        if self
            .lhs_pattern
            .match_pattern_with_context(context, &value)?
        {
            Ok(())
        } else {
            Err(cause!(
                TypeDerivationFailed,
                "failed to match {} against pattern",
                value
            ))
        }
    }
}
