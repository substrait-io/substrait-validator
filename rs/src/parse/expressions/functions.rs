// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating function calls.

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::output::extension;
use crate::output::type_system::data;
use crate::parse::context;
use crate::parse::expressions;
use crate::parse::extensions;
use crate::parse::sorts;
use crate::parse::types;
use crate::util;
use crate::util::string::Describe;

/// The type of function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionType {
    /// A scalar function, converting a single value to a single value.
    Scalar,

    /// An aggregate function, reducing a list of values to a single value.
    Aggregate,

    /// A window function, reducing a window within a list of values to a
    /// single value.
    Window,
}

/// A function argument; either a value, a type, or an enum option.
#[derive(Clone, Debug, PartialEq)]
pub enum FunctionArgument {
    /// Used when a function argument is so malformed that not even the type of
    /// argument is known.
    Unresolved,

    /// Used for value arguments or normal expressions.
    Value(data::Type, expressions::Expression),

    /// Used for type arguments.
    Type(data::Type),

    /// Used for enum option arguments.
    Enum(Option<String>),
}

impl Default for FunctionArgument {
    fn default() -> Self {
        FunctionArgument::Unresolved
    }
}

impl Describe for FunctionArgument {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match self {
            FunctionArgument::Unresolved => write!(f, "!"),
            FunctionArgument::Value(_, e) => e.describe(f, limit),
            FunctionArgument::Type(t) => t.describe(f, limit),
            FunctionArgument::Enum(Some(s)) => util::string::describe_identifier(f, s, limit),
            FunctionArgument::Enum(None) => write!(f, "-"),
        }
    }
}

impl std::fmt::Display for FunctionArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

/// Information about the context in which a function is being called.
#[derive(Clone, Debug)]
pub struct FunctionContext {
    /// The type of function expected.
    pub function_type: FunctionType,

    /// The list of arguments bound to the function.
    pub arguments: Vec<FunctionArgument>,

    /// If known, the expected return type of the function. If not known this
    /// can just be set to unresolved.
    pub return_type: data::Type,
}

/// Information about the context in which a function is being called.
#[derive(Clone, Debug)]
pub struct FunctionBinding {
    /// Reference to the bound function, for as far as this is known.
    pub function: extension::simple::function::Reference,

    /// An expression representing the function binding for diagnostics.
    pub expression: expressions::Expression,

    /// The return type of the function.
    pub return_type: data::Type,
}

impl FunctionBinding {
    /// Try to bind one of the provided function implementations to the
    /// provided function context.
    ///
    /// This is purely a validator thing. For valid plans, there should only
    /// ever be one implementation after name resolution, and the return type
    /// should already have been specified. Much more intelligence was thrown
    /// in here just to help people find and correct mistakes efficiently.
    /// Common misconceptions and mistakes, like using the simple function name
    /// vs. the compound name, not specifying optional arguments, or not
    /// specifying the (correct) return type should yield more than just a
    /// generic error message here!
    pub fn new(
        functions: Option<&extension::simple::function::ResolutionResult>,
        function_context: &FunctionContext,
        parse_context: &mut context::Context,
    ) -> FunctionBinding {
        // TODO: this should check each function implementation in the
        // resolution result against the provided argument list, in order to
        // try to derive which function overload (and compound name) is the
        // correct one for the provided arguments, and to check whether the
        // specified return type is correct. Or, if no return type was
        // specified, to provide an info message for what the correct one is.
        // It should NOT send errors for unresolved or ambiguously resolved
        // functions; this will already have been done when the anchor was
        // defined.

        // This should also check whether the additional context provided to
        // aggregate and window functions is valid. This will require adding
        // information to FunctionContext and/or FunctionType.

        // If there is a conflict between the derived and expected return type,
        // favor the expected return type, since this is more likely to avoid
        // trivial error messages downstream.
        let function = if let Some(functions) = functions {
            diagnostic!(
                parse_context,
                Warning,
                NotYetImplemented,
                "matching function calls with their definitions"
            );
            functions.as_item()
        } else {
            Default::default()
        };
        let name = function.name.to_string();
        FunctionBinding {
            function,
            expression: expressions::Expression::Function(name, function_context.arguments.clone()),
            return_type: function_context.return_type.clone(),
        }
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
        substrait::function_argument::ArgType::Value(x) => {
            let expression = expressions::parse_expression(x, y)?;
            Ok(FunctionArgument::Value(y.data_type(), expression))
        }
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
        expressions::ExpressionOrEnum::Value(x) => FunctionArgument::Value(y.data_type(), x),
        expressions::ExpressionOrEnum::Enum(x) => FunctionArgument::Enum(x),
    })
}

/// Reconcile v0.3.0+ vs older function argument syntax.
fn handle_legacy_arguments(
    y: &mut context::Context,
    arguments: Vec<FunctionArgument>,
    legacy_arguments: Vec<FunctionArgument>,
) -> Vec<FunctionArgument> {
    if legacy_arguments.is_empty() {
        arguments
    } else if arguments.is_empty() {
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
                the argument pack cannot be represented using the legacy syntax, \
                do not use it."
            );
        }
        arguments
    }
}

/// Parse a scalar function. Returns a description of the function call
/// expression.
pub fn parse_scalar_function(
    x: &substrait::expression::ScalarFunction,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse function information.
    let functions = proto_primitive_field!(
        x,
        y,
        function_reference,
        extensions::simple::parse_function_reference
    )
    .1;
    #[allow(deprecated)]
    let legacy_arguments = proto_repeated_field!(x, y, args, parse_legacy_function_argument)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let arguments = proto_repeated_field!(x, y, arguments, parse_function_argument)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let return_type = proto_required_field!(x, y, output_type, types::parse_type)
        .0
        .data_type();

    // Try to bind the function.
    let arguments = handle_legacy_arguments(y, arguments, legacy_arguments);
    let context = FunctionContext {
        function_type: FunctionType::Scalar,
        arguments,
        return_type,
    };
    let binding = FunctionBinding::new(functions.as_ref(), &context, y);

    // Describe node.
    y.set_data_type(binding.return_type);
    describe!(y, Expression, "{}", binding.expression);
    summary!(y, "Scalar function call: {:#}", binding.expression);
    Ok(binding.expression)
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
    let functions = proto_primitive_field!(
        x,
        y,
        function_reference,
        extensions::simple::parse_function_reference
    )
    .1;
    #[allow(deprecated)]
    let legacy_arguments = proto_repeated_field!(x, y, args, parse_legacy_function_argument)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let arguments = proto_repeated_field!(x, y, arguments, parse_function_argument)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let return_type = proto_required_field!(x, y, output_type, types::parse_type)
        .0
        .data_type();

    // Parse modifiers.
    // TODO: these should be checked and passed to FunctionType::Window for
    // verification during the binding!
    proto_repeated_field!(x, y, partitions, expressions::parse_expression);
    proto_repeated_field!(x, y, sorts, sorts::parse_sort_field);
    proto_field!(x, y, upper_bound, parse_bound);
    proto_field!(x, y, lower_bound, parse_bound);
    proto_enum_field!(x, y, phase, substrait::AggregationPhase);

    // Try to bind the function.
    let arguments = handle_legacy_arguments(y, arguments, legacy_arguments);
    let context = FunctionContext {
        function_type: FunctionType::Window,
        arguments,
        return_type,
    };
    let binding = FunctionBinding::new(functions.as_ref(), &context, y);

    // Describe node.
    y.set_data_type(binding.return_type);
    describe!(y, Expression, "{}", binding.expression);
    summary!(y, "Window function call: {:#}", binding.expression);
    Ok(binding.expression)
}

/// Parse an aggregate function. Returns a description of the function call
/// expression.
pub fn parse_aggregate_function(
    x: &substrait::AggregateFunction,
    y: &mut context::Context,
) -> diagnostic::Result<expressions::Expression> {
    // Parse function information.
    let functions = proto_primitive_field!(
        x,
        y,
        function_reference,
        extensions::simple::parse_function_reference
    )
    .1;
    #[allow(deprecated)]
    let legacy_arguments = proto_repeated_field!(x, y, args, parse_legacy_function_argument)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let arguments = proto_repeated_field!(x, y, arguments, parse_function_argument)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let return_type = proto_required_field!(x, y, output_type, types::parse_type)
        .0
        .data_type();

    // Parse modifiers.
    // TODO: these should be checked and passed to FunctionType::Aggregate for
    // verification during the binding!
    proto_repeated_field!(x, y, sorts, sorts::parse_sort_field);
    proto_enum_field!(x, y, phase, substrait::AggregationPhase);
    proto_enum_field!(
        x,
        y,
        invocation,
        substrait::aggregate_function::AggregationInvocation
    );

    // Try to bind the function.
    let arguments = handle_legacy_arguments(y, arguments, legacy_arguments);
    let context = FunctionContext {
        function_type: FunctionType::Aggregate,
        arguments,
        return_type,
    };
    let binding = FunctionBinding::new(functions.as_ref(), &context, y);

    // Describe node.
    y.set_data_type(binding.return_type);
    describe!(y, Expression, "{}", binding.expression);
    summary!(y, "Aggregate function call: {:#}", binding.expression);
    Ok(binding.expression)
}
