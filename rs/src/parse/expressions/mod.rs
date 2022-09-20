// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating expressions.

pub mod conditionals;
pub mod functions;
pub mod literals;
pub mod misc;
pub mod references;
pub mod subqueries;

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::output::type_system::data;
use crate::parse::context;
use crate::util;
use crate::util::string::Describe;

/// Description of an expression.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Expression {
    /// Used for unknown expression types.
    #[default]
    Unresolved,

    /// Used for literals.
    Literal(literals::Literal),

    /// Used for references.
    Reference(Box<references::Reference>),

    /// Used for function calls and conditionals (which, really, are just
    /// builtin function calls).
    Function(String, Vec<functions::FunctionArgument>),

    /// Used for subqueries, or anything else where the "arguments" are too
    /// extensive to be reasonably described; the argument list is always
    /// simply represented with an ellipsis.
    BigFunction(String),

    /// Used to represent the values of a MultiOrList.
    Tuple(Vec<Expression>),

    /// Used for type casts.
    Cast(data::Type, Box<Expression>),
}

impl From<literals::Literal> for Expression {
    fn from(l: literals::Literal) -> Self {
        Expression::Literal(l)
    }
}

impl From<references::Reference> for Expression {
    fn from(r: references::Reference) -> Self {
        Expression::Reference(Box::new(r))
    }
}

impl Describe for Expression {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            Expression::Unresolved => write!(f, "?"),
            Expression::Literal(x) => x.describe(f, limit),
            Expression::Reference(x) => x.describe(f, limit),
            Expression::Function(name, args) => {
                let (name_limit, args_limit) = limit.split(name.len());
                util::string::describe_identifier(f, name, name_limit)?;
                write!(f, "(")?;
                util::string::describe_sequence(f, args, args_limit, 20, |f, expr, _, limit| {
                    expr.describe(f, limit)
                })?;
                write!(f, ")")
            }
            Expression::BigFunction(name) => util::string::describe_identifier(f, name, limit),
            Expression::Tuple(items) => {
                write!(f, "(")?;
                util::string::describe_sequence(f, items, limit, 20, |f, expr, _, limit| {
                    expr.describe(f, limit)
                })?;
                write!(f, ")")
            }
            Expression::Cast(data_type, expression) => {
                let (type_limit, expr_limit) = limit.split(10);
                write!(f, "(")?;
                data_type.describe(f, type_limit)?;
                write!(f, ")(")?;
                expression.describe(f, expr_limit)?;
                write!(f, ")")
            }
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Expression {
    /// Shorthand for a new null literal.
    pub fn new_null(data_type: data::Type) -> Expression {
        literals::Literal::new_null(data_type).into()
    }
}

/// Expressions may include enums to support the legacy function argument
/// specification method.
#[derive(Clone, Debug)]
pub enum ExpressionOrEnum {
    /// Used for value arguments or normal expressions.
    Value(Expression),

    /// Used for enum function arguments.
    Enum(Option<String>),
}

impl Default for ExpressionOrEnum {
    fn default() -> Self {
        ExpressionOrEnum::Value(Expression::Unresolved)
    }
}

impl From<Expression> for ExpressionOrEnum {
    fn from(e: Expression) -> Self {
        ExpressionOrEnum::Value(e)
    }
}

impl Describe for ExpressionOrEnum {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            ExpressionOrEnum::Value(e) => e.describe(f, limit),
            ExpressionOrEnum::Enum(Some(x)) => util::string::describe_identifier(f, x, limit),
            ExpressionOrEnum::Enum(None) => write!(f, "-"),
        }
    }
}

impl std::fmt::Display for ExpressionOrEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

/// Parse an expression type. Returns a description of said expression.
fn parse_expression_type(
    x: &substrait::expression::RexType,
    y: &mut context::Context,
    enum_allowed: bool,
) -> diagnostic::Result<ExpressionOrEnum> {
    Ok(match x {
        substrait::expression::RexType::Literal(x) => {
            literals::parse_literal(x, y).map(Expression::from)?.into()
        }
        substrait::expression::RexType::Selection(x) => {
            references::parse_field_reference(x.as_ref(), y)
                .map(Expression::from)?
                .into()
        }
        substrait::expression::RexType::ScalarFunction(x) => {
            functions::parse_scalar_function(x, y)?.into()
        }
        substrait::expression::RexType::WindowFunction(x) => {
            functions::parse_window_function(x, y)?.into()
        }
        substrait::expression::RexType::IfThen(x) => {
            conditionals::parse_if_then(x.as_ref(), y)?.into()
        }
        substrait::expression::RexType::SwitchExpression(x) => {
            conditionals::parse_switch(x.as_ref(), y)?.into()
        }
        substrait::expression::RexType::SingularOrList(x) => {
            conditionals::parse_singular_or_list(x.as_ref(), y)?.into()
        }
        substrait::expression::RexType::MultiOrList(x) => {
            conditionals::parse_multi_or_list(x, y)?.into()
        }
        substrait::expression::RexType::Enum(x) => {
            if !enum_allowed {
                diagnostic!(
                    y,
                    Error,
                    IllegalValue,
                    "function option enum variants are not allowed here"
                );
            }
            misc::parse_enum(x, y)?
        }
        substrait::expression::RexType::Cast(x) => misc::parse_cast(x.as_ref(), y)?.into(),
        substrait::expression::RexType::Subquery(x) => {
            subqueries::parse_subquery(x.as_ref(), y)?.into()
        }
    })
}

/// Parse an expression. Returns a description of said expression.
fn parse_expression_internal(
    x: &substrait::Expression,
    y: &mut context::Context,
    enum_allowed: bool,
) -> diagnostic::Result<ExpressionOrEnum> {
    // Parse the expression.
    let (n, e) = proto_required_field!(x, y, rex_type, parse_expression_type, enum_allowed);
    let expression = e.unwrap_or_default();
    let data_type = n.data_type();

    // Describe node.
    y.set_data_type(data_type);
    describe!(y, Expression, "{}", expression);
    summary!(y, "Expression: {:#}", expression);
    Ok(expression)
}

/// Parse a regular expression (anything except a function option enum
/// variant). Returns a description of said expression.
pub fn parse_expression(
    x: &substrait::Expression,
    y: &mut context::Context,
) -> diagnostic::Result<Expression> {
    match parse_expression_internal(x, y, false)? {
        ExpressionOrEnum::Value(x) => Ok(x),
        ExpressionOrEnum::Enum(_) => Err(cause!(IllegalValue, "enums are not allowed here")),
    }
}

/// Parse a predicate expression (a normal expression that yields a boolean).
/// Returns a description of said expression.
pub fn parse_predicate(
    x: &substrait::Expression,
    y: &mut context::Context,
) -> diagnostic::Result<Expression> {
    let expression = parse_expression(x, y)?;
    let data_type = y.data_type();
    if !matches!(
        data_type.class(),
        data::Class::Simple(data::class::Simple::Boolean) | data::Class::Unresolved
    ) {
        diagnostic!(
            y,
            Error,
            TypeMismatch,
            "predicates must yield booleans, but found {}",
            data_type
        );
    }
    Ok(expression)
}

/// Parse a legacy function argument, which can be an expression or an enum
/// option.
fn parse_legacy_function_argument(
    x: &substrait::Expression,
    y: &mut context::Context,
) -> diagnostic::Result<ExpressionOrEnum> {
    parse_expression_internal(x, y, true)
}
