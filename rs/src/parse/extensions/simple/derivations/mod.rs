// SPDX-License-Identifier: Apache-2.0

//! Module for parsing type derivations using ANTLR.

// TODO
#![allow(dead_code)]

mod substraittypelexer;
mod substraittypelistener;
mod substraittypeparser;

//use antlr_rust::parser_rule_context::ParserRuleContext;
//use antlr_rust::tree::ParseTree;
use antlr_rust::Parser;
//use substraittypeparser::ProgramContextAttrs;
//use substraittypeparser::StartProgramContextAttrs;

//use std::sync::Arc;

/// Error listener that just collects error messages into a vector, such that
/// they can be obtained when parsing completes.
#[derive(Default, Clone)]
struct ErrorList {
    messages: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
}

impl<'a, T: antlr_rust::recognizer::Recognizer<'a>> antlr_rust::error_listener::ErrorListener<'a, T>
    for ErrorList
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
        self.push(format!("at {line}:{column}: {msg}"));
    }
}

impl ErrorList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<S: ToString>(&self, msg: S) {
        self.messages.borrow_mut().push(msg.to_string());
    }

    pub fn get(&self) -> Vec<String> {
        self.messages.borrow_mut().drain(..).collect()
    }
}
/*
/// AST node for derivation programs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    /// The statements, represented as (LHS, RHS) pairs.
    pub statements: Vec<(Pattern, Pattern)>,

    /// The final pattern that is to be evaluated.
    pub expression: Pattern,
}

/// AST node for patterns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pattern {
    /// Syntax: `?`.
    Any,

    /// Syntax:
    ///  - `metabool`: None.
    ///  - `true`: Some(true).
    ///  - `false`: Some(false).
    Boolean(Option<bool>),

    /// Syntax:
    ///  - `metaint`: (i64::MIN, i64::MAX).
    ///  - `<int>`: (x, x).
    ///  - `<int>..`: (x, i64::MAX).
    ///  - `..<int>`: (i64::MIN, x).
    ///  - `<int>..<int>`: (x, y).
    Integer(i64, i64),

    /// Syntax:
    ///  - `metaenum`: None.
    ///  - `{<identifier,+>}`: Some(options).
    Enum(Option<Vec<String>>),

    /// Syntax:
    ///  - `metastr`: None.
    ///  - quoted string: Some(x).
    String(Option<String>),

    /// Syntax:
    ///  - `typename`: None.
    ///  - Some(_): syntax of a data type, with arbitrary identifier path for
    ///    its class.
    DataTypeLike(Option<DataTypeLike>),

    /// Syntax: `<function-name>(<args>, ...)`, as well as expression-style
    /// syntactic sugar.
    Function(String, Vec<Pattern>),
}

/// AST node for things that look like data types, but may also be bindings or
/// enum constants (depends on name resolution).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTypeLike {
    /// Period-separated identifier path for the class, unless it resolves to
    /// an enum constant or named binding.
    pub class: Vec<String>,

    /// Syntax:
    ///  - no suffix: Boolean(Some(false)).
    ///  - `?` suffix: Boolean(Some(true)).
    ///  - `?` followed by pattern: just the supplied pattern.
    pub nullable: Arc<Pattern>,

    /// Type variation pattern.
    pub variation: Variation,

    /// Syntax:
    ///  - no suffix: None.
    ///  - `<>`-enclosed list of parameters: Some(x).
    pub parameters: Option<Vec<Parameter>>,
}

/// AST node for type variation suffixes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Variation {
    /// Syntax: `[?]` suffix.
    Any,

    /// Syntax: no suffix.
    Compatible,

    /// Syntax: `[0]` suffix.
    SystemPreferred,

    /// Syntax: identifier path.
    UserDefined(Vec<String>),
}

/// AST node for type parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    /// Syntax:
    ///  - no prefix: None.
    ///  - `<identifierOrString>:` prefix: Some(x).
    pub name: Option<String>,

    /// Syntax:
    ///  - `null`: None.
    ///  - anything else: Some(x).
    pub value: Option<Pattern>,
}*/

fn parse_program(_x: &std::rc::Rc<substraittypeparser::StartProgramContext>) -> i32 {
    42
}

pub fn test() {
    println!("test started");
    let mut _lexer = substraittypelexer::SubstraitTypeLexer::new(antlr_rust::InputStream::new(
        r#"          init_scale = max(S1,S2)
    init_prec = init_scale + max(P1 - S1, P2 - S2) + 1
    min_scale = min(init_scale, 6)
    delta = init_prec - 38
    prec = min(init_prec, 38)
    scale_after_borrow = max(init_scale - delta, min_scale)
    scale = if init_prec > 38 then scale_after_borrow else init_scale
    DECIMAL<prec, scale>
"#,
    ));
    let token_source = antlr_rust::common_token_stream::CommonTokenStream::new(_lexer);
    let mut parser = substraittypeparser::SubstraitTypeParser::new(token_source);
    let listener = Box::new(ErrorList::new());
    parser.remove_error_listeners();
    parser.add_error_listener(listener.clone());
    let _result = match parser.startProgram() {
        Ok(result) => Some(parse_program(&result)),
        Err(error) => {
            listener.push(error);
            None
        }
    };
    //result.program().unwrap().;
    for msg in listener.get().into_iter() {
        panic!("{msg}");
    }
    //panic!("{:#?}", result.to_string_tree(&*parser));
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        super::test();
    }
}
