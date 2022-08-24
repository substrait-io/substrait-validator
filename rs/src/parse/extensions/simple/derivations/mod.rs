// SPDX-License-Identifier: Apache-2.0

//! Module for parsing type derivations using ANTLR.

// TODO
#![allow(dead_code)]

use crate::output::diagnostic::Result;
use crate::output::extension;
use crate::output::type_system::data;
use crate::output::type_system::meta;
use crate::output::type_system::meta::Pattern;
use crate::parse::context;
use antlr_rust::Parser;
use itertools::Itertools;
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

// Boilerplate code for connecting ANTLR to our diagnostic system and parsing
// a simple string slice with it.
macro_rules! antlr_parse {
    ($x:expr, $y:expr, $start:ident) => {{
        let lexer = substraittypelexer::SubstraitTypeLexer::new(antlr_rust::InputStream::new($x));
        let token_source = antlr_rust::common_token_stream::CommonTokenStream::new(lexer);
        let mut parser = substraittypeparser::SubstraitTypeParser::new(token_source);
        let listener = ErrorListener::new();
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(listener.clone()));
        let result = parser.$start();
        listener.to_context($y);
        result.map_err(|e| cause!(TypeParseError, "{e}"))
    }};
}

/// Resolves an identifier path used in pattern scope.
fn resolve_pattern_identifier<S, I>(
    path: I,
    y: &mut context::Context,
) -> Result<context::IdentifiedObject>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    // Reconstruct the full identifier path for use in error messages.
    let path = path.collect::<Vec<_>>();
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
    // Parse the string with ANTLR.
    let _tree = antlr_parse!(x, y, startPattern)?;

    //
    Err(cause!(NotYetImplemented, "TODO"))
}

/// Parse a string as a meta-program.
pub fn parse_program(x: &str, y: &mut context::Context) -> Result<meta::Program> {
    // Parse the string with ANTLR.
    let _tree = antlr_parse!(x, y, startProgram)?;

    //
    Err(cause!(NotYetImplemented, "TODO"))
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

        parse_program(
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
        .ok();

        //result.program().unwrap().;
        //panic!("{node:?}");
        //panic!("{:#?}", result.to_string_tree(&*parser));
    }
}
