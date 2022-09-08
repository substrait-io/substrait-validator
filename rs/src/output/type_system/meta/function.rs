// SPDX-License-Identifier: Apache-2.0

//! Module for builtin [`Function`]s that operate on [`meta::Value`]s.

use crate::output::diagnostic;
use crate::output::type_system::meta;

use super::Pattern;

/// A function that operates on zero or more values.
#[derive(Clone, Debug, PartialEq, Eq, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Function {
    /// Used for unknown functions. Takes any number of arguments, doesn't
    /// evaluate them, and yields an unresolved value.
    #[strum(serialize = "!")]
    Unresolved,

    /// Boolean not: `not(metabool) -> metabool`
    Not,

    /// Boolean and: `and(metabool*) -> metabool`. Evaluated lazily from left
    /// to right.
    And,

    /// Boolean or: `or(metabool*) -> metabool`. Evaluated lazily from left to
    /// right.
    Or,

    /// Integer negate: `negate(metaint) -> metaint`
    Negate,

    /// Integer sum: `add(metaint*) -> metaint`
    Add,

    /// Integer subtraction: `subtract(metaint, metaint) -> metaint`
    Subtract,

    /// Integer product: `multiply(metaint*) -> metaint`
    Multiply,

    /// Integer division: `divide(metaint, metaint) -> metaint`
    Divide,

    /// Integer minimum: `min(metaint+) -> metaint`
    Min,

    /// Integer maximum: `max(metaint+) -> metaint`
    Max,

    /// Equality: `equal(T, T) -> metabool`
    Equal,

    /// Inequality: `not_equal(T, T) -> metabool`
    NotEqual,

    /// Integer greater than: `greater_than(metaint, metaint) -> metabool`
    GreaterThan,

    /// Integer less than: `less_than(metaint, metaint) -> metabool`
    LessThan,

    /// Integer greater or equal: `greater_equal(metaint, metaint) -> metabool`
    GreaterEqual,

    /// Integer less or equal: `less_equal(metaint, metaint) -> metabool`
    LessEqual,

    /// Coverage: `covers(value, pattern) -> metabool`
    /// This matches the RHS pattern against the LHS value, and returns whether
    /// the match was successful. Note that names used in the pattern are NOT
    /// captured, regardless of whether the pattern matches.
    Covers,

    /// If/then/else: `if_then_else(metabool, T, T) -> T`. Evaluated lazily.
    IfThenElse,
}

impl Default for Function {
    fn default() -> Self {
        Function::Unresolved
    }
}

impl Function {
    /// Evaluates this function.
    pub fn evaluate(
        &self,
        context: &mut meta::Context,
        args: &[meta::pattern::Value],
    ) -> diagnostic::Result<meta::Value> {
        match self {
            Function::Unresolved => Ok(meta::Value::Unresolved),
            Function::Not => {
                if args.len() != 1 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects a single argument"
                    ))
                } else if let Some(value) = args[0].evaluate_with_context(context)?.get_boolean() {
                    Ok((!value).into())
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() can only be applied to metabools"
                    ))
                }
            }
            Function::And => {
                for arg in args.iter() {
                    match arg.evaluate_with_context(context)?.get_boolean() {
                        Some(true) => continue,
                        Some(false) => return Ok(false.into()),
                        None => {
                            return Err(cause!(
                                TypeDerivationInvalid,
                                "{self}() can only be applied to metabools"
                            ))
                        }
                    }
                }
                Ok(true.into())
            }
            Function::Or => {
                for arg in args.iter() {
                    match arg.evaluate_with_context(context)?.get_boolean() {
                        Some(false) => continue,
                        Some(true) => return Ok(true.into()),
                        None => {
                            return Err(cause!(
                                TypeDerivationInvalid,
                                "{self}() can only be applied to metabools"
                            ))
                        }
                    }
                }
                Ok(false.into())
            }
            Function::Negate => {
                if args.len() != 1 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects a single argument"
                    ))
                } else if let Some(value) = args[0].evaluate_with_context(context)?.get_integer() {
                    if let Some(value) = value.checked_neg() {
                        Ok(value.into())
                    } else {
                        Err(cause!(TypeDerivationFailed, "integer overflow in {self}()"))
                    }
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() can only be applied to metaints"
                    ))
                }
            }
            Function::Add => {
                let mut accumulator = 0i64;
                for arg in args.iter() {
                    if let Some(value) = arg.evaluate_with_context(context)?.get_integer() {
                        accumulator = accumulator.checked_add(value).ok_or_else(|| {
                            cause!(TypeDerivationFailed, "integer overflow in {self}()")
                        })?;
                    } else {
                        return Err(cause!(
                            TypeDerivationInvalid,
                            "{self}() can only be applied to metaints"
                        ));
                    }
                }
                Ok(accumulator.into())
            }
            Function::Subtract => {
                if args.len() != 2 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly two arguments"
                    ))
                } else if let (Some(lhs), Some(rhs)) = (
                    args[0].evaluate_with_context(context)?.get_integer(),
                    args[1].evaluate_with_context(context)?.get_integer(),
                ) {
                    if let Some(value) = lhs.checked_sub(rhs) {
                        Ok(value.into())
                    } else {
                        Err(cause!(TypeDerivationFailed, "integer overflow in {self}()"))
                    }
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() can only be applied to metaints"
                    ))
                }
            }
            Function::Multiply => {
                let mut accumulator = 1i64;
                for arg in args.iter() {
                    if let Some(value) = arg.evaluate_with_context(context)?.get_integer() {
                        accumulator = accumulator.checked_mul(value).ok_or_else(|| {
                            cause!(TypeDerivationFailed, "integer overflow in {self}()")
                        })?;
                    } else {
                        return Err(cause!(
                            TypeDerivationInvalid,
                            "{self}() can only be applied to metaints"
                        ));
                    }
                }
                Ok(accumulator.into())
            }
            Function::Divide => {
                if args.len() != 2 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly two arguments"
                    ))
                } else if let (Some(lhs), Some(rhs)) = (
                    args[0].evaluate_with_context(context)?.get_integer(),
                    args[1].evaluate_with_context(context)?.get_integer(),
                ) {
                    if let Some(value) = lhs.checked_div(rhs) {
                        Ok(value.into())
                    } else {
                        Err(cause!(TypeDerivationFailed, "division by zero in {self}()"))
                    }
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() can only be applied to metaints"
                    ))
                }
            }
            Function::Min => {
                if args.is_empty() {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects at least one argument"
                    ))
                } else {
                    let mut accumulator = i64::MAX;
                    for arg in args.iter() {
                        if let Some(value) = arg.evaluate_with_context(context)?.get_integer() {
                            if value < accumulator {
                                accumulator = value;
                            }
                        } else {
                            return Err(cause!(
                                TypeDerivationInvalid,
                                "{self}() can only be applied to metaints"
                            ));
                        }
                    }
                    Ok(accumulator.into())
                }
            }
            Function::Max => {
                if args.is_empty() {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects at least one argument"
                    ))
                } else {
                    let mut accumulator = i64::MIN;
                    for arg in args.iter() {
                        if let Some(value) = arg.evaluate_with_context(context)?.get_integer() {
                            if value > accumulator {
                                accumulator = value;
                            }
                        } else {
                            return Err(cause!(
                                TypeDerivationInvalid,
                                "{self}() can only be applied to metaints"
                            ));
                        }
                    }
                    Ok(accumulator.into())
                }
            }
            Function::Equal => {
                if args.len() != 2 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly two arguments"
                    ))
                } else {
                    let lhs = args[0].evaluate_with_context(context)?;
                    let rhs = args[1].evaluate_with_context(context)?;
                    Ok((lhs == rhs).into())
                }
            }
            Function::NotEqual => {
                if args.len() != 2 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly two arguments"
                    ))
                } else {
                    let lhs = args[0].evaluate_with_context(context)?;
                    let rhs = args[1].evaluate_with_context(context)?;
                    Ok((lhs != rhs).into())
                }
            }
            Function::GreaterThan => {
                if args.len() != 2 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly two arguments"
                    ))
                } else if let (Some(lhs), Some(rhs)) = (
                    args[0].evaluate_with_context(context)?.get_integer(),
                    args[1].evaluate_with_context(context)?.get_integer(),
                ) {
                    Ok((lhs > rhs).into())
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() can only be applied to metaints"
                    ))
                }
            }
            Function::LessThan => {
                if args.len() != 2 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly two arguments"
                    ))
                } else if let (Some(lhs), Some(rhs)) = (
                    args[0].evaluate_with_context(context)?.get_integer(),
                    args[1].evaluate_with_context(context)?.get_integer(),
                ) {
                    Ok((lhs < rhs).into())
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() can only be applied to metaints"
                    ))
                }
            }
            Function::GreaterEqual => {
                if args.len() != 2 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly two arguments"
                    ))
                } else if let (Some(lhs), Some(rhs)) = (
                    args[0].evaluate_with_context(context)?.get_integer(),
                    args[1].evaluate_with_context(context)?.get_integer(),
                ) {
                    Ok((lhs >= rhs).into())
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() can only be applied to metaints"
                    ))
                }
            }
            Function::LessEqual => {
                if args.len() != 2 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly two arguments"
                    ))
                } else if let (Some(lhs), Some(rhs)) = (
                    args[0].evaluate_with_context(context)?.get_integer(),
                    args[1].evaluate_with_context(context)?.get_integer(),
                ) {
                    Ok((lhs <= rhs).into())
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() can only be applied to metaints"
                    ))
                }
            }
            Function::Covers => {
                if args.len() != 2 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly two arguments"
                    ))
                } else {
                    let value = args[0].evaluate_with_context(context)?;
                    // It's possible for a match to capture bindings even if it
                    // fails in the end, in case of a partial match. However, a
                    // failing covers() call should not capture anything. So we
                    // have to make a copy of the context here.
                    let mut context_copy = context.clone();
                    Ok(
                        if args[1].match_pattern_with_context(&mut context_copy, &value)? {
                            *context = context_copy;
                            true
                        } else {
                            false
                        }
                        .into(),
                    )
                }
            }
            Function::IfThenElse => {
                if args.len() != 3 {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "{self}() expects exactly three arguments"
                    ))
                } else if let Some(condition) =
                    args[0].evaluate_with_context(context)?.get_boolean()
                {
                    args[if condition { 1 } else { 2 }].evaluate_with_context(context)
                } else {
                    Err(cause!(
                        TypeDerivationInvalid,
                        "the first argument of {self}() must be a metabool"
                    ))
                }
            }
        }
    }

    /// Returns what type this function evaluates to. If unknown or multiple
    /// types can be matched, yield unresolved.
    pub fn determine_type(&self, arguments: &[meta::pattern::Value]) -> meta::Type {
        match self {
            Function::Unresolved => meta::Type::Unresolved,
            Function::Not
            | Function::And
            | Function::Or
            | Function::Equal
            | Function::NotEqual
            | Function::GreaterThan
            | Function::LessThan
            | Function::GreaterEqual
            | Function::LessEqual
            | Function::Covers => meta::Type::Boolean,
            Function::Negate
            | Function::Add
            | Function::Subtract
            | Function::Multiply
            | Function::Divide
            | Function::Min
            | Function::Max => meta::Type::Integer,
            Function::IfThenElse => {
                if arguments.len() == 3 {
                    let a = arguments[1].determine_type();
                    let b = arguments[2].determine_type();
                    if a == b {
                        a
                    } else {
                        meta::Type::Unresolved
                    }
                } else {
                    meta::Type::Unresolved
                }
            }
        }
    }
}
