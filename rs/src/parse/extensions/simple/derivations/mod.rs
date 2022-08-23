// SPDX-License-Identifier: Apache-2.0

//! Module for parsing type derivations using ANTLR.

// TODO
#![allow(dead_code)]

use crate::output::tree;
use crate::parse::context;
use antlr_rust::Parser;
use std::cell::RefCell;
use std::rc::Rc;

mod substraittypelexer;
mod substraittypelistener;
mod substraittypeparser;

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

fn parse_program(_x: &std::rc::Rc<substraittypeparser::StartProgramContext>) -> i32 {
    42
}

pub fn test() {
    let mut node = tree::Node::from(tree::NodeType::ProtoMessage("test"));
    let mut state = Default::default();
    let config = crate::Config::new();
    let mut context = context::Context::new("test", &mut node, &mut state, &config);

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
    let listener = ErrorListener::new();
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(listener.clone()));
    let _result = parser.startProgram();
    listener.to_context(&mut context);

    //result.program().unwrap().;
    panic!("{node:?}");
    //panic!("{:#?}", result.to_string_tree(&*parser));
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        super::test();
    }
}
