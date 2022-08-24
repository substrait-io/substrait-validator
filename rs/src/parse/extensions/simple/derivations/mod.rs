// SPDX-License-Identifier: Apache-2.0

//! Module for parsing type derivations using ANTLR.

// TODO
#![allow(dead_code)]

mod substraittypelexer;
mod substraittypelistener;
mod substraittypeparser;

use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::output::type_system::meta::Pattern;
use crate::parse::context;
use antlr_rust::Parser;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use substraittypeparser::*;

/// Resolves an identifier path used in pattern scope.
fn resolve_pattern_identifier<S, I>(
    x: I,
    y: &mut context::Context,
) -> Result<context::IdentifiedObject>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    // Reconstruct the full identifier path for use in error messages.
    let path = x.collect::<Vec<_>>();
    let ident = path.iter().map(|x| x.as_ref()).join(".");
    let mut path = path.into_iter();

    // Resolve the first element.
    let mut object = y
        .type_ident_map_resolve(
            path.next()
                .ok_or_else(|| cause!(TypeResolutionError, "empty identifier path"))?,
        )
        .clone();

    // Resolve the rest of the elements iteratively.
    for element in path {
        object = match object {
            context::IdentifiedObject::NamedDependency(dep) => {
                if let extension::YamlInfo::Resolved(dep) = dep {
                    let reference = dep.resolve_type(element.as_ref());
                    if reference.definition.is_none() {
                        diagnostic!(y, Error, TypeResolutionError, "could not resolve {ident}: type class {} is not defined in this scope", element.as_ref());
                    }
                    Ok(context::IdentifiedObject::TypeClass(data::Class::UserDefined(reference)))
                } else {
                    diagnostic!(y, Warning, TypeResolutionError, "could not resolve {ident}: the extension it should be defined in could not be resolved");
                    Ok(context::IdentifiedObject::TypeClass(data::Class::Unresolved))
                }
            }
            context::IdentifiedObject::Binding(_) => Err(cause!(TypeResolutionError, "could not resolve {ident}: prefix resolves to a binding, which does not have members")),
            context::IdentifiedObject::EnumLiteral(_) => Err(cause!(TypeResolutionError, "could not resolve {ident}: prefix resolves to an enum parameter literal, which does not have members")),
            context::IdentifiedObject::TypeClass(_) => Err(cause!(TypeResolutionError, "could not resolve {ident}: prefix resolves to a type class, which does not have members")),
        }?;
    }

    Ok(object)
}

/// Error listener that just collects error messages into a vector, such that
/// they can be obtained when parsing completes.
#[derive(Default, Clone)]
struct ErrorListener {
    messages: Rc<RefCell<Vec<String>>>,
}

impl<'a, T: antlr_rust::recognizer::Recognizer<'a>> antlr_rust::error_listener::ErrorListener<'a, T>
    for ErrorListener
{
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<<T>::TF as antlr_rust::token_factory::TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&antlr_rust::errors::ANTLRError>,
    ) {
        self.messages
            .borrow_mut()
            .push(format!("at {line}:{column}: {msg}"));
    }
}

impl ErrorListener {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_context(&self, y: &mut context::Context) {
        for message in self.messages.borrow_mut().drain(..) {
            diagnostic!(y, Error, TypeParseError, "{message}");
        }
    }
}

// Boilerplate code for connecting ANTLR to our diagnostic system and parsing
// a simple string slice with it.
macro_rules! antlr_parse {
    ($x:expr, $y:expr, $start:ident) => {{
        let lexer = substraittypelexer::SubstraitTypeLexer::new(antlr_rust::InputStream::new($x));
        let token_source = antlr_rust::common_token_stream::CommonTokenStream::new(lexer);
        let mut parser = SubstraitTypeParser::new(token_source);
        let listener = ErrorListener::new();
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(listener.clone()));
        let result = parser.$start();
        listener.to_context($y);
        result.map_err(|e| cause!(TypeParseError, "{e}"))
    }};
}

// Boilerplate code for converting the awkward left-recursion-avoidance rules
// for expressions into a normal expression tree.
macro_rules! antlr_reduce_left_recursion {
    ($x:expr, $y:expr, $x_typ:ty, $all_operands:ident, $next_analyzer:expr, $one_operator:ident, $operator_match:tt) => {{
        fn left_recursive(
            x: &$x_typ,
            y: &mut context::Context,
            start: usize,
        ) -> Result<meta::pattern::Value> {
            if start == 0 {
                // Only one operand remaining.
                Ok(antlr_hidden_child!(x, y, $next_analyzer).unwrap_or_default())
            } else {
                // We're traversing the tree bottom-up, so start with the last
                // operation. The operations are evaluated left-to-right, so that's
                // the rightmost operation.
                let lhs = antlr_recurse!(x, y, lhs, left_recursive, start - 1)
                    .1
                    .unwrap_or_default();
                let rhs = antlr_child!(x, y, rhs, start, $next_analyzer)
                    .1
                    .unwrap_or_default();
                let function = x.$one_operator(start - 1).map(|x| match x.as_ref() $operator_match).unwrap_or_default();
                Ok(meta::pattern::Value::Function(function, vec![lhs, rhs]))
            }
        }

        let total_operands = $x.$all_operands().len();
        left_recursive($x, $y, total_operands - 1)
    }};
}

/// Analyzes a string literal.
fn analyze_string<S: AsRef<str>>(x: S, _y: &mut context::Context) -> Option<String> {
    let x = x.as_ref();
    if !x.starts_with('"') || !x.ends_with('"') || x.len() < 2 {
        None
    } else {
        Some(x[1..x.len() - 1].to_string())
    }
}

/// Analyzes an integer literal.
fn analyze_integer(x: &IntegerContextAll, y: &mut context::Context) -> Result<i64> {
    let value = if let Some(value) = x.Nonzero() {
        value.symbol.text.parse().unwrap_or(i128::MAX)
    } else {
        0i128
    };
    Ok(if x.Minus().is_some() {
        match i64::try_from(-value) {
            Ok(val) => val,
            Err(_) => {
                diagnostic!(
                    y,
                    Error,
                    TypeDerivationInvalid,
                    "integer literal is too small, minimum is {}",
                    i64::MIN
                );
                i64::MIN
            }
        }
    } else {
        match i64::try_from(-value) {
            Ok(val) => val,
            Err(_) => {
                diagnostic!(
                    y,
                    Error,
                    TypeDerivationInvalid,
                    "integer literal is too large, maximum is {}",
                    i64::MAX
                );
                i64::MAX
            }
        }
    })
}

/// Analyzes and resolves an identifier path.
fn analyze_object(
    x: &IdentifierPathContextAll,
    y: &mut context::Context,
) -> Result<context::IdentifiedObject> {
    resolve_pattern_identifier(
        x.Identifier_all().iter().map(|x| x.symbol.text.to_string()),
        y,
    )
}

/// Analyzes miscellaneous pattern types.
fn analyze_pattern_misc(
    x: &PatternMiscContextAll,
    y: &mut context::Context,
) -> Result<meta::pattern::Value> {
    match x {
        PatternMiscContextAll::ParenthesesContext(x) => {
            Ok(antlr_hidden_child!(x, y, analyze_pattern).unwrap_or_default())
        }
        PatternMiscContextAll::IfThenElseContext(x) => {
            let condition = antlr_child!(x, y, condition, 0, analyze_pattern)
                .1
                .unwrap_or_default();
            let if_true = antlr_child!(x, y, if_true, 1, analyze_pattern)
                .1
                .unwrap_or_default();
            let if_false = antlr_child!(x, y, if_false, 2, analyze_pattern)
                .1
                .unwrap_or_default();
            Ok(meta::pattern::Value::Function(
                meta::Function::IfThenElse,
                vec![condition, if_true, if_false],
            ))
        }
        PatternMiscContextAll::UnaryNotContext(x) => {
            let expression = antlr_child!(x, y, expression, analyze_pattern)
                .1
                .unwrap_or_default();
            Ok(meta::pattern::Value::Function(
                meta::Function::Not,
                vec![expression],
            ))
        }
        PatternMiscContextAll::UnaryNegateContext(x) => {
            let expression = antlr_child!(x, y, expression, analyze_pattern)
                .1
                .unwrap_or_default();
            Ok(meta::pattern::Value::Function(
                meta::Function::Negate,
                vec![expression],
            ))
        }
        PatternMiscContextAll::AnyContext(_) => Ok(meta::pattern::Value::Any),
        PatternMiscContextAll::BoolAnyContext(_) => Ok(meta::pattern::Value::Boolean(None)),
        PatternMiscContextAll::BoolTrueContext(_) => Ok(meta::pattern::Value::Boolean(Some(true))),
        PatternMiscContextAll::BoolFalseContext(_) => {
            Ok(meta::pattern::Value::Boolean(Some(false)))
        }
        PatternMiscContextAll::IntAnyContext(_) => {
            Ok(meta::pattern::Value::Integer(i64::MIN, i64::MAX))
        }
        PatternMiscContextAll::IntRangeContext(x) => {
            let lower = antlr_hidden_child!(x, y, 0, analyze_integer).unwrap_or(i64::MIN);
            let upper = antlr_hidden_child!(x, y, 1, analyze_integer).unwrap_or(i64::MAX);
            if lower > upper {
                diagnostic!(
                    y,
                    Error,
                    TypeDerivationInvalid,
                    "lower bound of range is greater than upper bound"
                );
            }
            Ok(meta::pattern::Value::Integer(lower, upper))
        }
        PatternMiscContextAll::IntAtMostContext(x) => {
            let upper = antlr_hidden_child!(x, y, analyze_integer).unwrap_or(i64::MAX);
            Ok(meta::pattern::Value::Integer(i64::MIN, upper))
        }
        PatternMiscContextAll::IntAtLeastContext(x) => {
            let lower = antlr_hidden_child!(x, y, analyze_integer).unwrap_or(i64::MAX);
            Ok(meta::pattern::Value::Integer(lower, i64::MAX))
        }
        PatternMiscContextAll::IntExactlyContext(x) => {
            let value = antlr_hidden_child!(x, y, analyze_integer).unwrap_or(i64::MAX);
            Ok(meta::pattern::Value::Integer(value, value))
        }
        PatternMiscContextAll::EnumAnyContext(_) => Ok(meta::pattern::Value::Enum(None)),
        PatternMiscContextAll::EnumSetContext(x) => {
            let names = x
                .Identifier_all()
                .iter()
                .map(|x| x.symbol.text.to_string())
                .collect::<Vec<_>>();
            let mut unique_names = HashSet::new();
            let mut repeated_names = HashSet::new();
            for name in names.iter() {
                if !unique_names.insert(name.to_ascii_uppercase()) {
                    repeated_names.insert(name.to_ascii_uppercase());
                }
            }
            if !repeated_names.is_empty() {
                diagnostic!(
                    y,
                    Error,
                    RedundantEnumVariant,
                    "enumeration variant names should be case-insensitively unique: {}",
                    repeated_names.iter().join(", ")
                );
            }
            Ok(meta::pattern::Value::Enum(Some(names)))
        }
        PatternMiscContextAll::StrAnyContext(_) => Ok(meta::pattern::Value::String(None)),
        PatternMiscContextAll::StrExactlyContext(x) => {
            let s = x.String().and_then(|x| analyze_string(&x.symbol.text, y));
            if s.is_none() {
                diagnostic!(y, Error, TypeParseError, "invalid string literal");
            }
            Ok(meta::pattern::Value::String(s))
        }
        PatternMiscContextAll::DtAnyContext(_) => Ok(meta::pattern::Value::DataType(None)),
        PatternMiscContextAll::FunctionContext(x) => {
            let function = x
                .Identifier()
                .and_then(|x| meta::Function::try_from(x.symbol.text.as_ref()).ok());
            if function.is_none() {
                diagnostic!(y, Error, TypeParseError, "unknown function");
            }
            let arguments = antlr_children!(x, y, argument, analyze_pattern)
                .1
                .into_iter()
                .map(|x| x.unwrap_or_default())
                .collect();
            Ok(meta::pattern::Value::Function(
                function.unwrap_or_default(),
                arguments,
            ))
        }
        PatternMiscContextAll::DatatypeBindingOrConstantContext(x) => {
            let object = antlr_hidden_child!(x, y, analyze_object)
                .ok_or_else(|| cause!(TypeDerivationInvalid, "failed to resolve identifier"))?;
            match object {
                context::IdentifiedObject::Binding(name) => {
                    if x.variation().is_some() {
                        diagnostic!(
                            y,
                            Error,
                            TypeParseError,
                            "variation cannot be specified for bindings"
                        );
                    }
                    if x.parameters().is_some() {
                        diagnostic!(
                            y,
                            Error,
                            TypeParseError,
                            "parameters cannot be specified for bindings"
                        );
                    }
                    if let Some(qst) = x.nullability() {
                        if qst.pattern().is_some() {
                            diagnostic!(
                                y,
                                Error,
                                TypeParseError,
                                "nullability pattern cannot be specified for bindings"
                            );
                        }
                        Ok(meta::pattern::Value::ImplicitOrBinding(name))
                    } else {
                        Ok(meta::pattern::Value::Binding(name))
                    }
                }
                context::IdentifiedObject::EnumLiteral(name) => {
                    if x.nullability().is_some() {
                        diagnostic!(
                            y,
                            Error,
                            TypeParseError,
                            "nullability cannot be specified for enum literals"
                        );
                    }
                    if x.variation().is_some() {
                        diagnostic!(
                            y,
                            Error,
                            TypeParseError,
                            "variation cannot be specified for enum literals"
                        );
                    }
                    if x.parameters().is_some() {
                        diagnostic!(
                            y,
                            Error,
                            TypeParseError,
                            "parameters cannot be specified for enum literals"
                        );
                    }
                    Ok(meta::pattern::Value::Enum(Some(vec![name])))
                }
                context::IdentifiedObject::NamedDependency(_) => Err(cause!(
                    TypeDerivationInvalid,
                    "identifier resolves to dependency namespace, which cannot be used as such"
                )),
                context::IdentifiedObject::TypeClass(class) => {
                    let nullable = if let Some(qst) = x.nullability() {
                        if let Some(pattern) =
                            antlr_child!(qst.as_ref(), y, nullability, analyze_pattern).1
                        {
                            pattern
                        } else {
                            meta::pattern::Value::Boolean(Some(true))
                        }
                    } else {
                        meta::pattern::Value::Boolean(Some(false))
                    };
                    if x.variation().is_some() {
                        todo!()
                    }
                    if x.parameters().is_some() {
                        todo!()
                    }
                    Ok(meta::pattern::Value::DataType(Some(
                        meta::pattern::DataType {
                            class,
                            nullable: std::sync::Arc::new(nullable),
                            variation: meta::pattern::Variation::Any,
                            parameters: None,
                        },
                    )))
                }
            }
        }
        PatternMiscContextAll::Error(_) => Ok(meta::pattern::Value::Unresolved),
    }
}

/// Analyzes a set of zero or more a*b or a/b expressions.
fn analyze_pattern_mul_div(
    x: &PatternMulDivContextAll,
    y: &mut context::Context,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, PatternMulDivContextAll,
        patternMisc_all, analyze_pattern_misc, operatorMulDiv,
        {
            OperatorMulDivContextAll::MulContext(_) => meta::Function::Multiply,
            OperatorMulDivContextAll::DivContext(_) => meta::Function::Divide,
            OperatorMulDivContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more a+b or a-b expressions.
fn analyze_pattern_add_sub(
    x: &PatternAddSubContextAll,
    y: &mut context::Context,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, PatternAddSubContextAll,
        patternMulDiv_all, analyze_pattern_mul_div, operatorAddSub,
        {
            OperatorAddSubContextAll::AddContext(_) => meta::Function::Add,
            OperatorAddSubContextAll::SubContext(_) => meta::Function::Subtract,
            OperatorAddSubContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more integer inequality expressions.
fn analyze_pattern_ineq(
    x: &PatternIneqContextAll,
    y: &mut context::Context,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, PatternIneqContextAll,
        patternAddSub_all, analyze_pattern_add_sub, operatorIneq,
        {
            OperatorIneqContextAll::LtContext(_) => meta::Function::LessThan,
            OperatorIneqContextAll::LeContext(_) => meta::Function::LessEqual,
            OperatorIneqContextAll::GtContext(_) => meta::Function::GreaterThan,
            OperatorIneqContextAll::GeContext(_) => meta::Function::GreaterEqual,
            OperatorIneqContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more x==y or x!=y expressions.
fn analyze_pattern_eq_neq(
    x: &PatternEqNeqContextAll,
    y: &mut context::Context,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, PatternEqNeqContextAll,
        patternIneq_all, analyze_pattern_ineq, operatorEqNeq,
        {
            OperatorEqNeqContextAll::EqContext(_) => meta::Function::Equal,
            OperatorEqNeqContextAll::NeqContext(_) => meta::Function::NotEqual,
            OperatorEqNeqContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more x&&y expressions.
fn analyze_pattern_and(
    x: &PatternAndContextAll,
    y: &mut context::Context,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, PatternAndContextAll,
        patternEqNeq_all, analyze_pattern_eq_neq, operatorAnd,
        {
            OperatorAndContextAll::AndContext(_) => meta::Function::And,
            OperatorAndContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a set of zero or more x||y expressions.
fn analyze_pattern_or(
    x: &PatternOrContextAll,
    y: &mut context::Context,
) -> Result<meta::pattern::Value> {
    antlr_reduce_left_recursion!(
        x, y, PatternOrContextAll,
        patternAnd_all, analyze_pattern_and, operatorOr,
        {
            OperatorOrContextAll::OrContext(_) => meta::Function::Or,
            OperatorOrContextAll::Error(_) => meta::Function::Unresolved,
        }
    )
}

/// Analyzes a pattern parse tree node.
fn analyze_pattern(
    x: &PatternContextAll,
    y: &mut context::Context,
) -> Result<meta::pattern::Value> {
    Ok(antlr_hidden_child!(x, y, analyze_pattern_or).unwrap_or_default())
}

/// Analyzes a statement parse tree node.
fn analyze_statement(
    x: &StatementContextAll,
    y: &mut context::Context,
) -> Result<meta::program::Statement> {
    match x {
        StatementContextAll::AssertContext(x) => {
            let rhs_expression = antlr_child!(x, y, rhs, analyze_pattern)
                .1
                .unwrap_or_default();
            Ok(meta::program::Statement {
                lhs_pattern: meta::pattern::Value::Boolean(Some(true)),
                rhs_expression,
            })
        }
        StatementContextAll::NormalContext(x) => {
            let rhs_expression = antlr_child!(x, y, rhs, 1, analyze_pattern)
                .1
                .unwrap_or_default();
            let lhs_pattern = antlr_child!(x, y, lhs, 0, analyze_pattern)
                .1
                .unwrap_or_default();
            Ok(meta::program::Statement {
                lhs_pattern,
                rhs_expression,
            })
        }
        StatementContextAll::MatchContext(x) => {
            let rhs_expression = antlr_child!(x, y, rhs, 0, analyze_pattern)
                .1
                .unwrap_or_default();
            let lhs_pattern = antlr_child!(x, y, lhs, 1, analyze_pattern)
                .1
                .unwrap_or_default();
            Ok(meta::program::Statement {
                lhs_pattern,
                rhs_expression,
            })
        }
        StatementContextAll::Error(_) => Ok(meta::program::Statement::default()),
    }
}

/// Analyzes a program parse tree node.
fn analyze_program(x: &ProgramContextAll, y: &mut context::Context) -> Result<meta::Program> {
    let statements = antlr_children!(x, y, statement, analyze_statement)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    let expression = antlr_child!(x, y, pattern, analyze_pattern)
        .1
        .unwrap_or_default();
    Ok(meta::Program {
        statements,
        expression,
    })
}

/// Parse a string as just the class part of a data type.
pub fn parse_class(x: &str, y: &mut context::Context) -> Result<data::Class> {
    // Resolve from within a fresh scope.
    y.type_ident_map_init();
    let result = resolve_pattern_identifier(x.split('.'), y);
    y.type_ident_map_clear();
    let object = result?;

    // Only accept type classes.
    if let context::IdentifiedObject::TypeClass(class) = object {
        Ok(class)
    } else {
        Err(cause!(
            TypeResolutionError,
            "could not resolve {x} as a type class"
        ))
    }
}

/// Parse a string as a complete type.
pub fn parse_type(x: &str, y: &mut context::Context) -> Result<data::Type> {
    let pattern = parse_pattern(x, y)?;
    let value = pattern.evaluate()?;
    value.get_data_type().ok_or_else(|| {
        cause!(
            TypeDerivationInvalid,
            "expected a data type, but received a pattern that evaluated to {value}"
        )
    })
}

/// Parse a string as a meta-pattern.
pub fn parse_pattern(x: &str, y: &mut context::Context) -> Result<meta::pattern::Value> {
    let x = antlr_parse!(x, y, startPattern)?;
    y.type_ident_map_init();
    let result = antlr_child!(x.as_ref(), y, pattern, analyze_pattern)
        .1
        .unwrap_or_default();
    y.type_ident_map_clear();
    Ok(result)
}

/// Parse a string as a meta-program.
pub fn parse_program(x: &str, y: &mut context::Context) -> Result<meta::Program> {
    let x = antlr_parse!(x, y, startProgram)?;
    y.type_ident_map_init();
    let result = antlr_child!(x.as_ref(), y, program, analyze_program)
        .1
        .unwrap_or_default();
    y.type_ident_map_clear();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::output::tree;

    #[test]
    fn test() {
        let mut node = tree::Node::from(tree::NodeType::ProtoMessage("test"));
        let mut state = Default::default();
        let config = crate::Config::new();
        let mut context = context::Context::new("test", &mut node, &mut state, &config);

        /*let result = parse_program(
            r#"init_scale = max(S1,S2)
            init_prec = init_scale + max(P1 - S1, P2 - S2) + 1
            min_scale = min(init_scale, 6)
            delta = init_prec - 38
            prec = min(init_prec, 38)
            scale_after_borrow = max(init_scale - delta, min_scale)
            scale = if init_prec > 38 then scale_after_borrow else init_scale
            DECIMAL<prec, scale>"#,
            &mut context,
        )
        .ok();*/

        let _result = parse_program(r#"1 + 2 * 3 - 4 / 5"#, &mut context).unwrap_or_default();

        //let mut eval_context = meta::Context::default();
        //panic!("{:#?}", result.evaluate(&mut eval_context));
        //panic!("{:#?}", result);
        //panic!("{node:#?}");
        //panic!("{:#?}", result.to_string_tree(&*parser));
    }
}
