// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating function calls.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::output::extension;
use crate::output::tree;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::extensions;
use crate::parse::sorts;
use crate::parse::types;
use crate::util;
use crate::util::string::Describe;
use std::sync::Arc;

/// A function argument; either a value, a type, or an enum option.
#[derive(Clone, Debug, PartialEq)]
pub enum FunctionArgument {
    /// Used for value arguments or normal expressions.
    Value(expressions::Expression),

    /// Used for type arguments.
    Type(Arc<data_type::DataType>),

    /// Used for enum option arguments.
    Enum(Option<String>),
}

impl Default for FunctionArgument {
    fn default() -> Self {
        FunctionArgument::Value(expressions::Expression::default())
    }
}

impl From<expressions::Expression> for FunctionArgument {
    fn from(expr: expressions::Expression) -> Self {
        FunctionArgument::Value(expr)
    }
}

impl Describe for FunctionArgument {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            FunctionArgument::Value(e) => e.describe(f, limit),
            FunctionArgument::Type(e) => e.describe(f, limit),
            FunctionArgument::Enum(Some(x)) => util::string::describe_identifier(f, x, limit),
            FunctionArgument::Enum(None) => write!(f, "-"),
        }
    }
}

impl std::fmt::Display for FunctionArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

/// Parse an enum option argument type.
fn parse_enum_type(
    x: &substrait::function_argument::r#enum::EnumKind,
    _y: &mut context::Context,
) -> diagnostic::Result<Option<String>> {
    match x {
        substrait::function_argument::r#enum::EnumKind::Specified(x) => Ok(Some(x.clone())),
        substrait::function_argument::r#enum::EnumKind::Unspecified(_) => Ok(None),
    }
}

/// Parse an enum option argument.
fn parse_enum(
    x: &substrait::function_argument::Enum,
    y: &mut context::Context,
) -> diagnostic::Result<Option<String>> {
    Ok(proto_required_field!(x, y, enum_kind, parse_enum_type)
        .1
        .flatten())
}

/// Parse a 0.3.0+ function argument type.
fn parse_function_argument_type(
    x: &substrait::function_argument::ArgType,
    y: &mut context::Context,
) -> diagnostic::Result<FunctionArgument> {
    match x {
        substrait::function_argument::ArgType::Enum(x) => {
            Ok(FunctionArgument::Enum(parse_enum(x, y)?))
        }
        substrait::function_argument::ArgType::Type(x) => {
            types::parse_type(x, y)?;
            Ok(FunctionArgument::Type(y.data_type()))
        }
        substrait::function_argument::ArgType::Value(x) => Ok(FunctionArgument::Value(
            expressions::parse_expression(x, y)?,
        )),
    }
}

/// Parse a 0.3.0+ function argument.
fn parse_function_argument(
    x: &substrait::FunctionArgument,
    y: &mut context::Context,
) -> diagnostic::Result<FunctionArgument> {
    Ok(
        proto_required_field!(x, y, arg_type, parse_function_argument_type)
            .1
            .unwrap_or_default(),
    )
}

/// Parse a pre-0.3.0 function argument expression.
fn parse_legacy_function_argument(
    x: &substrait::Expression,
    y: &mut context::Context,
) -> diagnostic::Result<FunctionArgument> {
    expressions::parse_legacy_function_argument(x, y).map(|x| match x {
        expressions::ExpressionOrEnum::Value(x) => FunctionArgument::Value(x),
        expressions::ExpressionOrEnum::Enum(x) => FunctionArgument::Enum(x),
    })
}

/// Matches a function call with its YAML definition, yielding its return type.
/// Yields an unresolved type if resolution fails.
pub fn check_function(
    y: &mut context::Context,
    _function: &extension::Function,
    _options: &[Option<String>],
    _arg_types: &[Arc<data_type::DataType>],
) -> Arc<data_type::DataType> {
    // TODO: check consistency of:
    //  - _function (function definition information from the YAML file);
    //  - _options: number of options passed to the function, and validity of
    //    their values;
    //  - _arg_types: whether an overload exists for this set of argument
    //    types;
    diagnostic!(
        y,
        Warning,
        NotYetImplemented,
        "matching function calls with their definitions"
    );
    Arc::default()
}

/// Parsing logic common to scalar and window functions.
fn parse_function(
    y: &mut context::Context,
    function: Option<Arc<extension::Reference<extension::Function>>>,
    arguments: (Vec<Arc<tree::Node>>, Vec<Option<FunctionArgument>>),
    legacy_arguments: (Vec<Arc<tree::Node>>, Vec<Option<FunctionArgument>>),
    return_type: Arc<data_type::DataType>,
) -> (Arc<data_type::DataType>, expressions::Expression) {
    // Determine the name of the function.
    let name = function
        .as_ref()
        .map(|x| x.name.to_string())
        .unwrap_or_else(|| String::from("?"));

    // Reconcile v3.0.0+ vs older function argument syntax.
    let arguments = if legacy_arguments.1.is_empty() {
        arguments
    } else if arguments.1.is_empty() {
        diagnostic!(
            y,
            Warning,
            Deprecation,
            "the args field for specifying function arguments was deprecated Substrait 0.3.0 (#161)"
        );
        legacy_arguments
    } else {
        if arguments != legacy_arguments {
            diagnostic!(
                y,
                Error,
                IllegalValue,
                "mismatch between v0.3+ and legacy function argument specification"
            );
            comment!(
                y,
                "If both the v0.3+ and legacy syntax is used to specify function \
                arguments, please make sure both map to the same arguments. If \
                the argument pack is not representable using the legacy syntax, \
                do not use it."
            );
        }
        arguments
    };

    // Unpack the arguments into the function's enum options and regular
    // arguments.
    let mut opt_values = vec![];
    let mut opt_exprs = vec![];
    let mut arg_types = vec![];
    let mut arg_exprs = vec![];
    for (node, expr) in arguments
        .0
        .into_iter()
        .zip(arguments.1.into_iter().map(|x| x.unwrap_or_default()))
    {
        if let FunctionArgument::Enum(x) = &expr {
            if opt_exprs.is_empty() && !arg_exprs.is_empty() {
                diagnostic!(
                    y,
                    Error,
                    IllegalValue,
                    "function option argument specified after first regular argument"
                );
            }
            opt_values.push(x.clone());
            opt_exprs.push(expr);
        } else {
            arg_types.push(node.data_type());
            arg_exprs.push(expr);
        }
    }
    opt_exprs.extend(arg_exprs.into_iter());
    let expression = expressions::Expression::Function(name, opt_exprs);
    let opt_values = opt_values;
    let arg_types = arg_types;

    // If the function was resolved, check whether it's valid.
    let return_type = if let Some(reference) = function {
        if let Some(function) = &reference.definition {
            let derived = check_function(y, function, &opt_values, &arg_types);
            types::assert_equal(
                y,
                &return_type,
                &derived,
                "specified return type must match derived",
            )
        } else {
            diagnostic!(
                y,
                Warning,
                ExpressionFunctionDefinitionUnavailable,
                "cannot check validity of call"
            );
            return_type
        }
    } else {
        return_type
    };

    (return_type, expression)
}

/// Parse a scalar function. Returns a description of the function call
/// expression.
pub fn parse_scalar_function(
    x: &substrait::expression::ScalarFunction,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse function information.
    let function = proto_primitive_field!(
        x,
        y,
        function_reference,
        extensions::simple::parse_function_reference
    )
    .1;
    #[allow(deprecated)]
    let legacy_arguments = proto_repeated_field!(x, y, args, parse_legacy_function_argument);
    let arguments = proto_repeated_field!(x, y, arguments, parse_function_argument);
    let return_type = proto_required_field!(x, y, output_type, types::parse_type)
        .0
        .data_type();

    // Check function information.
    let (return_type, expression) =
        parse_function(y, function, arguments, legacy_arguments, return_type);

    // Describe node.
    y.set_data_type(return_type);
    describe!(y, Expression, "{}", expression);
    summary!(y, "Scalar function call: {:#}", expression);
    Ok(expression)
}

/// Parse a window function bound.
fn parse_bound(
    _x: &substrait::expression::window_function::Bound,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    // TODO: check window function bound.
    // FIXME: I have no idea what these bounds signify. The spec doesn't
    // seem to specify.
    diagnostic!(
        y,
        Warning,
        NotYetImplemented,
        "validation of window function bounds"
    );
    Ok(())
}

/// Parse a window function. Returns a description of the function call
/// expression.
pub fn parse_window_function(
    x: &substrait::expression::WindowFunction,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse function information.
    let function = proto_primitive_field!(
        x,
        y,
        function_reference,
        extensions::simple::parse_function_reference
    )
    .1;
    #[allow(deprecated)]
    let legacy_arguments = proto_repeated_field!(x, y, args, parse_legacy_function_argument);
    let arguments = proto_repeated_field!(x, y, arguments, parse_function_argument);
    let return_type = proto_required_field!(x, y, output_type, types::parse_type)
        .0
        .data_type();

    // Check function information.
    let (return_type, expression) =
        parse_function(y, function, arguments, legacy_arguments, return_type);

    // Parse modifiers.
    proto_repeated_field!(x, y, partitions, expressions::parse_expression);
    proto_repeated_field!(x, y, sorts, sorts::parse_sort_field);
    proto_field!(x, y, upper_bound, parse_bound);
    proto_field!(x, y, lower_bound, parse_bound);
    proto_enum_field!(x, y, phase, substrait::AggregationPhase);

    // TODO: check window function configuration.
    // FIXME: I have no idea what these partitions signify. The spec doesn't
    // seem to specify.
    if !x.partitions.is_empty() {
        diagnostic!(
            y,
            Warning,
            NotYetImplemented,
            "validation of partitions field"
        );
    }

    // Describe node.
    y.set_data_type(return_type);
    describe!(y, Expression, "{}", expression);
    summary!(y, "Window function call: {:#}", expression);
    Ok(expression)
}

/// Parse an aggregate function. Returns a description of the function call
/// expression.
pub fn parse_aggregate_function(
    x: &substrait::AggregateFunction,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse function information.
    let function = proto_primitive_field!(
        x,
        y,
        function_reference,
        extensions::simple::parse_function_reference
    )
    .1;
    #[allow(deprecated)]
    let legacy_arguments = proto_repeated_field!(x, y, args, parse_legacy_function_argument);
    let arguments = proto_repeated_field!(x, y, arguments, parse_function_argument);
    let return_type = proto_required_field!(x, y, output_type, types::parse_type)
        .0
        .data_type();

    // Check function information.
    let (return_type, expression) =
        parse_function(y, function, arguments, legacy_arguments, return_type);

    // Parse modifiers.
    proto_repeated_field!(x, y, sorts, sorts::parse_sort_field);
    proto_enum_field!(x, y, phase, substrait::AggregationPhase);
    proto_enum_field!(
        x,
        y,
        invocation,
        substrait::aggregate_function::AggregationInvocation
    );

    // Describe node.
    y.set_data_type(return_type);
    describe!(y, Expression, "{}", expression);
    summary!(y, "Aggregate function call: {:#}", expression);
    Ok(expression)
}
