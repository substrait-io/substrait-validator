// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]
// Generated from SubstraitType.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::substraittypelistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const LineComment:isize=1; 
		pub const BlockComment:isize=2; 
		pub const Whitespace:isize=3; 
		pub const Newline:isize=4; 
		pub const EscNewline:isize=5; 
		pub const Assert:isize=6; 
		pub const Matches:isize=7; 
		pub const If:isize=8; 
		pub const Then:isize=9; 
		pub const Else:isize=10; 
		pub const Null:isize=11; 
		pub const True:isize=12; 
		pub const False:isize=13; 
		pub const Metabool:isize=14; 
		pub const Metaint:isize=15; 
		pub const Metaenum:isize=16; 
		pub const Metastr:isize=17; 
		pub const Typename:isize=18; 
		pub const Period:isize=19; 
		pub const Comma:isize=20; 
		pub const Colon:isize=21; 
		pub const Semicolon:isize=22; 
		pub const Question:isize=23; 
		pub const OpenParen:isize=24; 
		pub const CloseParen:isize=25; 
		pub const OpenCurly:isize=26; 
		pub const CloseCurly:isize=27; 
		pub const OpenSquare:isize=28; 
		pub const CloseSquare:isize=29; 
		pub const Assign:isize=30; 
		pub const BooleanOr:isize=31; 
		pub const BooleanAnd:isize=32; 
		pub const BooleanNot:isize=33; 
		pub const Equal:isize=34; 
		pub const NotEqual:isize=35; 
		pub const LessThan:isize=36; 
		pub const LessEqual:isize=37; 
		pub const GreaterThan:isize=38; 
		pub const GreaterEqual:isize=39; 
		pub const Plus:isize=40; 
		pub const Minus:isize=41; 
		pub const Multiply:isize=42; 
		pub const Divide:isize=43; 
		pub const Range:isize=44; 
		pub const Nonzero:isize=45; 
		pub const Zero:isize=46; 
		pub const String:isize=47; 
		pub const Identifier:isize=48;
	pub const RULE_startPattern:usize = 0; 
	pub const RULE_startProgram:usize = 1; 
	pub const RULE_program:usize = 2; 
	pub const RULE_statementSeparator:usize = 3; 
	pub const RULE_statement:usize = 4; 
	pub const RULE_pattern:usize = 5; 
	pub const RULE_patternOr:usize = 6; 
	pub const RULE_operatorOr:usize = 7; 
	pub const RULE_patternAnd:usize = 8; 
	pub const RULE_operatorAnd:usize = 9; 
	pub const RULE_patternEqNeq:usize = 10; 
	pub const RULE_operatorEqNeq:usize = 11; 
	pub const RULE_patternIneq:usize = 12; 
	pub const RULE_operatorIneq:usize = 13; 
	pub const RULE_patternAddSub:usize = 14; 
	pub const RULE_operatorAddSub:usize = 15; 
	pub const RULE_patternMulDiv:usize = 16; 
	pub const RULE_operatorMulDiv:usize = 17; 
	pub const RULE_patternMisc:usize = 18; 
	pub const RULE_nullability:usize = 19; 
	pub const RULE_variation:usize = 20; 
	pub const RULE_variationBody:usize = 21; 
	pub const RULE_parameters:usize = 22; 
	pub const RULE_parameter:usize = 23; 
	pub const RULE_parameterValue:usize = 24; 
	pub const RULE_integer:usize = 25; 
	pub const RULE_identifierPath:usize = 26; 
	pub const RULE_identifierOrString:usize = 27;
	pub const ruleNames: [&'static str; 28] =  [
		"startPattern", "startProgram", "program", "statementSeparator", "statement", 
		"pattern", "patternOr", "operatorOr", "patternAnd", "operatorAnd", "patternEqNeq", 
		"operatorEqNeq", "patternIneq", "operatorIneq", "patternAddSub", "operatorAddSub", 
		"patternMulDiv", "operatorMulDiv", "patternMisc", "nullability", "variation", 
		"variationBody", "parameters", "parameter", "parameterValue", "integer", 
		"identifierPath", "identifierOrString"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;47] = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, Some("'.'"), Some("','"), Some("':'"), 
		Some("';'"), Some("'?'"), Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), 
		Some("'['"), Some("']'"), Some("'='"), Some("'||'"), Some("'&&'"), Some("'!'"), 
		Some("'=='"), Some("'!='"), Some("'<'"), Some("'<='"), Some("'>'"), Some("'>='"), 
		Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), Some("'..'"), None, 
		Some("'0'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;49]  = [
		None, Some("LineComment"), Some("BlockComment"), Some("Whitespace"), Some("Newline"), 
		Some("EscNewline"), Some("Assert"), Some("Matches"), Some("If"), Some("Then"), 
		Some("Else"), Some("Null"), Some("True"), Some("False"), Some("Metabool"), 
		Some("Metaint"), Some("Metaenum"), Some("Metastr"), Some("Typename"), 
		Some("Period"), Some("Comma"), Some("Colon"), Some("Semicolon"), Some("Question"), 
		Some("OpenParen"), Some("CloseParen"), Some("OpenCurly"), Some("CloseCurly"), 
		Some("OpenSquare"), Some("CloseSquare"), Some("Assign"), Some("BooleanOr"), 
		Some("BooleanAnd"), Some("BooleanNot"), Some("Equal"), Some("NotEqual"), 
		Some("LessThan"), Some("LessEqual"), Some("GreaterThan"), Some("GreaterEqual"), 
		Some("Plus"), Some("Minus"), Some("Multiply"), Some("Divide"), Some("Range"), 
		Some("Nonzero"), Some("Zero"), Some("String"), Some("Identifier")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,SubstraitTypeParserExt<'input>, I, SubstraitTypeParserContextType , dyn SubstraitTypeListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SubstraitTypeTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, SubstraitTypeParserContextType , dyn SubstraitTypeListener<'input> + 'a>;

/// Parser for SubstraitType grammar
pub struct SubstraitTypeParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				SubstraitTypeParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> SubstraitTypeParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SubstraitTypeParser<'input, I, DefaultErrorStrategy<'input,SubstraitTypeParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SubstraitTypeParser
pub trait SubstraitTypeParserContext<'input>:
	for<'x> Listenable<dyn SubstraitTypeListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=SubstraitTypeParserContextType>
{}

antlr_rust::coerce_from!{ 'input : SubstraitTypeParserContext<'input> }

impl<'input> SubstraitTypeParserContext<'input> for TerminalNode<'input,SubstraitTypeParserContextType> {}
impl<'input> SubstraitTypeParserContext<'input> for ErrorNode<'input,SubstraitTypeParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SubstraitTypeParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SubstraitTypeListener<'input> + 'input }

pub struct SubstraitTypeParserContextType;
antlr_rust::tid!{SubstraitTypeParserContextType}

impl<'input> ParserNodeType<'input> for SubstraitTypeParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn SubstraitTypeParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SubstraitTypeParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> SubstraitTypeParserExt<'input>{
}
antlr_rust::tid! { SubstraitTypeParserExt<'a> }

impl<'input> TokenAware<'input> for SubstraitTypeParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for SubstraitTypeParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for SubstraitTypeParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "SubstraitType.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- startPattern ----------------
pub type StartPatternContextAll<'input> = StartPatternContext<'input>;


pub type StartPatternContext<'input> = BaseParserRuleContext<'input,StartPatternContextExt<'input>>;

#[derive(Clone)]
pub struct StartPatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for StartPatternContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StartPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_startPattern(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_startPattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StartPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_startPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_startPattern }
}
antlr_rust::tid!{StartPatternContextExt<'a>}

impl<'input> StartPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StartPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StartPatternContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StartPatternContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<StartPatternContextExt<'input>>{

fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token Whitespace in current rule
fn Whitespace_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Whitespace, starting from 0.
/// Returns `None` if number of children corresponding to token Whitespace is less or equal than `i`.
fn Whitespace(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Whitespace, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Newline in current rule
fn Newline_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Newline, starting from 0.
/// Returns `None` if number of children corresponding to token Newline is less or equal than `i`.
fn Newline(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Newline, i)
}

}

impl<'input> StartPatternContextAttrs<'input> for StartPatternContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn startPattern(&mut self,)
	-> Result<Rc<StartPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StartPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_startPattern);
        let mut _localctx: Rc<StartPatternContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(59);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Whitespace {
				{
				{
				recog.base.set_state(56);
				recog.base.match_token(Whitespace,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(61);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(65);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Newline {
				{
				{
				recog.base.set_state(62);
				recog.base.match_token(Newline,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(67);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule pattern*/
			recog.base.set_state(68);
			recog.pattern()?;

			recog.base.set_state(72);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Newline {
				{
				{
				recog.base.set_state(69);
				recog.base.match_token(Newline,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(74);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(75);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- startProgram ----------------
pub type StartProgramContextAll<'input> = StartProgramContext<'input>;


pub type StartProgramContext<'input> = BaseParserRuleContext<'input,StartProgramContextExt<'input>>;

#[derive(Clone)]
pub struct StartProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for StartProgramContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StartProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_startProgram(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_startProgram(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StartProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_startProgram }
	//fn type_rule_index() -> usize where Self: Sized { RULE_startProgram }
}
antlr_rust::tid!{StartProgramContextExt<'a>}

impl<'input> StartProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StartProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StartProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StartProgramContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<StartProgramContextExt<'input>>{

fn program(&self) -> Option<Rc<ProgramContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token Whitespace in current rule
fn Whitespace_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Whitespace, starting from 0.
/// Returns `None` if number of children corresponding to token Whitespace is less or equal than `i`.
fn Whitespace(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Whitespace, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Newline in current rule
fn Newline_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Newline, starting from 0.
/// Returns `None` if number of children corresponding to token Newline is less or equal than `i`.
fn Newline(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Newline, i)
}

}

impl<'input> StartProgramContextAttrs<'input> for StartProgramContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn startProgram(&mut self,)
	-> Result<Rc<StartProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StartProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_startProgram);
        let mut _localctx: Rc<StartProgramContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(80);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Whitespace {
				{
				{
				recog.base.set_state(77);
				recog.base.match_token(Whitespace,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(82);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(86);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Newline {
				{
				{
				recog.base.set_state(83);
				recog.base.match_token(Newline,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(88);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule program*/
			recog.base.set_state(89);
			recog.program()?;

			recog.base.set_state(93);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Newline {
				{
				{
				recog.base.set_state(90);
				recog.base.match_token(Newline,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(95);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(96);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn statementSeparator_all(&self) ->  Vec<Rc<StatementSeparatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statementSeparator(&self, i: usize) -> Option<Rc<StatementSeparatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(103);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule statement*/
					recog.base.set_state(98);
					recog.statement()?;

					/*InvokeRule statementSeparator*/
					recog.base.set_state(99);
					recog.statementSeparator()?;

					}
					} 
				}
				recog.base.set_state(105);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			}
			/*InvokeRule pattern*/
			recog.base.set_state(106);
			recog.pattern()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statementSeparator ----------------
pub type StatementSeparatorContextAll<'input> = StatementSeparatorContext<'input>;


pub type StatementSeparatorContext<'input> = BaseParserRuleContext<'input,StatementSeparatorContextExt<'input>>;

#[derive(Clone)]
pub struct StatementSeparatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for StatementSeparatorContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StatementSeparatorContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statementSeparator(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_statementSeparator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StatementSeparatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statementSeparator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statementSeparator }
}
antlr_rust::tid!{StatementSeparatorContextExt<'a>}

impl<'input> StatementSeparatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementSeparatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementSeparatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementSeparatorContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<StatementSeparatorContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Newline in current rule
fn Newline_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Newline, starting from 0.
/// Returns `None` if number of children corresponding to token Newline is less or equal than `i`.
fn Newline(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Newline, i)
}
/// Retrieves first TerminalNode corresponding to token Semicolon
/// Returns `None` if there is no child corresponding to token Semicolon
fn Semicolon(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Semicolon, 0)
}

}

impl<'input> StatementSeparatorContextAttrs<'input> for StatementSeparatorContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statementSeparator(&mut self,)
	-> Result<Rc<StatementSeparatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementSeparatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_statementSeparator);
        let mut _localctx: Rc<StatementSeparatorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(111);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(108);
					recog.base.match_token(Newline,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(113);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
			}
			recog.base.set_state(122);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Newline 
				=> {
					{
					recog.base.set_state(114);
					recog.base.match_token(Newline,&mut recog.err_handler)?;

					}
				}

			 Semicolon 
				=> {
					{
					recog.base.set_state(115);
					recog.base.match_token(Semicolon,&mut recog.err_handler)?;

					recog.base.set_state(119);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==Newline {
						{
						{
						recog.base.set_state(116);
						recog.base.match_token(Newline,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(121);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
#[derive(Debug)]
pub enum StatementContextAll<'input>{
	AssertContext(AssertContext<'input>),
	NormalContext(NormalContext<'input>),
	MatchContext(MatchContext<'input>),
Error(StatementContext<'input>)
}
antlr_rust::tid!{StatementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for StatementContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for StatementContextAll<'input>{}

impl<'input> Deref for StatementContextAll<'input>{
	type Target = dyn StatementContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use StatementContextAll::*;
		match self{
			AssertContext(inner) => inner,
			NormalContext(inner) => inner,
			MatchContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StatementContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StatementContext<'input>{
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
		StatementContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait StatementContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<StatementContextExt<'input>>{


}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

pub type AssertContext<'input> = BaseParserRuleContext<'input,AssertContextExt<'input>>;

pub trait AssertContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Assert
	/// Returns `None` if there is no child corresponding to token Assert
	fn Assert(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Assert, 0)
	}
	fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AssertContextAttrs<'input> for AssertContext<'input>{}

pub struct AssertContextExt<'input>{
	base:StatementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AssertContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AssertContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AssertContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Assert(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssertContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for AssertContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for AssertContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for AssertContext<'input> {}

impl<'input> AssertContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::AssertContext(
				BaseParserRuleContext::copy_from(ctx,AssertContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NormalContext<'input> = BaseParserRuleContext<'input,NormalContextExt<'input>>;

pub trait NormalContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn pattern_all(&self) ->  Vec<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn pattern(&self, i: usize) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Assign
	/// Returns `None` if there is no child corresponding to token Assign
	fn Assign(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Assign, 0)
	}
}

impl<'input> NormalContextAttrs<'input> for NormalContext<'input>{}

pub struct NormalContextExt<'input>{
	base:StatementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NormalContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NormalContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NormalContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Normal(self);
	}
}

impl<'input> CustomRuleContext<'input> for NormalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for NormalContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for NormalContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for NormalContext<'input> {}

impl<'input> NormalContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::NormalContext(
				BaseParserRuleContext::copy_from(ctx,NormalContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MatchContext<'input> = BaseParserRuleContext<'input,MatchContextExt<'input>>;

pub trait MatchContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Assert
	/// Returns `None` if there is no child corresponding to token Assert
	fn Assert(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Assert, 0)
	}
	fn pattern_all(&self) ->  Vec<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn pattern(&self, i: usize) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Matches
	/// Returns `None` if there is no child corresponding to token Matches
	fn Matches(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Matches, 0)
	}
}

impl<'input> MatchContextAttrs<'input> for MatchContext<'input>{}

pub struct MatchContextExt<'input>{
	base:StatementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MatchContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for MatchContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for MatchContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Match(self);
	}
}

impl<'input> CustomRuleContext<'input> for MatchContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}

impl<'input> Borrow<StatementContextExt<'input>> for MatchContext<'input>{
	fn borrow(&self) -> &StatementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StatementContextExt<'input>> for MatchContext<'input>{
	fn borrow_mut(&mut self) -> &mut StatementContextExt<'input> { &mut self.base }
}

impl<'input> StatementContextAttrs<'input> for MatchContext<'input> {}

impl<'input> MatchContextExt<'input>{
	fn new(ctx: &dyn StatementContextAttrs<'input>) -> Rc<StatementContextAll<'input>>  {
		Rc::new(
			StatementContextAll::MatchContext(
				BaseParserRuleContext::copy_from(ctx,MatchContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(135);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
				1 =>{
					let tmp = NormalContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule pattern*/
					recog.base.set_state(124);
					recog.pattern()?;

					recog.base.set_state(125);
					recog.base.match_token(Assign,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(126);
					recog.pattern()?;

					}
				}
			,
				2 =>{
					let tmp = MatchContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(128);
					recog.base.match_token(Assert,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(129);
					recog.pattern()?;

					recog.base.set_state(130);
					recog.base.match_token(Matches,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(131);
					recog.pattern()?;

					}
				}
			,
				3 =>{
					let tmp = AssertContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(133);
					recog.base.match_token(Assert,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(134);
					recog.pattern()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pattern ----------------
pub type PatternContextAll<'input> = PatternContext<'input>;


pub type PatternContext<'input> = BaseParserRuleContext<'input,PatternContextExt<'input>>;

#[derive(Clone)]
pub struct PatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for PatternContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pattern(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_pattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern }
}
antlr_rust::tid!{PatternContextExt<'a>}

impl<'input> PatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternContextExt<'input>>{

fn patternOr(&self) -> Option<Rc<PatternOrContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PatternContextAttrs<'input> for PatternContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pattern(&mut self,)
	-> Result<Rc<PatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_pattern);
        let mut _localctx: Rc<PatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule patternOr*/
			recog.base.set_state(137);
			recog.patternOr()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patternOr ----------------
pub type PatternOrContextAll<'input> = PatternOrContext<'input>;


pub type PatternOrContext<'input> = BaseParserRuleContext<'input,PatternOrContextExt<'input>>;

#[derive(Clone)]
pub struct PatternOrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for PatternOrContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternOrContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternOr(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_patternOr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternOrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternOr }
}
antlr_rust::tid!{PatternOrContextExt<'a>}

impl<'input> PatternOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternOrContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternOrContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternOrContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternOrContextExt<'input>>{

fn patternAnd_all(&self) ->  Vec<Rc<PatternAndContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn patternAnd(&self, i: usize) -> Option<Rc<PatternAndContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn operatorOr_all(&self) ->  Vec<Rc<OperatorOrContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn operatorOr(&self, i: usize) -> Option<Rc<OperatorOrContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PatternOrContextAttrs<'input> for PatternOrContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternOr(&mut self,)
	-> Result<Rc<PatternOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_patternOr);
        let mut _localctx: Rc<PatternOrContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule patternAnd*/
			recog.base.set_state(139);
			recog.patternAnd()?;

			recog.base.set_state(145);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorOr*/
					recog.base.set_state(140);
					recog.operatorOr()?;

					/*InvokeRule patternAnd*/
					recog.base.set_state(141);
					recog.patternAnd()?;

					}
					} 
				}
				recog.base.set_state(147);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- operatorOr ----------------
#[derive(Debug)]
pub enum OperatorOrContextAll<'input>{
	OrContext(OrContext<'input>),
Error(OperatorOrContext<'input>)
}
antlr_rust::tid!{OperatorOrContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for OperatorOrContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for OperatorOrContextAll<'input>{}

impl<'input> Deref for OperatorOrContextAll<'input>{
	type Target = dyn OperatorOrContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use OperatorOrContextAll::*;
		match self{
			OrContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorOrContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type OperatorOrContext<'input> = BaseParserRuleContext<'input,OperatorOrContextExt<'input>>;

#[derive(Clone)]
pub struct OperatorOrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for OperatorOrContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorOrContext<'input>{
}

impl<'input> CustomRuleContext<'input> for OperatorOrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorOr }
}
antlr_rust::tid!{OperatorOrContextExt<'a>}

impl<'input> OperatorOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OperatorOrContextAll<'input>> {
		Rc::new(
		OperatorOrContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OperatorOrContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait OperatorOrContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<OperatorOrContextExt<'input>>{


}

impl<'input> OperatorOrContextAttrs<'input> for OperatorOrContext<'input>{}

pub type OrContext<'input> = BaseParserRuleContext<'input,OrContextExt<'input>>;

pub trait OrContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token BooleanOr
	/// Returns `None` if there is no child corresponding to token BooleanOr
	fn BooleanOr(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(BooleanOr, 0)
	}
}

impl<'input> OrContextAttrs<'input> for OrContext<'input>{}

pub struct OrContextExt<'input>{
	base:OperatorOrContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{OrContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for OrContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OrContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Or(self);
	}
}

impl<'input> CustomRuleContext<'input> for OrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorOr }
}

impl<'input> Borrow<OperatorOrContextExt<'input>> for OrContext<'input>{
	fn borrow(&self) -> &OperatorOrContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorOrContextExt<'input>> for OrContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorOrContextExt<'input> { &mut self.base }
}

impl<'input> OperatorOrContextAttrs<'input> for OrContext<'input> {}

impl<'input> OrContextExt<'input>{
	fn new(ctx: &dyn OperatorOrContextAttrs<'input>) -> Rc<OperatorOrContextAll<'input>>  {
		Rc::new(
			OperatorOrContextAll::OrContext(
				BaseParserRuleContext::copy_from(ctx,OrContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn operatorOr(&mut self,)
	-> Result<Rc<OperatorOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_operatorOr);
        let mut _localctx: Rc<OperatorOrContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = OrContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			recog.base.set_state(148);
			recog.base.match_token(BooleanOr,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patternAnd ----------------
pub type PatternAndContextAll<'input> = PatternAndContext<'input>;


pub type PatternAndContext<'input> = BaseParserRuleContext<'input,PatternAndContextExt<'input>>;

#[derive(Clone)]
pub struct PatternAndContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for PatternAndContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternAndContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternAnd(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_patternAnd(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternAndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternAnd }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternAnd }
}
antlr_rust::tid!{PatternAndContextExt<'a>}

impl<'input> PatternAndContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternAndContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternAndContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternAndContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternAndContextExt<'input>>{

fn patternEqNeq_all(&self) ->  Vec<Rc<PatternEqNeqContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn patternEqNeq(&self, i: usize) -> Option<Rc<PatternEqNeqContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn operatorAnd_all(&self) ->  Vec<Rc<OperatorAndContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn operatorAnd(&self, i: usize) -> Option<Rc<OperatorAndContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PatternAndContextAttrs<'input> for PatternAndContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternAnd(&mut self,)
	-> Result<Rc<PatternAndContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternAndContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_patternAnd);
        let mut _localctx: Rc<PatternAndContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule patternEqNeq*/
			recog.base.set_state(150);
			recog.patternEqNeq()?;

			recog.base.set_state(156);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorAnd*/
					recog.base.set_state(151);
					recog.operatorAnd()?;

					/*InvokeRule patternEqNeq*/
					recog.base.set_state(152);
					recog.patternEqNeq()?;

					}
					} 
				}
				recog.base.set_state(158);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- operatorAnd ----------------
#[derive(Debug)]
pub enum OperatorAndContextAll<'input>{
	AndContext(AndContext<'input>),
Error(OperatorAndContext<'input>)
}
antlr_rust::tid!{OperatorAndContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for OperatorAndContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for OperatorAndContextAll<'input>{}

impl<'input> Deref for OperatorAndContextAll<'input>{
	type Target = dyn OperatorAndContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use OperatorAndContextAll::*;
		match self{
			AndContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorAndContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type OperatorAndContext<'input> = BaseParserRuleContext<'input,OperatorAndContextExt<'input>>;

#[derive(Clone)]
pub struct OperatorAndContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for OperatorAndContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorAndContext<'input>{
}

impl<'input> CustomRuleContext<'input> for OperatorAndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorAnd }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorAnd }
}
antlr_rust::tid!{OperatorAndContextExt<'a>}

impl<'input> OperatorAndContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OperatorAndContextAll<'input>> {
		Rc::new(
		OperatorAndContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OperatorAndContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait OperatorAndContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<OperatorAndContextExt<'input>>{


}

impl<'input> OperatorAndContextAttrs<'input> for OperatorAndContext<'input>{}

pub type AndContext<'input> = BaseParserRuleContext<'input,AndContextExt<'input>>;

pub trait AndContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token BooleanAnd
	/// Returns `None` if there is no child corresponding to token BooleanAnd
	fn BooleanAnd(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(BooleanAnd, 0)
	}
}

impl<'input> AndContextAttrs<'input> for AndContext<'input>{}

pub struct AndContextExt<'input>{
	base:OperatorAndContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AndContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AndContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AndContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_And(self);
	}
}

impl<'input> CustomRuleContext<'input> for AndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorAnd }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorAnd }
}

impl<'input> Borrow<OperatorAndContextExt<'input>> for AndContext<'input>{
	fn borrow(&self) -> &OperatorAndContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorAndContextExt<'input>> for AndContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorAndContextExt<'input> { &mut self.base }
}

impl<'input> OperatorAndContextAttrs<'input> for AndContext<'input> {}

impl<'input> AndContextExt<'input>{
	fn new(ctx: &dyn OperatorAndContextAttrs<'input>) -> Rc<OperatorAndContextAll<'input>>  {
		Rc::new(
			OperatorAndContextAll::AndContext(
				BaseParserRuleContext::copy_from(ctx,AndContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn operatorAnd(&mut self,)
	-> Result<Rc<OperatorAndContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorAndContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_operatorAnd);
        let mut _localctx: Rc<OperatorAndContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = AndContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			recog.base.set_state(159);
			recog.base.match_token(BooleanAnd,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patternEqNeq ----------------
pub type PatternEqNeqContextAll<'input> = PatternEqNeqContext<'input>;


pub type PatternEqNeqContext<'input> = BaseParserRuleContext<'input,PatternEqNeqContextExt<'input>>;

#[derive(Clone)]
pub struct PatternEqNeqContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for PatternEqNeqContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternEqNeqContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternEqNeq(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_patternEqNeq(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternEqNeqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternEqNeq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternEqNeq }
}
antlr_rust::tid!{PatternEqNeqContextExt<'a>}

impl<'input> PatternEqNeqContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternEqNeqContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternEqNeqContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternEqNeqContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternEqNeqContextExt<'input>>{

fn patternIneq_all(&self) ->  Vec<Rc<PatternIneqContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn patternIneq(&self, i: usize) -> Option<Rc<PatternIneqContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn operatorEqNeq_all(&self) ->  Vec<Rc<OperatorEqNeqContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn operatorEqNeq(&self, i: usize) -> Option<Rc<OperatorEqNeqContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PatternEqNeqContextAttrs<'input> for PatternEqNeqContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternEqNeq(&mut self,)
	-> Result<Rc<PatternEqNeqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternEqNeqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_patternEqNeq);
        let mut _localctx: Rc<PatternEqNeqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule patternIneq*/
			recog.base.set_state(161);
			recog.patternIneq()?;

			recog.base.set_state(167);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorEqNeq*/
					recog.base.set_state(162);
					recog.operatorEqNeq()?;

					/*InvokeRule patternIneq*/
					recog.base.set_state(163);
					recog.patternIneq()?;

					}
					} 
				}
				recog.base.set_state(169);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- operatorEqNeq ----------------
#[derive(Debug)]
pub enum OperatorEqNeqContextAll<'input>{
	NeqContext(NeqContext<'input>),
	EqContext(EqContext<'input>),
Error(OperatorEqNeqContext<'input>)
}
antlr_rust::tid!{OperatorEqNeqContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for OperatorEqNeqContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for OperatorEqNeqContextAll<'input>{}

impl<'input> Deref for OperatorEqNeqContextAll<'input>{
	type Target = dyn OperatorEqNeqContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use OperatorEqNeqContextAll::*;
		match self{
			NeqContext(inner) => inner,
			EqContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorEqNeqContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type OperatorEqNeqContext<'input> = BaseParserRuleContext<'input,OperatorEqNeqContextExt<'input>>;

#[derive(Clone)]
pub struct OperatorEqNeqContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for OperatorEqNeqContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorEqNeqContext<'input>{
}

impl<'input> CustomRuleContext<'input> for OperatorEqNeqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorEqNeq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorEqNeq }
}
antlr_rust::tid!{OperatorEqNeqContextExt<'a>}

impl<'input> OperatorEqNeqContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OperatorEqNeqContextAll<'input>> {
		Rc::new(
		OperatorEqNeqContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OperatorEqNeqContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait OperatorEqNeqContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<OperatorEqNeqContextExt<'input>>{


}

impl<'input> OperatorEqNeqContextAttrs<'input> for OperatorEqNeqContext<'input>{}

pub type NeqContext<'input> = BaseParserRuleContext<'input,NeqContextExt<'input>>;

pub trait NeqContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NotEqual
	/// Returns `None` if there is no child corresponding to token NotEqual
	fn NotEqual(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(NotEqual, 0)
	}
}

impl<'input> NeqContextAttrs<'input> for NeqContext<'input>{}

pub struct NeqContextExt<'input>{
	base:OperatorEqNeqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NeqContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NeqContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NeqContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Neq(self);
	}
}

impl<'input> CustomRuleContext<'input> for NeqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorEqNeq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorEqNeq }
}

impl<'input> Borrow<OperatorEqNeqContextExt<'input>> for NeqContext<'input>{
	fn borrow(&self) -> &OperatorEqNeqContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorEqNeqContextExt<'input>> for NeqContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorEqNeqContextExt<'input> { &mut self.base }
}

impl<'input> OperatorEqNeqContextAttrs<'input> for NeqContext<'input> {}

impl<'input> NeqContextExt<'input>{
	fn new(ctx: &dyn OperatorEqNeqContextAttrs<'input>) -> Rc<OperatorEqNeqContextAll<'input>>  {
		Rc::new(
			OperatorEqNeqContextAll::NeqContext(
				BaseParserRuleContext::copy_from(ctx,NeqContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EqContext<'input> = BaseParserRuleContext<'input,EqContextExt<'input>>;

pub trait EqContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Equal
	/// Returns `None` if there is no child corresponding to token Equal
	fn Equal(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Equal, 0)
	}
}

impl<'input> EqContextAttrs<'input> for EqContext<'input>{}

pub struct EqContextExt<'input>{
	base:OperatorEqNeqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EqContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for EqContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for EqContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Eq(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorEqNeq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorEqNeq }
}

impl<'input> Borrow<OperatorEqNeqContextExt<'input>> for EqContext<'input>{
	fn borrow(&self) -> &OperatorEqNeqContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorEqNeqContextExt<'input>> for EqContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorEqNeqContextExt<'input> { &mut self.base }
}

impl<'input> OperatorEqNeqContextAttrs<'input> for EqContext<'input> {}

impl<'input> EqContextExt<'input>{
	fn new(ctx: &dyn OperatorEqNeqContextAttrs<'input>) -> Rc<OperatorEqNeqContextAll<'input>>  {
		Rc::new(
			OperatorEqNeqContextAll::EqContext(
				BaseParserRuleContext::copy_from(ctx,EqContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn operatorEqNeq(&mut self,)
	-> Result<Rc<OperatorEqNeqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorEqNeqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_operatorEqNeq);
        let mut _localctx: Rc<OperatorEqNeqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(172);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Equal 
				=> {
					let tmp = EqContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(170);
					recog.base.match_token(Equal,&mut recog.err_handler)?;

					}
				}

			 NotEqual 
				=> {
					let tmp = NeqContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(171);
					recog.base.match_token(NotEqual,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patternIneq ----------------
pub type PatternIneqContextAll<'input> = PatternIneqContext<'input>;


pub type PatternIneqContext<'input> = BaseParserRuleContext<'input,PatternIneqContextExt<'input>>;

#[derive(Clone)]
pub struct PatternIneqContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for PatternIneqContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternIneqContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternIneq(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_patternIneq(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternIneqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternIneq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternIneq }
}
antlr_rust::tid!{PatternIneqContextExt<'a>}

impl<'input> PatternIneqContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternIneqContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternIneqContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternIneqContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternIneqContextExt<'input>>{

fn patternAddSub_all(&self) ->  Vec<Rc<PatternAddSubContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn patternAddSub(&self, i: usize) -> Option<Rc<PatternAddSubContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn operatorIneq_all(&self) ->  Vec<Rc<OperatorIneqContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn operatorIneq(&self, i: usize) -> Option<Rc<OperatorIneqContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PatternIneqContextAttrs<'input> for PatternIneqContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternIneq(&mut self,)
	-> Result<Rc<PatternIneqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternIneqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_patternIneq);
        let mut _localctx: Rc<PatternIneqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule patternAddSub*/
			recog.base.set_state(174);
			recog.patternAddSub()?;

			recog.base.set_state(180);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorIneq*/
					recog.base.set_state(175);
					recog.operatorIneq()?;

					/*InvokeRule patternAddSub*/
					recog.base.set_state(176);
					recog.patternAddSub()?;

					}
					} 
				}
				recog.base.set_state(182);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- operatorIneq ----------------
#[derive(Debug)]
pub enum OperatorIneqContextAll<'input>{
	LtContext(LtContext<'input>),
	LeContext(LeContext<'input>),
	GtContext(GtContext<'input>),
	GeContext(GeContext<'input>),
Error(OperatorIneqContext<'input>)
}
antlr_rust::tid!{OperatorIneqContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for OperatorIneqContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for OperatorIneqContextAll<'input>{}

impl<'input> Deref for OperatorIneqContextAll<'input>{
	type Target = dyn OperatorIneqContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use OperatorIneqContextAll::*;
		match self{
			LtContext(inner) => inner,
			LeContext(inner) => inner,
			GtContext(inner) => inner,
			GeContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorIneqContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type OperatorIneqContext<'input> = BaseParserRuleContext<'input,OperatorIneqContextExt<'input>>;

#[derive(Clone)]
pub struct OperatorIneqContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for OperatorIneqContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorIneqContext<'input>{
}

impl<'input> CustomRuleContext<'input> for OperatorIneqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorIneq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorIneq }
}
antlr_rust::tid!{OperatorIneqContextExt<'a>}

impl<'input> OperatorIneqContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OperatorIneqContextAll<'input>> {
		Rc::new(
		OperatorIneqContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OperatorIneqContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait OperatorIneqContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<OperatorIneqContextExt<'input>>{


}

impl<'input> OperatorIneqContextAttrs<'input> for OperatorIneqContext<'input>{}

pub type LtContext<'input> = BaseParserRuleContext<'input,LtContextExt<'input>>;

pub trait LtContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LessThan
	/// Returns `None` if there is no child corresponding to token LessThan
	fn LessThan(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(LessThan, 0)
	}
}

impl<'input> LtContextAttrs<'input> for LtContext<'input>{}

pub struct LtContextExt<'input>{
	base:OperatorIneqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LtContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for LtContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for LtContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Lt(self);
	}
}

impl<'input> CustomRuleContext<'input> for LtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorIneq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorIneq }
}

impl<'input> Borrow<OperatorIneqContextExt<'input>> for LtContext<'input>{
	fn borrow(&self) -> &OperatorIneqContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorIneqContextExt<'input>> for LtContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorIneqContextExt<'input> { &mut self.base }
}

impl<'input> OperatorIneqContextAttrs<'input> for LtContext<'input> {}

impl<'input> LtContextExt<'input>{
	fn new(ctx: &dyn OperatorIneqContextAttrs<'input>) -> Rc<OperatorIneqContextAll<'input>>  {
		Rc::new(
			OperatorIneqContextAll::LtContext(
				BaseParserRuleContext::copy_from(ctx,LtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LeContext<'input> = BaseParserRuleContext<'input,LeContextExt<'input>>;

pub trait LeContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LessEqual
	/// Returns `None` if there is no child corresponding to token LessEqual
	fn LessEqual(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(LessEqual, 0)
	}
}

impl<'input> LeContextAttrs<'input> for LeContext<'input>{}

pub struct LeContextExt<'input>{
	base:OperatorIneqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LeContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for LeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for LeContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Le(self);
	}
}

impl<'input> CustomRuleContext<'input> for LeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorIneq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorIneq }
}

impl<'input> Borrow<OperatorIneqContextExt<'input>> for LeContext<'input>{
	fn borrow(&self) -> &OperatorIneqContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorIneqContextExt<'input>> for LeContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorIneqContextExt<'input> { &mut self.base }
}

impl<'input> OperatorIneqContextAttrs<'input> for LeContext<'input> {}

impl<'input> LeContextExt<'input>{
	fn new(ctx: &dyn OperatorIneqContextAttrs<'input>) -> Rc<OperatorIneqContextAll<'input>>  {
		Rc::new(
			OperatorIneqContextAll::LeContext(
				BaseParserRuleContext::copy_from(ctx,LeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type GtContext<'input> = BaseParserRuleContext<'input,GtContextExt<'input>>;

pub trait GtContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token GreaterThan
	/// Returns `None` if there is no child corresponding to token GreaterThan
	fn GreaterThan(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(GreaterThan, 0)
	}
}

impl<'input> GtContextAttrs<'input> for GtContext<'input>{}

pub struct GtContextExt<'input>{
	base:OperatorIneqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{GtContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for GtContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for GtContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Gt(self);
	}
}

impl<'input> CustomRuleContext<'input> for GtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorIneq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorIneq }
}

impl<'input> Borrow<OperatorIneqContextExt<'input>> for GtContext<'input>{
	fn borrow(&self) -> &OperatorIneqContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorIneqContextExt<'input>> for GtContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorIneqContextExt<'input> { &mut self.base }
}

impl<'input> OperatorIneqContextAttrs<'input> for GtContext<'input> {}

impl<'input> GtContextExt<'input>{
	fn new(ctx: &dyn OperatorIneqContextAttrs<'input>) -> Rc<OperatorIneqContextAll<'input>>  {
		Rc::new(
			OperatorIneqContextAll::GtContext(
				BaseParserRuleContext::copy_from(ctx,GtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type GeContext<'input> = BaseParserRuleContext<'input,GeContextExt<'input>>;

pub trait GeContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token GreaterEqual
	/// Returns `None` if there is no child corresponding to token GreaterEqual
	fn GreaterEqual(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(GreaterEqual, 0)
	}
}

impl<'input> GeContextAttrs<'input> for GeContext<'input>{}

pub struct GeContextExt<'input>{
	base:OperatorIneqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{GeContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for GeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for GeContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Ge(self);
	}
}

impl<'input> CustomRuleContext<'input> for GeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorIneq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorIneq }
}

impl<'input> Borrow<OperatorIneqContextExt<'input>> for GeContext<'input>{
	fn borrow(&self) -> &OperatorIneqContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorIneqContextExt<'input>> for GeContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorIneqContextExt<'input> { &mut self.base }
}

impl<'input> OperatorIneqContextAttrs<'input> for GeContext<'input> {}

impl<'input> GeContextExt<'input>{
	fn new(ctx: &dyn OperatorIneqContextAttrs<'input>) -> Rc<OperatorIneqContextAll<'input>>  {
		Rc::new(
			OperatorIneqContextAll::GeContext(
				BaseParserRuleContext::copy_from(ctx,GeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn operatorIneq(&mut self,)
	-> Result<Rc<OperatorIneqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorIneqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_operatorIneq);
        let mut _localctx: Rc<OperatorIneqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(187);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LessThan 
				=> {
					let tmp = LtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(183);
					recog.base.match_token(LessThan,&mut recog.err_handler)?;

					}
				}

			 LessEqual 
				=> {
					let tmp = LeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(184);
					recog.base.match_token(LessEqual,&mut recog.err_handler)?;

					}
				}

			 GreaterThan 
				=> {
					let tmp = GtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(185);
					recog.base.match_token(GreaterThan,&mut recog.err_handler)?;

					}
				}

			 GreaterEqual 
				=> {
					let tmp = GeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(186);
					recog.base.match_token(GreaterEqual,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patternAddSub ----------------
pub type PatternAddSubContextAll<'input> = PatternAddSubContext<'input>;


pub type PatternAddSubContext<'input> = BaseParserRuleContext<'input,PatternAddSubContextExt<'input>>;

#[derive(Clone)]
pub struct PatternAddSubContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for PatternAddSubContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternAddSubContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternAddSub(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_patternAddSub(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternAddSubContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternAddSub }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternAddSub }
}
antlr_rust::tid!{PatternAddSubContextExt<'a>}

impl<'input> PatternAddSubContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternAddSubContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternAddSubContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternAddSubContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternAddSubContextExt<'input>>{

fn patternMulDiv_all(&self) ->  Vec<Rc<PatternMulDivContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn patternMulDiv(&self, i: usize) -> Option<Rc<PatternMulDivContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn operatorAddSub_all(&self) ->  Vec<Rc<OperatorAddSubContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn operatorAddSub(&self, i: usize) -> Option<Rc<OperatorAddSubContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PatternAddSubContextAttrs<'input> for PatternAddSubContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternAddSub(&mut self,)
	-> Result<Rc<PatternAddSubContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternAddSubContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_patternAddSub);
        let mut _localctx: Rc<PatternAddSubContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule patternMulDiv*/
			recog.base.set_state(189);
			recog.patternMulDiv()?;

			recog.base.set_state(195);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorAddSub*/
					recog.base.set_state(190);
					recog.operatorAddSub()?;

					/*InvokeRule patternMulDiv*/
					recog.base.set_state(191);
					recog.patternMulDiv()?;

					}
					} 
				}
				recog.base.set_state(197);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- operatorAddSub ----------------
#[derive(Debug)]
pub enum OperatorAddSubContextAll<'input>{
	AddContext(AddContext<'input>),
	SubContext(SubContext<'input>),
Error(OperatorAddSubContext<'input>)
}
antlr_rust::tid!{OperatorAddSubContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for OperatorAddSubContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for OperatorAddSubContextAll<'input>{}

impl<'input> Deref for OperatorAddSubContextAll<'input>{
	type Target = dyn OperatorAddSubContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use OperatorAddSubContextAll::*;
		match self{
			AddContext(inner) => inner,
			SubContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorAddSubContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type OperatorAddSubContext<'input> = BaseParserRuleContext<'input,OperatorAddSubContextExt<'input>>;

#[derive(Clone)]
pub struct OperatorAddSubContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for OperatorAddSubContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorAddSubContext<'input>{
}

impl<'input> CustomRuleContext<'input> for OperatorAddSubContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorAddSub }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorAddSub }
}
antlr_rust::tid!{OperatorAddSubContextExt<'a>}

impl<'input> OperatorAddSubContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OperatorAddSubContextAll<'input>> {
		Rc::new(
		OperatorAddSubContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OperatorAddSubContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait OperatorAddSubContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<OperatorAddSubContextExt<'input>>{


}

impl<'input> OperatorAddSubContextAttrs<'input> for OperatorAddSubContext<'input>{}

pub type AddContext<'input> = BaseParserRuleContext<'input,AddContextExt<'input>>;

pub trait AddContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Plus
	/// Returns `None` if there is no child corresponding to token Plus
	fn Plus(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Plus, 0)
	}
}

impl<'input> AddContextAttrs<'input> for AddContext<'input>{}

pub struct AddContextExt<'input>{
	base:OperatorAddSubContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AddContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AddContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AddContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Add(self);
	}
}

impl<'input> CustomRuleContext<'input> for AddContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorAddSub }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorAddSub }
}

impl<'input> Borrow<OperatorAddSubContextExt<'input>> for AddContext<'input>{
	fn borrow(&self) -> &OperatorAddSubContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorAddSubContextExt<'input>> for AddContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorAddSubContextExt<'input> { &mut self.base }
}

impl<'input> OperatorAddSubContextAttrs<'input> for AddContext<'input> {}

impl<'input> AddContextExt<'input>{
	fn new(ctx: &dyn OperatorAddSubContextAttrs<'input>) -> Rc<OperatorAddSubContextAll<'input>>  {
		Rc::new(
			OperatorAddSubContextAll::AddContext(
				BaseParserRuleContext::copy_from(ctx,AddContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SubContext<'input> = BaseParserRuleContext<'input,SubContextExt<'input>>;

pub trait SubContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Minus
	/// Returns `None` if there is no child corresponding to token Minus
	fn Minus(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Minus, 0)
	}
}

impl<'input> SubContextAttrs<'input> for SubContext<'input>{}

pub struct SubContextExt<'input>{
	base:OperatorAddSubContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SubContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for SubContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for SubContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Sub(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorAddSub }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorAddSub }
}

impl<'input> Borrow<OperatorAddSubContextExt<'input>> for SubContext<'input>{
	fn borrow(&self) -> &OperatorAddSubContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorAddSubContextExt<'input>> for SubContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorAddSubContextExt<'input> { &mut self.base }
}

impl<'input> OperatorAddSubContextAttrs<'input> for SubContext<'input> {}

impl<'input> SubContextExt<'input>{
	fn new(ctx: &dyn OperatorAddSubContextAttrs<'input>) -> Rc<OperatorAddSubContextAll<'input>>  {
		Rc::new(
			OperatorAddSubContextAll::SubContext(
				BaseParserRuleContext::copy_from(ctx,SubContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn operatorAddSub(&mut self,)
	-> Result<Rc<OperatorAddSubContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorAddSubContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_operatorAddSub);
        let mut _localctx: Rc<OperatorAddSubContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(200);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Plus 
				=> {
					let tmp = AddContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(198);
					recog.base.match_token(Plus,&mut recog.err_handler)?;

					}
				}

			 Minus 
				=> {
					let tmp = SubContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(199);
					recog.base.match_token(Minus,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patternMulDiv ----------------
pub type PatternMulDivContextAll<'input> = PatternMulDivContext<'input>;


pub type PatternMulDivContext<'input> = BaseParserRuleContext<'input,PatternMulDivContextExt<'input>>;

#[derive(Clone)]
pub struct PatternMulDivContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for PatternMulDivContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternMulDivContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternMulDiv(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_patternMulDiv(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternMulDivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMulDiv }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMulDiv }
}
antlr_rust::tid!{PatternMulDivContextExt<'a>}

impl<'input> PatternMulDivContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternMulDivContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternMulDivContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternMulDivContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternMulDivContextExt<'input>>{

fn patternMisc_all(&self) ->  Vec<Rc<PatternMiscContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn patternMisc(&self, i: usize) -> Option<Rc<PatternMiscContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn operatorMulDiv_all(&self) ->  Vec<Rc<OperatorMulDivContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn operatorMulDiv(&self, i: usize) -> Option<Rc<OperatorMulDivContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PatternMulDivContextAttrs<'input> for PatternMulDivContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternMulDiv(&mut self,)
	-> Result<Rc<PatternMulDivContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternMulDivContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_patternMulDiv);
        let mut _localctx: Rc<PatternMulDivContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule patternMisc*/
			recog.base.set_state(202);
			recog.patternMisc()?;

			recog.base.set_state(208);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorMulDiv*/
					recog.base.set_state(203);
					recog.operatorMulDiv()?;

					/*InvokeRule patternMisc*/
					recog.base.set_state(204);
					recog.patternMisc()?;

					}
					} 
				}
				recog.base.set_state(210);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- operatorMulDiv ----------------
#[derive(Debug)]
pub enum OperatorMulDivContextAll<'input>{
	DivContext(DivContext<'input>),
	MulContext(MulContext<'input>),
Error(OperatorMulDivContext<'input>)
}
antlr_rust::tid!{OperatorMulDivContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for OperatorMulDivContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for OperatorMulDivContextAll<'input>{}

impl<'input> Deref for OperatorMulDivContextAll<'input>{
	type Target = dyn OperatorMulDivContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use OperatorMulDivContextAll::*;
		match self{
			DivContext(inner) => inner,
			MulContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorMulDivContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type OperatorMulDivContext<'input> = BaseParserRuleContext<'input,OperatorMulDivContextExt<'input>>;

#[derive(Clone)]
pub struct OperatorMulDivContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for OperatorMulDivContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OperatorMulDivContext<'input>{
}

impl<'input> CustomRuleContext<'input> for OperatorMulDivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorMulDiv }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorMulDiv }
}
antlr_rust::tid!{OperatorMulDivContextExt<'a>}

impl<'input> OperatorMulDivContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OperatorMulDivContextAll<'input>> {
		Rc::new(
		OperatorMulDivContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OperatorMulDivContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait OperatorMulDivContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<OperatorMulDivContextExt<'input>>{


}

impl<'input> OperatorMulDivContextAttrs<'input> for OperatorMulDivContext<'input>{}

pub type DivContext<'input> = BaseParserRuleContext<'input,DivContextExt<'input>>;

pub trait DivContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Divide
	/// Returns `None` if there is no child corresponding to token Divide
	fn Divide(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Divide, 0)
	}
}

impl<'input> DivContextAttrs<'input> for DivContext<'input>{}

pub struct DivContextExt<'input>{
	base:OperatorMulDivContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DivContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for DivContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for DivContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Div(self);
	}
}

impl<'input> CustomRuleContext<'input> for DivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorMulDiv }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorMulDiv }
}

impl<'input> Borrow<OperatorMulDivContextExt<'input>> for DivContext<'input>{
	fn borrow(&self) -> &OperatorMulDivContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorMulDivContextExt<'input>> for DivContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorMulDivContextExt<'input> { &mut self.base }
}

impl<'input> OperatorMulDivContextAttrs<'input> for DivContext<'input> {}

impl<'input> DivContextExt<'input>{
	fn new(ctx: &dyn OperatorMulDivContextAttrs<'input>) -> Rc<OperatorMulDivContextAll<'input>>  {
		Rc::new(
			OperatorMulDivContextAll::DivContext(
				BaseParserRuleContext::copy_from(ctx,DivContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MulContext<'input> = BaseParserRuleContext<'input,MulContextExt<'input>>;

pub trait MulContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Multiply
	/// Returns `None` if there is no child corresponding to token Multiply
	fn Multiply(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Multiply, 0)
	}
}

impl<'input> MulContextAttrs<'input> for MulContext<'input>{}

pub struct MulContextExt<'input>{
	base:OperatorMulDivContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MulContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for MulContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for MulContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Mul(self);
	}
}

impl<'input> CustomRuleContext<'input> for MulContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_operatorMulDiv }
	//fn type_rule_index() -> usize where Self: Sized { RULE_operatorMulDiv }
}

impl<'input> Borrow<OperatorMulDivContextExt<'input>> for MulContext<'input>{
	fn borrow(&self) -> &OperatorMulDivContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OperatorMulDivContextExt<'input>> for MulContext<'input>{
	fn borrow_mut(&mut self) -> &mut OperatorMulDivContextExt<'input> { &mut self.base }
}

impl<'input> OperatorMulDivContextAttrs<'input> for MulContext<'input> {}

impl<'input> MulContextExt<'input>{
	fn new(ctx: &dyn OperatorMulDivContextAttrs<'input>) -> Rc<OperatorMulDivContextAll<'input>>  {
		Rc::new(
			OperatorMulDivContextAll::MulContext(
				BaseParserRuleContext::copy_from(ctx,MulContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn operatorMulDiv(&mut self,)
	-> Result<Rc<OperatorMulDivContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorMulDivContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_operatorMulDiv);
        let mut _localctx: Rc<OperatorMulDivContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(213);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Multiply 
				=> {
					let tmp = MulContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(211);
					recog.base.match_token(Multiply,&mut recog.err_handler)?;

					}
				}

			 Divide 
				=> {
					let tmp = DivContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(212);
					recog.base.match_token(Divide,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patternMisc ----------------
#[derive(Debug)]
pub enum PatternMiscContextAll<'input>{
	ParenthesesContext(ParenthesesContext<'input>),
	IntRangeContext(IntRangeContext<'input>),
	UnaryNegateContext(UnaryNegateContext<'input>),
	StrExactlyContext(StrExactlyContext<'input>),
	IfThenElseContext(IfThenElseContext<'input>),
	BoolFalseContext(BoolFalseContext<'input>),
	EnumAnyContext(EnumAnyContext<'input>),
	DtAnyContext(DtAnyContext<'input>),
	AnyContext(AnyContext<'input>),
	IntAnyContext(IntAnyContext<'input>),
	DatatypeBindingOrConstantContext(DatatypeBindingOrConstantContext<'input>),
	EnumSetContext(EnumSetContext<'input>),
	StrAnyContext(StrAnyContext<'input>),
	BoolTrueContext(BoolTrueContext<'input>),
	IntAtMostContext(IntAtMostContext<'input>),
	IntAtLeastContext(IntAtLeastContext<'input>),
	IntExactlyContext(IntExactlyContext<'input>),
	FunctionContext(FunctionContext<'input>),
	BoolAnyContext(BoolAnyContext<'input>),
	UnaryNotContext(UnaryNotContext<'input>),
Error(PatternMiscContext<'input>)
}
antlr_rust::tid!{PatternMiscContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for PatternMiscContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for PatternMiscContextAll<'input>{}

impl<'input> Deref for PatternMiscContextAll<'input>{
	type Target = dyn PatternMiscContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use PatternMiscContextAll::*;
		match self{
			ParenthesesContext(inner) => inner,
			IntRangeContext(inner) => inner,
			UnaryNegateContext(inner) => inner,
			StrExactlyContext(inner) => inner,
			IfThenElseContext(inner) => inner,
			BoolFalseContext(inner) => inner,
			EnumAnyContext(inner) => inner,
			DtAnyContext(inner) => inner,
			AnyContext(inner) => inner,
			IntAnyContext(inner) => inner,
			DatatypeBindingOrConstantContext(inner) => inner,
			EnumSetContext(inner) => inner,
			StrAnyContext(inner) => inner,
			BoolTrueContext(inner) => inner,
			IntAtMostContext(inner) => inner,
			IntAtLeastContext(inner) => inner,
			IntExactlyContext(inner) => inner,
			FunctionContext(inner) => inner,
			BoolAnyContext(inner) => inner,
			UnaryNotContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternMiscContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type PatternMiscContext<'input> = BaseParserRuleContext<'input,PatternMiscContextExt<'input>>;

#[derive(Clone)]
pub struct PatternMiscContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for PatternMiscContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternMiscContext<'input>{
}

impl<'input> CustomRuleContext<'input> for PatternMiscContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}
antlr_rust::tid!{PatternMiscContextExt<'a>}

impl<'input> PatternMiscContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternMiscContextAll<'input>> {
		Rc::new(
		PatternMiscContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternMiscContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait PatternMiscContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternMiscContextExt<'input>>{


}

impl<'input> PatternMiscContextAttrs<'input> for PatternMiscContext<'input>{}

pub type ParenthesesContext<'input> = BaseParserRuleContext<'input,ParenthesesContextExt<'input>>;

pub trait ParenthesesContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token OpenParen
	/// Returns `None` if there is no child corresponding to token OpenParen
	fn OpenParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(OpenParen, 0)
	}
	fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token CloseParen
	/// Returns `None` if there is no child corresponding to token CloseParen
	fn CloseParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(CloseParen, 0)
	}
}

impl<'input> ParenthesesContextAttrs<'input> for ParenthesesContext<'input>{}

pub struct ParenthesesContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParenthesesContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for ParenthesesContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParenthesesContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parentheses(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParenthesesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for ParenthesesContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for ParenthesesContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for ParenthesesContext<'input> {}

impl<'input> ParenthesesContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::ParenthesesContext(
				BaseParserRuleContext::copy_from(ctx,ParenthesesContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntRangeContext<'input> = BaseParserRuleContext<'input,IntRangeContextExt<'input>>;

pub trait IntRangeContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn integer_all(&self) ->  Vec<Rc<IntegerContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn integer(&self, i: usize) -> Option<Rc<IntegerContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Range
	/// Returns `None` if there is no child corresponding to token Range
	fn Range(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Range, 0)
	}
}

impl<'input> IntRangeContextAttrs<'input> for IntRangeContext<'input>{}

pub struct IntRangeContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IntRangeContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntRangeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntRangeContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_intRange(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntRangeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for IntRangeContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for IntRangeContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for IntRangeContext<'input> {}

impl<'input> IntRangeContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::IntRangeContext(
				BaseParserRuleContext::copy_from(ctx,IntRangeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UnaryNegateContext<'input> = BaseParserRuleContext<'input,UnaryNegateContextExt<'input>>;

pub trait UnaryNegateContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Minus
	/// Returns `None` if there is no child corresponding to token Minus
	fn Minus(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Minus, 0)
	}
	fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> UnaryNegateContextAttrs<'input> for UnaryNegateContext<'input>{}

pub struct UnaryNegateContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{UnaryNegateContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for UnaryNegateContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for UnaryNegateContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unaryNegate(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryNegateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for UnaryNegateContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for UnaryNegateContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for UnaryNegateContext<'input> {}

impl<'input> UnaryNegateContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::UnaryNegateContext(
				BaseParserRuleContext::copy_from(ctx,UnaryNegateContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StrExactlyContext<'input> = BaseParserRuleContext<'input,StrExactlyContextExt<'input>>;

pub trait StrExactlyContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token String
	/// Returns `None` if there is no child corresponding to token String
	fn String(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(String, 0)
	}
}

impl<'input> StrExactlyContextAttrs<'input> for StrExactlyContext<'input>{}

pub struct StrExactlyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StrExactlyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for StrExactlyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StrExactlyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_strExactly(self);
	}
}

impl<'input> CustomRuleContext<'input> for StrExactlyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for StrExactlyContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for StrExactlyContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for StrExactlyContext<'input> {}

impl<'input> StrExactlyContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::StrExactlyContext(
				BaseParserRuleContext::copy_from(ctx,StrExactlyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IfThenElseContext<'input> = BaseParserRuleContext<'input,IfThenElseContextExt<'input>>;

pub trait IfThenElseContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token If
	/// Returns `None` if there is no child corresponding to token If
	fn If(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(If, 0)
	}
	fn pattern_all(&self) ->  Vec<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn pattern(&self, i: usize) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Then
	/// Returns `None` if there is no child corresponding to token Then
	fn Then(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Then, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Else
	/// Returns `None` if there is no child corresponding to token Else
	fn Else(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Else, 0)
	}
}

impl<'input> IfThenElseContextAttrs<'input> for IfThenElseContext<'input>{}

pub struct IfThenElseContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IfThenElseContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IfThenElseContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IfThenElseContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ifThenElse(self);
	}
}

impl<'input> CustomRuleContext<'input> for IfThenElseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for IfThenElseContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for IfThenElseContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for IfThenElseContext<'input> {}

impl<'input> IfThenElseContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::IfThenElseContext(
				BaseParserRuleContext::copy_from(ctx,IfThenElseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BoolFalseContext<'input> = BaseParserRuleContext<'input,BoolFalseContextExt<'input>>;

pub trait BoolFalseContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token False
	/// Returns `None` if there is no child corresponding to token False
	fn False(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(False, 0)
	}
}

impl<'input> BoolFalseContextAttrs<'input> for BoolFalseContext<'input>{}

pub struct BoolFalseContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BoolFalseContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for BoolFalseContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for BoolFalseContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_boolFalse(self);
	}
}

impl<'input> CustomRuleContext<'input> for BoolFalseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for BoolFalseContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for BoolFalseContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for BoolFalseContext<'input> {}

impl<'input> BoolFalseContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::BoolFalseContext(
				BaseParserRuleContext::copy_from(ctx,BoolFalseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EnumAnyContext<'input> = BaseParserRuleContext<'input,EnumAnyContextExt<'input>>;

pub trait EnumAnyContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Metaenum
	/// Returns `None` if there is no child corresponding to token Metaenum
	fn Metaenum(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Metaenum, 0)
	}
}

impl<'input> EnumAnyContextAttrs<'input> for EnumAnyContext<'input>{}

pub struct EnumAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EnumAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for EnumAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for EnumAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_enumAny(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumAnyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for EnumAnyContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for EnumAnyContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for EnumAnyContext<'input> {}

impl<'input> EnumAnyContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::EnumAnyContext(
				BaseParserRuleContext::copy_from(ctx,EnumAnyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DtAnyContext<'input> = BaseParserRuleContext<'input,DtAnyContextExt<'input>>;

pub trait DtAnyContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Typename
	/// Returns `None` if there is no child corresponding to token Typename
	fn Typename(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Typename, 0)
	}
}

impl<'input> DtAnyContextAttrs<'input> for DtAnyContext<'input>{}

pub struct DtAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DtAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for DtAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for DtAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_dtAny(self);
	}
}

impl<'input> CustomRuleContext<'input> for DtAnyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for DtAnyContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for DtAnyContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for DtAnyContext<'input> {}

impl<'input> DtAnyContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::DtAnyContext(
				BaseParserRuleContext::copy_from(ctx,DtAnyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AnyContext<'input> = BaseParserRuleContext<'input,AnyContextExt<'input>>;

pub trait AnyContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Question
	/// Returns `None` if there is no child corresponding to token Question
	fn Question(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Question, 0)
	}
}

impl<'input> AnyContextAttrs<'input> for AnyContext<'input>{}

pub struct AnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_any(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for AnyContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for AnyContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for AnyContext<'input> {}

impl<'input> AnyContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::AnyContext(
				BaseParserRuleContext::copy_from(ctx,AnyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntAnyContext<'input> = BaseParserRuleContext<'input,IntAnyContextExt<'input>>;

pub trait IntAnyContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Metaint
	/// Returns `None` if there is no child corresponding to token Metaint
	fn Metaint(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Metaint, 0)
	}
}

impl<'input> IntAnyContextAttrs<'input> for IntAnyContext<'input>{}

pub struct IntAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IntAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_intAny(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntAnyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for IntAnyContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for IntAnyContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for IntAnyContext<'input> {}

impl<'input> IntAnyContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::IntAnyContext(
				BaseParserRuleContext::copy_from(ctx,IntAnyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DatatypeBindingOrConstantContext<'input> = BaseParserRuleContext<'input,DatatypeBindingOrConstantContextExt<'input>>;

pub trait DatatypeBindingOrConstantContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn identifierPath(&self) -> Option<Rc<IdentifierPathContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn nullability(&self) -> Option<Rc<NullabilityContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn variation(&self) -> Option<Rc<VariationContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn parameters(&self) -> Option<Rc<ParametersContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DatatypeBindingOrConstantContextAttrs<'input> for DatatypeBindingOrConstantContext<'input>{}

pub struct DatatypeBindingOrConstantContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DatatypeBindingOrConstantContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for DatatypeBindingOrConstantContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for DatatypeBindingOrConstantContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_datatypeBindingOrConstant(self);
	}
}

impl<'input> CustomRuleContext<'input> for DatatypeBindingOrConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for DatatypeBindingOrConstantContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for DatatypeBindingOrConstantContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for DatatypeBindingOrConstantContext<'input> {}

impl<'input> DatatypeBindingOrConstantContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::DatatypeBindingOrConstantContext(
				BaseParserRuleContext::copy_from(ctx,DatatypeBindingOrConstantContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EnumSetContext<'input> = BaseParserRuleContext<'input,EnumSetContextExt<'input>>;

pub trait EnumSetContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token OpenCurly
	/// Returns `None` if there is no child corresponding to token OpenCurly
	fn OpenCurly(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(OpenCurly, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
	fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
	/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
	fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Identifier, i)
	}
	/// Retrieves first TerminalNode corresponding to token CloseCurly
	/// Returns `None` if there is no child corresponding to token CloseCurly
	fn CloseCurly(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(CloseCurly, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Comma, i)
	}
}

impl<'input> EnumSetContextAttrs<'input> for EnumSetContext<'input>{}

pub struct EnumSetContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EnumSetContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for EnumSetContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for EnumSetContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_enumSet(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumSetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for EnumSetContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for EnumSetContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for EnumSetContext<'input> {}

impl<'input> EnumSetContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::EnumSetContext(
				BaseParserRuleContext::copy_from(ctx,EnumSetContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StrAnyContext<'input> = BaseParserRuleContext<'input,StrAnyContextExt<'input>>;

pub trait StrAnyContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Metastr
	/// Returns `None` if there is no child corresponding to token Metastr
	fn Metastr(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Metastr, 0)
	}
}

impl<'input> StrAnyContextAttrs<'input> for StrAnyContext<'input>{}

pub struct StrAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StrAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for StrAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StrAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_strAny(self);
	}
}

impl<'input> CustomRuleContext<'input> for StrAnyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for StrAnyContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for StrAnyContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for StrAnyContext<'input> {}

impl<'input> StrAnyContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::StrAnyContext(
				BaseParserRuleContext::copy_from(ctx,StrAnyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BoolTrueContext<'input> = BaseParserRuleContext<'input,BoolTrueContextExt<'input>>;

pub trait BoolTrueContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token True
	/// Returns `None` if there is no child corresponding to token True
	fn True(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(True, 0)
	}
}

impl<'input> BoolTrueContextAttrs<'input> for BoolTrueContext<'input>{}

pub struct BoolTrueContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BoolTrueContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for BoolTrueContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for BoolTrueContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_boolTrue(self);
	}
}

impl<'input> CustomRuleContext<'input> for BoolTrueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for BoolTrueContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for BoolTrueContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for BoolTrueContext<'input> {}

impl<'input> BoolTrueContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::BoolTrueContext(
				BaseParserRuleContext::copy_from(ctx,BoolTrueContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntAtMostContext<'input> = BaseParserRuleContext<'input,IntAtMostContextExt<'input>>;

pub trait IntAtMostContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Range
	/// Returns `None` if there is no child corresponding to token Range
	fn Range(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Range, 0)
	}
	fn integer(&self) -> Option<Rc<IntegerContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IntAtMostContextAttrs<'input> for IntAtMostContext<'input>{}

pub struct IntAtMostContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IntAtMostContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntAtMostContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntAtMostContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_intAtMost(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntAtMostContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for IntAtMostContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for IntAtMostContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for IntAtMostContext<'input> {}

impl<'input> IntAtMostContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::IntAtMostContext(
				BaseParserRuleContext::copy_from(ctx,IntAtMostContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntAtLeastContext<'input> = BaseParserRuleContext<'input,IntAtLeastContextExt<'input>>;

pub trait IntAtLeastContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn integer(&self) -> Option<Rc<IntegerContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token Range
	/// Returns `None` if there is no child corresponding to token Range
	fn Range(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Range, 0)
	}
}

impl<'input> IntAtLeastContextAttrs<'input> for IntAtLeastContext<'input>{}

pub struct IntAtLeastContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IntAtLeastContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntAtLeastContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntAtLeastContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_intAtLeast(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntAtLeastContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for IntAtLeastContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for IntAtLeastContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for IntAtLeastContext<'input> {}

impl<'input> IntAtLeastContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::IntAtLeastContext(
				BaseParserRuleContext::copy_from(ctx,IntAtLeastContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntExactlyContext<'input> = BaseParserRuleContext<'input,IntExactlyContextExt<'input>>;

pub trait IntExactlyContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn integer(&self) -> Option<Rc<IntegerContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IntExactlyContextAttrs<'input> for IntExactlyContext<'input>{}

pub struct IntExactlyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IntExactlyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntExactlyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntExactlyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_intExactly(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntExactlyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for IntExactlyContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for IntExactlyContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for IntExactlyContext<'input> {}

impl<'input> IntExactlyContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::IntExactlyContext(
				BaseParserRuleContext::copy_from(ctx,IntExactlyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FunctionContext<'input> = BaseParserRuleContext<'input,FunctionContextExt<'input>>;

pub trait FunctionContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Identifier
	/// Returns `None` if there is no child corresponding to token Identifier
	fn Identifier(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Identifier, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OpenParen
	/// Returns `None` if there is no child corresponding to token OpenParen
	fn OpenParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(OpenParen, 0)
	}
	/// Retrieves first TerminalNode corresponding to token CloseParen
	/// Returns `None` if there is no child corresponding to token CloseParen
	fn CloseParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(CloseParen, 0)
	}
	fn pattern_all(&self) ->  Vec<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn pattern(&self, i: usize) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Comma, i)
	}
}

impl<'input> FunctionContextAttrs<'input> for FunctionContext<'input>{}

pub struct FunctionContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FunctionContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for FunctionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for FunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_function(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for FunctionContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for FunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for FunctionContext<'input> {}

impl<'input> FunctionContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::FunctionContext(
				BaseParserRuleContext::copy_from(ctx,FunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BoolAnyContext<'input> = BaseParserRuleContext<'input,BoolAnyContextExt<'input>>;

pub trait BoolAnyContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Metabool
	/// Returns `None` if there is no child corresponding to token Metabool
	fn Metabool(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Metabool, 0)
	}
}

impl<'input> BoolAnyContextAttrs<'input> for BoolAnyContext<'input>{}

pub struct BoolAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BoolAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for BoolAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for BoolAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_boolAny(self);
	}
}

impl<'input> CustomRuleContext<'input> for BoolAnyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for BoolAnyContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for BoolAnyContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for BoolAnyContext<'input> {}

impl<'input> BoolAnyContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::BoolAnyContext(
				BaseParserRuleContext::copy_from(ctx,BoolAnyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UnaryNotContext<'input> = BaseParserRuleContext<'input,UnaryNotContextExt<'input>>;

pub trait UnaryNotContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token BooleanNot
	/// Returns `None` if there is no child corresponding to token BooleanNot
	fn BooleanNot(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(BooleanNot, 0)
	}
	fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> UnaryNotContextAttrs<'input> for UnaryNotContext<'input>{}

pub struct UnaryNotContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{UnaryNotContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for UnaryNotContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for UnaryNotContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unaryNot(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryNotContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for UnaryNotContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for UnaryNotContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for UnaryNotContext<'input> {}

impl<'input> UnaryNotContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::UnaryNotContext(
				BaseParserRuleContext::copy_from(ctx,UnaryNotContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternMisc(&mut self,)
	-> Result<Rc<PatternMiscContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternMiscContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_patternMisc);
        let mut _localctx: Rc<PatternMiscContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(282);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
				1 =>{
					let tmp = ParenthesesContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(215);
					recog.base.match_token(OpenParen,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(216);
					recog.pattern()?;

					recog.base.set_state(217);
					recog.base.match_token(CloseParen,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = IfThenElseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(219);
					recog.base.match_token(If,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(220);
					recog.pattern()?;

					recog.base.set_state(221);
					recog.base.match_token(Then,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(222);
					recog.pattern()?;

					recog.base.set_state(223);
					recog.base.match_token(Else,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(224);
					recog.pattern()?;

					}
				}
			,
				3 =>{
					let tmp = UnaryNotContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(226);
					recog.base.match_token(BooleanNot,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(227);
					recog.pattern()?;

					}
				}
			,
				4 =>{
					let tmp = AnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(228);
					recog.base.match_token(Question,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					let tmp = BoolAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(229);
					recog.base.match_token(Metabool,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					let tmp = BoolTrueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(230);
					recog.base.match_token(True,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					let tmp = BoolFalseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(231);
					recog.base.match_token(False,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					let tmp = IntAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(232);
					recog.base.match_token(Metaint,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					let tmp = IntRangeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9);
					_localctx = tmp;
					{
					/*InvokeRule integer*/
					recog.base.set_state(233);
					recog.integer()?;

					recog.base.set_state(234);
					recog.base.match_token(Range,&mut recog.err_handler)?;

					/*InvokeRule integer*/
					recog.base.set_state(235);
					recog.integer()?;

					}
				}
			,
				10 =>{
					let tmp = IntAtLeastContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10);
					_localctx = tmp;
					{
					/*InvokeRule integer*/
					recog.base.set_state(237);
					recog.integer()?;

					recog.base.set_state(238);
					recog.base.match_token(Range,&mut recog.err_handler)?;

					}
				}
			,
				11 =>{
					let tmp = IntAtMostContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11);
					_localctx = tmp;
					{
					recog.base.set_state(240);
					recog.base.match_token(Range,&mut recog.err_handler)?;

					/*InvokeRule integer*/
					recog.base.set_state(241);
					recog.integer()?;

					}
				}
			,
				12 =>{
					let tmp = IntExactlyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12);
					_localctx = tmp;
					{
					/*InvokeRule integer*/
					recog.base.set_state(242);
					recog.integer()?;

					}
				}
			,
				13 =>{
					let tmp = EnumAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 13);
					_localctx = tmp;
					{
					recog.base.set_state(243);
					recog.base.match_token(Metaenum,&mut recog.err_handler)?;

					}
				}
			,
				14 =>{
					let tmp = EnumSetContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 14);
					_localctx = tmp;
					{
					recog.base.set_state(244);
					recog.base.match_token(OpenCurly,&mut recog.err_handler)?;

					recog.base.set_state(245);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(250);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==Comma {
						{
						{
						recog.base.set_state(246);
						recog.base.match_token(Comma,&mut recog.err_handler)?;

						recog.base.set_state(247);
						recog.base.match_token(Identifier,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(252);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(253);
					recog.base.match_token(CloseCurly,&mut recog.err_handler)?;

					}
				}
			,
				15 =>{
					let tmp = StrAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 15);
					_localctx = tmp;
					{
					recog.base.set_state(254);
					recog.base.match_token(Metastr,&mut recog.err_handler)?;

					}
				}
			,
				16 =>{
					let tmp = StrExactlyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 16);
					_localctx = tmp;
					{
					recog.base.set_state(255);
					recog.base.match_token(String,&mut recog.err_handler)?;

					}
				}
			,
				17 =>{
					let tmp = DtAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 17);
					_localctx = tmp;
					{
					recog.base.set_state(256);
					recog.base.match_token(Typename,&mut recog.err_handler)?;

					}
				}
			,
				18 =>{
					let tmp = FunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 18);
					_localctx = tmp;
					{
					recog.base.set_state(257);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(258);
					recog.base.match_token(OpenParen,&mut recog.err_handler)?;

					recog.base.set_state(267);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << If) | (1usize << True) | (1usize << False) | (1usize << Metabool) | (1usize << Metaint) | (1usize << Metaenum) | (1usize << Metastr) | (1usize << Typename) | (1usize << Question) | (1usize << OpenParen) | (1usize << OpenCurly))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (BooleanNot - 33)) | (1usize << (Plus - 33)) | (1usize << (Minus - 33)) | (1usize << (Range - 33)) | (1usize << (Nonzero - 33)) | (1usize << (Zero - 33)) | (1usize << (String - 33)) | (1usize << (Identifier - 33)))) != 0) {
						{
						/*InvokeRule pattern*/
						recog.base.set_state(259);
						recog.pattern()?;

						recog.base.set_state(264);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==Comma {
							{
							{
							recog.base.set_state(260);
							recog.base.match_token(Comma,&mut recog.err_handler)?;

							/*InvokeRule pattern*/
							recog.base.set_state(261);
							recog.pattern()?;

							}
							}
							recog.base.set_state(266);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(269);
					recog.base.match_token(CloseParen,&mut recog.err_handler)?;

					}
				}
			,
				19 =>{
					let tmp = DatatypeBindingOrConstantContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 19);
					_localctx = tmp;
					{
					/*InvokeRule identifierPath*/
					recog.base.set_state(270);
					recog.identifierPath()?;

					recog.base.set_state(272);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Question {
						{
						/*InvokeRule nullability*/
						recog.base.set_state(271);
						recog.nullability()?;

						}
					}

					recog.base.set_state(275);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(25,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule variation*/
							recog.base.set_state(274);
							recog.variation()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(278);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule parameters*/
							recog.base.set_state(277);
							recog.parameters()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				20 =>{
					let tmp = UnaryNegateContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 20);
					_localctx = tmp;
					{
					recog.base.set_state(280);
					recog.base.match_token(Minus,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(281);
					recog.pattern()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- nullability ----------------
pub type NullabilityContextAll<'input> = NullabilityContext<'input>;


pub type NullabilityContext<'input> = BaseParserRuleContext<'input,NullabilityContextExt<'input>>;

#[derive(Clone)]
pub struct NullabilityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for NullabilityContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NullabilityContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_nullability(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_nullability(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for NullabilityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nullability }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nullability }
}
antlr_rust::tid!{NullabilityContextExt<'a>}

impl<'input> NullabilityContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NullabilityContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NullabilityContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NullabilityContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<NullabilityContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Question
/// Returns `None` if there is no child corresponding to token Question
fn Question(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Question, 0)
}
fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> NullabilityContextAttrs<'input> for NullabilityContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nullability(&mut self,)
	-> Result<Rc<NullabilityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NullabilityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_nullability);
        let mut _localctx: Rc<NullabilityContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(284);
			recog.base.match_token(Question,&mut recog.err_handler)?;

			recog.base.set_state(286);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule pattern*/
					recog.base.set_state(285);
					recog.pattern()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variation ----------------
pub type VariationContextAll<'input> = VariationContext<'input>;


pub type VariationContext<'input> = BaseParserRuleContext<'input,VariationContextExt<'input>>;

#[derive(Clone)]
pub struct VariationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for VariationContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VariationContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variation(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_variation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for VariationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variation }
}
antlr_rust::tid!{VariationContextExt<'a>}

impl<'input> VariationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariationContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<VariationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OpenSquare
/// Returns `None` if there is no child corresponding to token OpenSquare
fn OpenSquare(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(OpenSquare, 0)
}
fn variationBody(&self) -> Option<Rc<VariationBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CloseSquare
/// Returns `None` if there is no child corresponding to token CloseSquare
fn CloseSquare(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(CloseSquare, 0)
}

}

impl<'input> VariationContextAttrs<'input> for VariationContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variation(&mut self,)
	-> Result<Rc<VariationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_variation);
        let mut _localctx: Rc<VariationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(288);
			recog.base.match_token(OpenSquare,&mut recog.err_handler)?;

			/*InvokeRule variationBody*/
			recog.base.set_state(289);
			recog.variationBody()?;

			recog.base.set_state(290);
			recog.base.match_token(CloseSquare,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variationBody ----------------
#[derive(Debug)]
pub enum VariationBodyContextAll<'input>{
	VarAnyContext(VarAnyContext<'input>),
	VarSystemPreferredContext(VarSystemPreferredContext<'input>),
	VarUserDefinedContext(VarUserDefinedContext<'input>),
Error(VariationBodyContext<'input>)
}
antlr_rust::tid!{VariationBodyContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for VariationBodyContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for VariationBodyContextAll<'input>{}

impl<'input> Deref for VariationBodyContextAll<'input>{
	type Target = dyn VariationBodyContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use VariationBodyContextAll::*;
		match self{
			VarAnyContext(inner) => inner,
			VarSystemPreferredContext(inner) => inner,
			VarUserDefinedContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VariationBodyContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type VariationBodyContext<'input> = BaseParserRuleContext<'input,VariationBodyContextExt<'input>>;

#[derive(Clone)]
pub struct VariationBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for VariationBodyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VariationBodyContext<'input>{
}

impl<'input> CustomRuleContext<'input> for VariationBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variationBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variationBody }
}
antlr_rust::tid!{VariationBodyContextExt<'a>}

impl<'input> VariationBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariationBodyContextAll<'input>> {
		Rc::new(
		VariationBodyContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariationBodyContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait VariationBodyContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<VariationBodyContextExt<'input>>{


}

impl<'input> VariationBodyContextAttrs<'input> for VariationBodyContext<'input>{}

pub type VarAnyContext<'input> = BaseParserRuleContext<'input,VarAnyContextExt<'input>>;

pub trait VarAnyContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Question
	/// Returns `None` if there is no child corresponding to token Question
	fn Question(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Question, 0)
	}
}

impl<'input> VarAnyContextAttrs<'input> for VarAnyContext<'input>{}

pub struct VarAnyContextExt<'input>{
	base:VariationBodyContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{VarAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for VarAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VarAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_varAny(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarAnyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variationBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variationBody }
}

impl<'input> Borrow<VariationBodyContextExt<'input>> for VarAnyContext<'input>{
	fn borrow(&self) -> &VariationBodyContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<VariationBodyContextExt<'input>> for VarAnyContext<'input>{
	fn borrow_mut(&mut self) -> &mut VariationBodyContextExt<'input> { &mut self.base }
}

impl<'input> VariationBodyContextAttrs<'input> for VarAnyContext<'input> {}

impl<'input> VarAnyContextExt<'input>{
	fn new(ctx: &dyn VariationBodyContextAttrs<'input>) -> Rc<VariationBodyContextAll<'input>>  {
		Rc::new(
			VariationBodyContextAll::VarAnyContext(
				BaseParserRuleContext::copy_from(ctx,VarAnyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VarSystemPreferredContext<'input> = BaseParserRuleContext<'input,VarSystemPreferredContextExt<'input>>;

pub trait VarSystemPreferredContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Zero
	/// Returns `None` if there is no child corresponding to token Zero
	fn Zero(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Zero, 0)
	}
}

impl<'input> VarSystemPreferredContextAttrs<'input> for VarSystemPreferredContext<'input>{}

pub struct VarSystemPreferredContextExt<'input>{
	base:VariationBodyContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{VarSystemPreferredContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for VarSystemPreferredContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VarSystemPreferredContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_varSystemPreferred(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarSystemPreferredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variationBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variationBody }
}

impl<'input> Borrow<VariationBodyContextExt<'input>> for VarSystemPreferredContext<'input>{
	fn borrow(&self) -> &VariationBodyContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<VariationBodyContextExt<'input>> for VarSystemPreferredContext<'input>{
	fn borrow_mut(&mut self) -> &mut VariationBodyContextExt<'input> { &mut self.base }
}

impl<'input> VariationBodyContextAttrs<'input> for VarSystemPreferredContext<'input> {}

impl<'input> VarSystemPreferredContextExt<'input>{
	fn new(ctx: &dyn VariationBodyContextAttrs<'input>) -> Rc<VariationBodyContextAll<'input>>  {
		Rc::new(
			VariationBodyContextAll::VarSystemPreferredContext(
				BaseParserRuleContext::copy_from(ctx,VarSystemPreferredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VarUserDefinedContext<'input> = BaseParserRuleContext<'input,VarUserDefinedContextExt<'input>>;

pub trait VarUserDefinedContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn identifierPath(&self) -> Option<Rc<IdentifierPathContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> VarUserDefinedContextAttrs<'input> for VarUserDefinedContext<'input>{}

pub struct VarUserDefinedContextExt<'input>{
	base:VariationBodyContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{VarUserDefinedContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for VarUserDefinedContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VarUserDefinedContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_varUserDefined(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarUserDefinedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variationBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variationBody }
}

impl<'input> Borrow<VariationBodyContextExt<'input>> for VarUserDefinedContext<'input>{
	fn borrow(&self) -> &VariationBodyContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<VariationBodyContextExt<'input>> for VarUserDefinedContext<'input>{
	fn borrow_mut(&mut self) -> &mut VariationBodyContextExt<'input> { &mut self.base }
}

impl<'input> VariationBodyContextAttrs<'input> for VarUserDefinedContext<'input> {}

impl<'input> VarUserDefinedContextExt<'input>{
	fn new(ctx: &dyn VariationBodyContextAttrs<'input>) -> Rc<VariationBodyContextAll<'input>>  {
		Rc::new(
			VariationBodyContextAll::VarUserDefinedContext(
				BaseParserRuleContext::copy_from(ctx,VarUserDefinedContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variationBody(&mut self,)
	-> Result<Rc<VariationBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariationBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_variationBody);
        let mut _localctx: Rc<VariationBodyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(295);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Question 
				=> {
					let tmp = VarAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(292);
					recog.base.match_token(Question,&mut recog.err_handler)?;

					}
				}

			 Zero 
				=> {
					let tmp = VarSystemPreferredContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(293);
					recog.base.match_token(Zero,&mut recog.err_handler)?;

					}
				}

			 Identifier 
				=> {
					let tmp = VarUserDefinedContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule identifierPath*/
					recog.base.set_state(294);
					recog.identifierPath()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameters ----------------
pub type ParametersContextAll<'input> = ParametersContext<'input>;


pub type ParametersContext<'input> = BaseParserRuleContext<'input,ParametersContextExt<'input>>;

#[derive(Clone)]
pub struct ParametersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for ParametersContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParametersContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameters(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_parameters(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ParametersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameters }
}
antlr_rust::tid!{ParametersContextExt<'a>}

impl<'input> ParametersContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParametersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParametersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParametersContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<ParametersContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LessThan
/// Returns `None` if there is no child corresponding to token LessThan
fn LessThan(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(LessThan, 0)
}
/// Retrieves first TerminalNode corresponding to token GreaterThan
/// Returns `None` if there is no child corresponding to token GreaterThan
fn GreaterThan(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(GreaterThan, 0)
}
fn parameter_all(&self) ->  Vec<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parameter(&self, i: usize) -> Option<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Comma, i)
}

}

impl<'input> ParametersContextAttrs<'input> for ParametersContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameters(&mut self,)
	-> Result<Rc<ParametersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_parameters);
        let mut _localctx: Rc<ParametersContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(297);
			recog.base.match_token(LessThan,&mut recog.err_handler)?;

			recog.base.set_state(306);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << If) | (1usize << Null) | (1usize << True) | (1usize << False) | (1usize << Metabool) | (1usize << Metaint) | (1usize << Metaenum) | (1usize << Metastr) | (1usize << Typename) | (1usize << Question) | (1usize << OpenParen) | (1usize << OpenCurly))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (BooleanNot - 33)) | (1usize << (Plus - 33)) | (1usize << (Minus - 33)) | (1usize << (Range - 33)) | (1usize << (Nonzero - 33)) | (1usize << (Zero - 33)) | (1usize << (String - 33)) | (1usize << (Identifier - 33)))) != 0) {
				{
				/*InvokeRule parameter*/
				recog.base.set_state(298);
				recog.parameter()?;

				recog.base.set_state(303);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==Comma {
					{
					{
					recog.base.set_state(299);
					recog.base.match_token(Comma,&mut recog.err_handler)?;

					/*InvokeRule parameter*/
					recog.base.set_state(300);
					recog.parameter()?;

					}
					}
					recog.base.set_state(305);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(308);
			recog.base.match_token(GreaterThan,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameter ----------------
pub type ParameterContextAll<'input> = ParameterContext<'input>;


pub type ParameterContext<'input> = BaseParserRuleContext<'input,ParameterContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for ParameterContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameter(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_parameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameter }
}
antlr_rust::tid!{ParameterContextExt<'a>}

impl<'input> ParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<ParameterContextExt<'input>>{

fn parameterValue(&self) -> Option<Rc<ParameterValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifierOrString(&self) -> Option<Rc<IdentifierOrStringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Colon, 0)
}

}

impl<'input> ParameterContextAttrs<'input> for ParameterContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameter(&mut self,)
	-> Result<Rc<ParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_parameter);
        let mut _localctx: Rc<ParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(313);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule identifierOrString*/
					recog.base.set_state(310);
					recog.identifierOrString()?;

					recog.base.set_state(311);
					recog.base.match_token(Colon,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule parameterValue*/
			recog.base.set_state(315);
			recog.parameterValue()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameterValue ----------------
#[derive(Debug)]
pub enum ParameterValueContextAll<'input>{
	SpecifiedContext(SpecifiedContext<'input>),
	NullContext(NullContext<'input>),
Error(ParameterValueContext<'input>)
}
antlr_rust::tid!{ParameterValueContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ParameterValueContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for ParameterValueContextAll<'input>{}

impl<'input> Deref for ParameterValueContextAll<'input>{
	type Target = dyn ParameterValueContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ParameterValueContextAll::*;
		match self{
			SpecifiedContext(inner) => inner,
			NullContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParameterValueContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ParameterValueContext<'input> = BaseParserRuleContext<'input,ParameterValueContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for ParameterValueContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParameterValueContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ParameterValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterValue }
}
antlr_rust::tid!{ParameterValueContextExt<'a>}

impl<'input> ParameterValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterValueContextAll<'input>> {
		Rc::new(
		ParameterValueContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterValueContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ParameterValueContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<ParameterValueContextExt<'input>>{


}

impl<'input> ParameterValueContextAttrs<'input> for ParameterValueContext<'input>{}

pub type SpecifiedContext<'input> = BaseParserRuleContext<'input,SpecifiedContextExt<'input>>;

pub trait SpecifiedContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SpecifiedContextAttrs<'input> for SpecifiedContext<'input>{}

pub struct SpecifiedContextExt<'input>{
	base:ParameterValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SpecifiedContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for SpecifiedContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for SpecifiedContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Specified(self);
	}
}

impl<'input> CustomRuleContext<'input> for SpecifiedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterValue }
}

impl<'input> Borrow<ParameterValueContextExt<'input>> for SpecifiedContext<'input>{
	fn borrow(&self) -> &ParameterValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterValueContextExt<'input>> for SpecifiedContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterValueContextExt<'input> { &mut self.base }
}

impl<'input> ParameterValueContextAttrs<'input> for SpecifiedContext<'input> {}

impl<'input> SpecifiedContextExt<'input>{
	fn new(ctx: &dyn ParameterValueContextAttrs<'input>) -> Rc<ParameterValueContextAll<'input>>  {
		Rc::new(
			ParameterValueContextAll::SpecifiedContext(
				BaseParserRuleContext::copy_from(ctx,SpecifiedContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NullContext<'input> = BaseParserRuleContext<'input,NullContextExt<'input>>;

pub trait NullContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Null
	/// Returns `None` if there is no child corresponding to token Null
	fn Null(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Null, 0)
	}
}

impl<'input> NullContextAttrs<'input> for NullContext<'input>{}

pub struct NullContextExt<'input>{
	base:ParameterValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NullContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NullContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NullContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Null(self);
	}
}

impl<'input> CustomRuleContext<'input> for NullContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterValue }
}

impl<'input> Borrow<ParameterValueContextExt<'input>> for NullContext<'input>{
	fn borrow(&self) -> &ParameterValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterValueContextExt<'input>> for NullContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterValueContextExt<'input> { &mut self.base }
}

impl<'input> ParameterValueContextAttrs<'input> for NullContext<'input> {}

impl<'input> NullContextExt<'input>{
	fn new(ctx: &dyn ParameterValueContextAttrs<'input>) -> Rc<ParameterValueContextAll<'input>>  {
		Rc::new(
			ParameterValueContextAll::NullContext(
				BaseParserRuleContext::copy_from(ctx,NullContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameterValue(&mut self,)
	-> Result<Rc<ParameterValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_parameterValue);
        let mut _localctx: Rc<ParameterValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(319);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Null 
				=> {
					let tmp = NullContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(317);
					recog.base.match_token(Null,&mut recog.err_handler)?;

					}
				}

			 If | True | False | Metabool | Metaint | Metaenum | Metastr | Typename |
			 Question | OpenParen | OpenCurly | BooleanNot | Plus | Minus | Range |
			 Nonzero | Zero | String | Identifier 
				=> {
					let tmp = SpecifiedContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule pattern*/
					recog.base.set_state(318);
					recog.pattern()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- integer ----------------
pub type IntegerContextAll<'input> = IntegerContext<'input>;


pub type IntegerContext<'input> = BaseParserRuleContext<'input,IntegerContextExt<'input>>;

#[derive(Clone)]
pub struct IntegerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for IntegerContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntegerContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_integer(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_integer(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for IntegerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_integer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_integer }
}
antlr_rust::tid!{IntegerContextExt<'a>}

impl<'input> IntegerContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IntegerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntegerContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IntegerContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<IntegerContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Zero
/// Returns `None` if there is no child corresponding to token Zero
fn Zero(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Zero, 0)
}
/// Retrieves first TerminalNode corresponding to token Nonzero
/// Returns `None` if there is no child corresponding to token Nonzero
fn Nonzero(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Nonzero, 0)
}
/// Retrieves first TerminalNode corresponding to token Plus
/// Returns `None` if there is no child corresponding to token Plus
fn Plus(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Plus, 0)
}
/// Retrieves first TerminalNode corresponding to token Minus
/// Returns `None` if there is no child corresponding to token Minus
fn Minus(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Minus, 0)
}

}

impl<'input> IntegerContextAttrs<'input> for IntegerContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn integer(&mut self,)
	-> Result<Rc<IntegerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntegerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_integer);
        let mut _localctx: Rc<IntegerContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(322);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Plus || _la==Minus {
				{
				recog.base.set_state(321);
				_la = recog.base.input.la(1);
				if { !(_la==Plus || _la==Minus) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
			}

			recog.base.set_state(324);
			_la = recog.base.input.la(1);
			if { !(_la==Nonzero || _la==Zero) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- identifierPath ----------------
pub type IdentifierPathContextAll<'input> = IdentifierPathContext<'input>;


pub type IdentifierPathContext<'input> = BaseParserRuleContext<'input,IdentifierPathContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierPathContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for IdentifierPathContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IdentifierPathContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identifierPath(self);
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
			listener.exit_identifierPath(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for IdentifierPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierPath }
}
antlr_rust::tid!{IdentifierPathContextExt<'a>}

impl<'input> IdentifierPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierPathContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<IdentifierPathContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Identifier, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Period in current rule
fn Period_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Period, starting from 0.
/// Returns `None` if number of children corresponding to token Period is less or equal than `i`.
fn Period(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(Period, i)
}

}

impl<'input> IdentifierPathContextAttrs<'input> for IdentifierPathContext<'input>{}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifierPath(&mut self,)
	-> Result<Rc<IdentifierPathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierPathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_identifierPath);
        let mut _localctx: Rc<IdentifierPathContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(330);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(35,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(326);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					recog.base.set_state(327);
					recog.base.match_token(Period,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(332);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(35,&mut recog.base)?;
			}
			recog.base.set_state(333);
			recog.base.match_token(Identifier,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- identifierOrString ----------------
#[derive(Debug)]
pub enum IdentifierOrStringContextAll<'input>{
	StrContext(StrContext<'input>),
	IdentContext(IdentContext<'input>),
Error(IdentifierOrStringContext<'input>)
}
antlr_rust::tid!{IdentifierOrStringContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for IdentifierOrStringContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for IdentifierOrStringContextAll<'input>{}

impl<'input> Deref for IdentifierOrStringContextAll<'input>{
	type Target = dyn IdentifierOrStringContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use IdentifierOrStringContextAll::*;
		match self{
			StrContext(inner) => inner,
			IdentContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IdentifierOrStringContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type IdentifierOrStringContext<'input> = BaseParserRuleContext<'input,IdentifierOrStringContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierOrStringContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for IdentifierOrStringContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IdentifierOrStringContext<'input>{
}

impl<'input> CustomRuleContext<'input> for IdentifierOrStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierOrString }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierOrString }
}
antlr_rust::tid!{IdentifierOrStringContextExt<'a>}

impl<'input> IdentifierOrStringContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierOrStringContextAll<'input>> {
		Rc::new(
		IdentifierOrStringContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierOrStringContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait IdentifierOrStringContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<IdentifierOrStringContextExt<'input>>{


}

impl<'input> IdentifierOrStringContextAttrs<'input> for IdentifierOrStringContext<'input>{}

pub type StrContext<'input> = BaseParserRuleContext<'input,StrContextExt<'input>>;

pub trait StrContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token String
	/// Returns `None` if there is no child corresponding to token String
	fn String(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(String, 0)
	}
}

impl<'input> StrContextAttrs<'input> for StrContext<'input>{}

pub struct StrContextExt<'input>{
	base:IdentifierOrStringContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StrContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for StrContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StrContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Str(self);
	}
}

impl<'input> CustomRuleContext<'input> for StrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierOrString }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierOrString }
}

impl<'input> Borrow<IdentifierOrStringContextExt<'input>> for StrContext<'input>{
	fn borrow(&self) -> &IdentifierOrStringContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<IdentifierOrStringContextExt<'input>> for StrContext<'input>{
	fn borrow_mut(&mut self) -> &mut IdentifierOrStringContextExt<'input> { &mut self.base }
}

impl<'input> IdentifierOrStringContextAttrs<'input> for StrContext<'input> {}

impl<'input> StrContextExt<'input>{
	fn new(ctx: &dyn IdentifierOrStringContextAttrs<'input>) -> Rc<IdentifierOrStringContextAll<'input>>  {
		Rc::new(
			IdentifierOrStringContextAll::StrContext(
				BaseParserRuleContext::copy_from(ctx,StrContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdentContext<'input> = BaseParserRuleContext<'input,IdentContextExt<'input>>;

pub trait IdentContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Identifier
	/// Returns `None` if there is no child corresponding to token Identifier
	fn Identifier(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(Identifier, 0)
	}
}

impl<'input> IdentContextAttrs<'input> for IdentContext<'input>{}

pub struct IdentContextExt<'input>{
	base:IdentifierOrStringContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IdentContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IdentContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IdentContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Ident(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierOrString }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierOrString }
}

impl<'input> Borrow<IdentifierOrStringContextExt<'input>> for IdentContext<'input>{
	fn borrow(&self) -> &IdentifierOrStringContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<IdentifierOrStringContextExt<'input>> for IdentContext<'input>{
	fn borrow_mut(&mut self) -> &mut IdentifierOrStringContextExt<'input> { &mut self.base }
}

impl<'input> IdentifierOrStringContextAttrs<'input> for IdentContext<'input> {}

impl<'input> IdentContextExt<'input>{
	fn new(ctx: &dyn IdentifierOrStringContextAttrs<'input>) -> Rc<IdentifierOrStringContextAll<'input>>  {
		Rc::new(
			IdentifierOrStringContextAll::IdentContext(
				BaseParserRuleContext::copy_from(ctx,IdentContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitTypeParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifierOrString(&mut self,)
	-> Result<Rc<IdentifierOrStringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierOrStringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_identifierOrString);
        let mut _localctx: Rc<IdentifierOrStringContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(337);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 String 
				=> {
					let tmp = StrContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(335);
					recog.base.match_token(String,&mut recog.err_handler)?;

					}
				}

			 Identifier 
				=> {
					let tmp = IdentContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(336);
					recog.base.match_token(Identifier,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x32\u{156}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x03\x02\x07\x02\x3c\x0a\x02\x0c\x02\x0e\x02\
	\x3f\x0b\x02\x03\x02\x07\x02\x42\x0a\x02\x0c\x02\x0e\x02\x45\x0b\x02\x03\
	\x02\x03\x02\x07\x02\x49\x0a\x02\x0c\x02\x0e\x02\x4c\x0b\x02\x03\x02\x03\
	\x02\x03\x03\x07\x03\x51\x0a\x03\x0c\x03\x0e\x03\x54\x0b\x03\x03\x03\x07\
	\x03\x57\x0a\x03\x0c\x03\x0e\x03\x5a\x0b\x03\x03\x03\x03\x03\x07\x03\x5e\
	\x0a\x03\x0c\x03\x0e\x03\x61\x0b\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\
	\x04\x07\x04\x68\x0a\x04\x0c\x04\x0e\x04\x6b\x0b\x04\x03\x04\x03\x04\x03\
	\x05\x07\x05\x70\x0a\x05\x0c\x05\x0e\x05\x73\x0b\x05\x03\x05\x03\x05\x03\
	\x05\x07\x05\x78\x0a\x05\x0c\x05\x0e\x05\x7b\x0b\x05\x05\x05\x7d\x0a\x05\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\
	\x03\x06\x03\x06\x05\x06\u{8a}\x0a\x06\x03\x07\x03\x07\x03\x08\x03\x08\x03\
	\x08\x03\x08\x07\x08\u{92}\x0a\x08\x0c\x08\x0e\x08\u{95}\x0b\x08\x03\x09\
	\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x07\x0a\u{9d}\x0a\x0a\x0c\x0a\x0e\
	\x0a\u{a0}\x0b\x0a\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\
	\u{a8}\x0a\x0c\x0c\x0c\x0e\x0c\u{ab}\x0b\x0c\x03\x0d\x03\x0d\x05\x0d\u{af}\
	\x0a\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x07\x0e\u{b5}\x0a\x0e\x0c\x0e\x0e\
	\x0e\u{b8}\x0b\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{be}\x0a\x0f\
	\x03\x10\x03\x10\x03\x10\x03\x10\x07\x10\u{c4}\x0a\x10\x0c\x10\x0e\x10\u{c7}\
	\x0b\x10\x03\x11\x03\x11\x05\x11\u{cb}\x0a\x11\x03\x12\x03\x12\x03\x12\x03\
	\x12\x07\x12\u{d1}\x0a\x12\x0c\x12\x0e\x12\u{d4}\x0b\x12\x03\x13\x03\x13\
	\x05\x13\u{d8}\x0a\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\
	\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\
	\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\
	\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x07\
	\x14\u{fb}\x0a\x14\x0c\x14\x0e\x14\u{fe}\x0b\x14\x03\x14\x03\x14\x03\x14\
	\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x07\x14\u{109}\x0a\x14\
	\x0c\x14\x0e\x14\u{10c}\x0b\x14\x05\x14\u{10e}\x0a\x14\x03\x14\x03\x14\x03\
	\x14\x05\x14\u{113}\x0a\x14\x03\x14\x05\x14\u{116}\x0a\x14\x03\x14\x05\x14\
	\u{119}\x0a\x14\x03\x14\x03\x14\x05\x14\u{11d}\x0a\x14\x03\x15\x03\x15\x05\
	\x15\u{121}\x0a\x15\x03\x16\x03\x16\x03\x16\x03\x16\x03\x17\x03\x17\x03\
	\x17\x05\x17\u{12a}\x0a\x17\x03\x18\x03\x18\x03\x18\x03\x18\x07\x18\u{130}\
	\x0a\x18\x0c\x18\x0e\x18\u{133}\x0b\x18\x05\x18\u{135}\x0a\x18\x03\x18\x03\
	\x18\x03\x19\x03\x19\x03\x19\x05\x19\u{13c}\x0a\x19\x03\x19\x03\x19\x03\
	\x1a\x03\x1a\x05\x1a\u{142}\x0a\x1a\x03\x1b\x05\x1b\u{145}\x0a\x1b\x03\x1b\
	\x03\x1b\x03\x1c\x03\x1c\x07\x1c\u{14b}\x0a\x1c\x0c\x1c\x0e\x1c\u{14e}\x0b\
	\x1c\x03\x1c\x03\x1c\x03\x1d\x03\x1d\x05\x1d\u{154}\x0a\x1d\x03\x1d\x02\
	\x02\x1e\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\
	\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x02\x04\x03\x02\x2a\x2b\
	\x03\x02\x2f\x30\x02\u{174}\x02\x3d\x03\x02\x02\x02\x04\x52\x03\x02\x02\
	\x02\x06\x69\x03\x02\x02\x02\x08\x71\x03\x02\x02\x02\x0a\u{89}\x03\x02\x02\
	\x02\x0c\u{8b}\x03\x02\x02\x02\x0e\u{8d}\x03\x02\x02\x02\x10\u{96}\x03\x02\
	\x02\x02\x12\u{98}\x03\x02\x02\x02\x14\u{a1}\x03\x02\x02\x02\x16\u{a3}\x03\
	\x02\x02\x02\x18\u{ae}\x03\x02\x02\x02\x1a\u{b0}\x03\x02\x02\x02\x1c\u{bd}\
	\x03\x02\x02\x02\x1e\u{bf}\x03\x02\x02\x02\x20\u{ca}\x03\x02\x02\x02\x22\
	\u{cc}\x03\x02\x02\x02\x24\u{d7}\x03\x02\x02\x02\x26\u{11c}\x03\x02\x02\
	\x02\x28\u{11e}\x03\x02\x02\x02\x2a\u{122}\x03\x02\x02\x02\x2c\u{129}\x03\
	\x02\x02\x02\x2e\u{12b}\x03\x02\x02\x02\x30\u{13b}\x03\x02\x02\x02\x32\u{141}\
	\x03\x02\x02\x02\x34\u{144}\x03\x02\x02\x02\x36\u{14c}\x03\x02\x02\x02\x38\
	\u{153}\x03\x02\x02\x02\x3a\x3c\x07\x05\x02\x02\x3b\x3a\x03\x02\x02\x02\
	\x3c\x3f\x03\x02\x02\x02\x3d\x3b\x03\x02\x02\x02\x3d\x3e\x03\x02\x02\x02\
	\x3e\x43\x03\x02\x02\x02\x3f\x3d\x03\x02\x02\x02\x40\x42\x07\x06\x02\x02\
	\x41\x40\x03\x02\x02\x02\x42\x45\x03\x02\x02\x02\x43\x41\x03\x02\x02\x02\
	\x43\x44\x03\x02\x02\x02\x44\x46\x03\x02\x02\x02\x45\x43\x03\x02\x02\x02\
	\x46\x4a\x05\x0c\x07\x02\x47\x49\x07\x06\x02\x02\x48\x47\x03\x02\x02\x02\
	\x49\x4c\x03\x02\x02\x02\x4a\x48\x03\x02\x02\x02\x4a\x4b\x03\x02\x02\x02\
	\x4b\x4d\x03\x02\x02\x02\x4c\x4a\x03\x02\x02\x02\x4d\x4e\x07\x02\x02\x03\
	\x4e\x03\x03\x02\x02\x02\x4f\x51\x07\x05\x02\x02\x50\x4f\x03\x02\x02\x02\
	\x51\x54\x03\x02\x02\x02\x52\x50\x03\x02\x02\x02\x52\x53\x03\x02\x02\x02\
	\x53\x58\x03\x02\x02\x02\x54\x52\x03\x02\x02\x02\x55\x57\x07\x06\x02\x02\
	\x56\x55\x03\x02\x02\x02\x57\x5a\x03\x02\x02\x02\x58\x56\x03\x02\x02\x02\
	\x58\x59\x03\x02\x02\x02\x59\x5b\x03\x02\x02\x02\x5a\x58\x03\x02\x02\x02\
	\x5b\x5f\x05\x06\x04\x02\x5c\x5e\x07\x06\x02\x02\x5d\x5c\x03\x02\x02\x02\
	\x5e\x61\x03\x02\x02\x02\x5f\x5d\x03\x02\x02\x02\x5f\x60\x03\x02\x02\x02\
	\x60\x62\x03\x02\x02\x02\x61\x5f\x03\x02\x02\x02\x62\x63\x07\x02\x02\x03\
	\x63\x05\x03\x02\x02\x02\x64\x65\x05\x0a\x06\x02\x65\x66\x05\x08\x05\x02\
	\x66\x68\x03\x02\x02\x02\x67\x64\x03\x02\x02\x02\x68\x6b\x03\x02\x02\x02\
	\x69\x67\x03\x02\x02\x02\x69\x6a\x03\x02\x02\x02\x6a\x6c\x03\x02\x02\x02\
	\x6b\x69\x03\x02\x02\x02\x6c\x6d\x05\x0c\x07\x02\x6d\x07\x03\x02\x02\x02\
	\x6e\x70\x07\x06\x02\x02\x6f\x6e\x03\x02\x02\x02\x70\x73\x03\x02\x02\x02\
	\x71\x6f\x03\x02\x02\x02\x71\x72\x03\x02\x02\x02\x72\x7c\x03\x02\x02\x02\
	\x73\x71\x03\x02\x02\x02\x74\x7d\x07\x06\x02\x02\x75\x79\x07\x18\x02\x02\
	\x76\x78\x07\x06\x02\x02\x77\x76\x03\x02\x02\x02\x78\x7b\x03\x02\x02\x02\
	\x79\x77\x03\x02\x02\x02\x79\x7a\x03\x02\x02\x02\x7a\x7d\x03\x02\x02\x02\
	\x7b\x79\x03\x02\x02\x02\x7c\x74\x03\x02\x02\x02\x7c\x75\x03\x02\x02\x02\
	\x7d\x09\x03\x02\x02\x02\x7e\x7f\x05\x0c\x07\x02\x7f\u{80}\x07\x20\x02\x02\
	\u{80}\u{81}\x05\x0c\x07\x02\u{81}\u{8a}\x03\x02\x02\x02\u{82}\u{83}\x07\
	\x08\x02\x02\u{83}\u{84}\x05\x0c\x07\x02\u{84}\u{85}\x07\x09\x02\x02\u{85}\
	\u{86}\x05\x0c\x07\x02\u{86}\u{8a}\x03\x02\x02\x02\u{87}\u{88}\x07\x08\x02\
	\x02\u{88}\u{8a}\x05\x0c\x07\x02\u{89}\x7e\x03\x02\x02\x02\u{89}\u{82}\x03\
	\x02\x02\x02\u{89}\u{87}\x03\x02\x02\x02\u{8a}\x0b\x03\x02\x02\x02\u{8b}\
	\u{8c}\x05\x0e\x08\x02\u{8c}\x0d\x03\x02\x02\x02\u{8d}\u{93}\x05\x12\x0a\
	\x02\u{8e}\u{8f}\x05\x10\x09\x02\u{8f}\u{90}\x05\x12\x0a\x02\u{90}\u{92}\
	\x03\x02\x02\x02\u{91}\u{8e}\x03\x02\x02\x02\u{92}\u{95}\x03\x02\x02\x02\
	\u{93}\u{91}\x03\x02\x02\x02\u{93}\u{94}\x03\x02\x02\x02\u{94}\x0f\x03\x02\
	\x02\x02\u{95}\u{93}\x03\x02\x02\x02\u{96}\u{97}\x07\x21\x02\x02\u{97}\x11\
	\x03\x02\x02\x02\u{98}\u{9e}\x05\x16\x0c\x02\u{99}\u{9a}\x05\x14\x0b\x02\
	\u{9a}\u{9b}\x05\x16\x0c\x02\u{9b}\u{9d}\x03\x02\x02\x02\u{9c}\u{99}\x03\
	\x02\x02\x02\u{9d}\u{a0}\x03\x02\x02\x02\u{9e}\u{9c}\x03\x02\x02\x02\u{9e}\
	\u{9f}\x03\x02\x02\x02\u{9f}\x13\x03\x02\x02\x02\u{a0}\u{9e}\x03\x02\x02\
	\x02\u{a1}\u{a2}\x07\x22\x02\x02\u{a2}\x15\x03\x02\x02\x02\u{a3}\u{a9}\x05\
	\x1a\x0e\x02\u{a4}\u{a5}\x05\x18\x0d\x02\u{a5}\u{a6}\x05\x1a\x0e\x02\u{a6}\
	\u{a8}\x03\x02\x02\x02\u{a7}\u{a4}\x03\x02\x02\x02\u{a8}\u{ab}\x03\x02\x02\
	\x02\u{a9}\u{a7}\x03\x02\x02\x02\u{a9}\u{aa}\x03\x02\x02\x02\u{aa}\x17\x03\
	\x02\x02\x02\u{ab}\u{a9}\x03\x02\x02\x02\u{ac}\u{af}\x07\x24\x02\x02\u{ad}\
	\u{af}\x07\x25\x02\x02\u{ae}\u{ac}\x03\x02\x02\x02\u{ae}\u{ad}\x03\x02\x02\
	\x02\u{af}\x19\x03\x02\x02\x02\u{b0}\u{b6}\x05\x1e\x10\x02\u{b1}\u{b2}\x05\
	\x1c\x0f\x02\u{b2}\u{b3}\x05\x1e\x10\x02\u{b3}\u{b5}\x03\x02\x02\x02\u{b4}\
	\u{b1}\x03\x02\x02\x02\u{b5}\u{b8}\x03\x02\x02\x02\u{b6}\u{b4}\x03\x02\x02\
	\x02\u{b6}\u{b7}\x03\x02\x02\x02\u{b7}\x1b\x03\x02\x02\x02\u{b8}\u{b6}\x03\
	\x02\x02\x02\u{b9}\u{be}\x07\x26\x02\x02\u{ba}\u{be}\x07\x27\x02\x02\u{bb}\
	\u{be}\x07\x28\x02\x02\u{bc}\u{be}\x07\x29\x02\x02\u{bd}\u{b9}\x03\x02\x02\
	\x02\u{bd}\u{ba}\x03\x02\x02\x02\u{bd}\u{bb}\x03\x02\x02\x02\u{bd}\u{bc}\
	\x03\x02\x02\x02\u{be}\x1d\x03\x02\x02\x02\u{bf}\u{c5}\x05\x22\x12\x02\u{c0}\
	\u{c1}\x05\x20\x11\x02\u{c1}\u{c2}\x05\x22\x12\x02\u{c2}\u{c4}\x03\x02\x02\
	\x02\u{c3}\u{c0}\x03\x02\x02\x02\u{c4}\u{c7}\x03\x02\x02\x02\u{c5}\u{c3}\
	\x03\x02\x02\x02\u{c5}\u{c6}\x03\x02\x02\x02\u{c6}\x1f\x03\x02\x02\x02\u{c7}\
	\u{c5}\x03\x02\x02\x02\u{c8}\u{cb}\x07\x2a\x02\x02\u{c9}\u{cb}\x07\x2b\x02\
	\x02\u{ca}\u{c8}\x03\x02\x02\x02\u{ca}\u{c9}\x03\x02\x02\x02\u{cb}\x21\x03\
	\x02\x02\x02\u{cc}\u{d2}\x05\x26\x14\x02\u{cd}\u{ce}\x05\x24\x13\x02\u{ce}\
	\u{cf}\x05\x26\x14\x02\u{cf}\u{d1}\x03\x02\x02\x02\u{d0}\u{cd}\x03\x02\x02\
	\x02\u{d1}\u{d4}\x03\x02\x02\x02\u{d2}\u{d0}\x03\x02\x02\x02\u{d2}\u{d3}\
	\x03\x02\x02\x02\u{d3}\x23\x03\x02\x02\x02\u{d4}\u{d2}\x03\x02\x02\x02\u{d5}\
	\u{d8}\x07\x2c\x02\x02\u{d6}\u{d8}\x07\x2d\x02\x02\u{d7}\u{d5}\x03\x02\x02\
	\x02\u{d7}\u{d6}\x03\x02\x02\x02\u{d8}\x25\x03\x02\x02\x02\u{d9}\u{da}\x07\
	\x1a\x02\x02\u{da}\u{db}\x05\x0c\x07\x02\u{db}\u{dc}\x07\x1b\x02\x02\u{dc}\
	\u{11d}\x03\x02\x02\x02\u{dd}\u{de}\x07\x0a\x02\x02\u{de}\u{df}\x05\x0c\
	\x07\x02\u{df}\u{e0}\x07\x0b\x02\x02\u{e0}\u{e1}\x05\x0c\x07\x02\u{e1}\u{e2}\
	\x07\x0c\x02\x02\u{e2}\u{e3}\x05\x0c\x07\x02\u{e3}\u{11d}\x03\x02\x02\x02\
	\u{e4}\u{e5}\x07\x23\x02\x02\u{e5}\u{11d}\x05\x0c\x07\x02\u{e6}\u{11d}\x07\
	\x19\x02\x02\u{e7}\u{11d}\x07\x10\x02\x02\u{e8}\u{11d}\x07\x0e\x02\x02\u{e9}\
	\u{11d}\x07\x0f\x02\x02\u{ea}\u{11d}\x07\x11\x02\x02\u{eb}\u{ec}\x05\x34\
	\x1b\x02\u{ec}\u{ed}\x07\x2e\x02\x02\u{ed}\u{ee}\x05\x34\x1b\x02\u{ee}\u{11d}\
	\x03\x02\x02\x02\u{ef}\u{f0}\x05\x34\x1b\x02\u{f0}\u{f1}\x07\x2e\x02\x02\
	\u{f1}\u{11d}\x03\x02\x02\x02\u{f2}\u{f3}\x07\x2e\x02\x02\u{f3}\u{11d}\x05\
	\x34\x1b\x02\u{f4}\u{11d}\x05\x34\x1b\x02\u{f5}\u{11d}\x07\x12\x02\x02\u{f6}\
	\u{f7}\x07\x1c\x02\x02\u{f7}\u{fc}\x07\x32\x02\x02\u{f8}\u{f9}\x07\x16\x02\
	\x02\u{f9}\u{fb}\x07\x32\x02\x02\u{fa}\u{f8}\x03\x02\x02\x02\u{fb}\u{fe}\
	\x03\x02\x02\x02\u{fc}\u{fa}\x03\x02\x02\x02\u{fc}\u{fd}\x03\x02\x02\x02\
	\u{fd}\u{ff}\x03\x02\x02\x02\u{fe}\u{fc}\x03\x02\x02\x02\u{ff}\u{11d}\x07\
	\x1d\x02\x02\u{100}\u{11d}\x07\x13\x02\x02\u{101}\u{11d}\x07\x31\x02\x02\
	\u{102}\u{11d}\x07\x14\x02\x02\u{103}\u{104}\x07\x32\x02\x02\u{104}\u{10d}\
	\x07\x1a\x02\x02\u{105}\u{10a}\x05\x0c\x07\x02\u{106}\u{107}\x07\x16\x02\
	\x02\u{107}\u{109}\x05\x0c\x07\x02\u{108}\u{106}\x03\x02\x02\x02\u{109}\
	\u{10c}\x03\x02\x02\x02\u{10a}\u{108}\x03\x02\x02\x02\u{10a}\u{10b}\x03\
	\x02\x02\x02\u{10b}\u{10e}\x03\x02\x02\x02\u{10c}\u{10a}\x03\x02\x02\x02\
	\u{10d}\u{105}\x03\x02\x02\x02\u{10d}\u{10e}\x03\x02\x02\x02\u{10e}\u{10f}\
	\x03\x02\x02\x02\u{10f}\u{11d}\x07\x1b\x02\x02\u{110}\u{112}\x05\x36\x1c\
	\x02\u{111}\u{113}\x05\x28\x15\x02\u{112}\u{111}\x03\x02\x02\x02\u{112}\
	\u{113}\x03\x02\x02\x02\u{113}\u{115}\x03\x02\x02\x02\u{114}\u{116}\x05\
	\x2a\x16\x02\u{115}\u{114}\x03\x02\x02\x02\u{115}\u{116}\x03\x02\x02\x02\
	\u{116}\u{118}\x03\x02\x02\x02\u{117}\u{119}\x05\x2e\x18\x02\u{118}\u{117}\
	\x03\x02\x02\x02\u{118}\u{119}\x03\x02\x02\x02\u{119}\u{11d}\x03\x02\x02\
	\x02\u{11a}\u{11b}\x07\x2b\x02\x02\u{11b}\u{11d}\x05\x0c\x07\x02\u{11c}\
	\u{d9}\x03\x02\x02\x02\u{11c}\u{dd}\x03\x02\x02\x02\u{11c}\u{e4}\x03\x02\
	\x02\x02\u{11c}\u{e6}\x03\x02\x02\x02\u{11c}\u{e7}\x03\x02\x02\x02\u{11c}\
	\u{e8}\x03\x02\x02\x02\u{11c}\u{e9}\x03\x02\x02\x02\u{11c}\u{ea}\x03\x02\
	\x02\x02\u{11c}\u{eb}\x03\x02\x02\x02\u{11c}\u{ef}\x03\x02\x02\x02\u{11c}\
	\u{f2}\x03\x02\x02\x02\u{11c}\u{f4}\x03\x02\x02\x02\u{11c}\u{f5}\x03\x02\
	\x02\x02\u{11c}\u{f6}\x03\x02\x02\x02\u{11c}\u{100}\x03\x02\x02\x02\u{11c}\
	\u{101}\x03\x02\x02\x02\u{11c}\u{102}\x03\x02\x02\x02\u{11c}\u{103}\x03\
	\x02\x02\x02\u{11c}\u{110}\x03\x02\x02\x02\u{11c}\u{11a}\x03\x02\x02\x02\
	\u{11d}\x27\x03\x02\x02\x02\u{11e}\u{120}\x07\x19\x02\x02\u{11f}\u{121}\
	\x05\x0c\x07\x02\u{120}\u{11f}\x03\x02\x02\x02\u{120}\u{121}\x03\x02\x02\
	\x02\u{121}\x29\x03\x02\x02\x02\u{122}\u{123}\x07\x1e\x02\x02\u{123}\u{124}\
	\x05\x2c\x17\x02\u{124}\u{125}\x07\x1f\x02\x02\u{125}\x2b\x03\x02\x02\x02\
	\u{126}\u{12a}\x07\x19\x02\x02\u{127}\u{12a}\x07\x30\x02\x02\u{128}\u{12a}\
	\x05\x36\x1c\x02\u{129}\u{126}\x03\x02\x02\x02\u{129}\u{127}\x03\x02\x02\
	\x02\u{129}\u{128}\x03\x02\x02\x02\u{12a}\x2d\x03\x02\x02\x02\u{12b}\u{134}\
	\x07\x26\x02\x02\u{12c}\u{131}\x05\x30\x19\x02\u{12d}\u{12e}\x07\x16\x02\
	\x02\u{12e}\u{130}\x05\x30\x19\x02\u{12f}\u{12d}\x03\x02\x02\x02\u{130}\
	\u{133}\x03\x02\x02\x02\u{131}\u{12f}\x03\x02\x02\x02\u{131}\u{132}\x03\
	\x02\x02\x02\u{132}\u{135}\x03\x02\x02\x02\u{133}\u{131}\x03\x02\x02\x02\
	\u{134}\u{12c}\x03\x02\x02\x02\u{134}\u{135}\x03\x02\x02\x02\u{135}\u{136}\
	\x03\x02\x02\x02\u{136}\u{137}\x07\x28\x02\x02\u{137}\x2f\x03\x02\x02\x02\
	\u{138}\u{139}\x05\x38\x1d\x02\u{139}\u{13a}\x07\x17\x02\x02\u{13a}\u{13c}\
	\x03\x02\x02\x02\u{13b}\u{138}\x03\x02\x02\x02\u{13b}\u{13c}\x03\x02\x02\
	\x02\u{13c}\u{13d}\x03\x02\x02\x02\u{13d}\u{13e}\x05\x32\x1a\x02\u{13e}\
	\x31\x03\x02\x02\x02\u{13f}\u{142}\x07\x0d\x02\x02\u{140}\u{142}\x05\x0c\
	\x07\x02\u{141}\u{13f}\x03\x02\x02\x02\u{141}\u{140}\x03\x02\x02\x02\u{142}\
	\x33\x03\x02\x02\x02\u{143}\u{145}\x09\x02\x02\x02\u{144}\u{143}\x03\x02\
	\x02\x02\u{144}\u{145}\x03\x02\x02\x02\u{145}\u{146}\x03\x02\x02\x02\u{146}\
	\u{147}\x09\x03\x02\x02\u{147}\x35\x03\x02\x02\x02\u{148}\u{149}\x07\x32\
	\x02\x02\u{149}\u{14b}\x07\x15\x02\x02\u{14a}\u{148}\x03\x02\x02\x02\u{14b}\
	\u{14e}\x03\x02\x02\x02\u{14c}\u{14a}\x03\x02\x02\x02\u{14c}\u{14d}\x03\
	\x02\x02\x02\u{14d}\u{14f}\x03\x02\x02\x02\u{14e}\u{14c}\x03\x02\x02\x02\
	\u{14f}\u{150}\x07\x32\x02\x02\u{150}\x37\x03\x02\x02\x02\u{151}\u{154}\
	\x07\x31\x02\x02\u{152}\u{154}\x07\x32\x02\x02\u{153}\u{151}\x03\x02\x02\
	\x02\u{153}\u{152}\x03\x02\x02\x02\u{154}\x39\x03\x02\x02\x02\x27\x3d\x43\
	\x4a\x52\x58\x5f\x69\x71\x79\x7c\u{89}\u{93}\u{9e}\u{a9}\u{ae}\u{b6}\u{bd}\
	\u{c5}\u{ca}\u{d2}\u{d7}\u{fc}\u{10a}\u{10d}\u{112}\u{115}\u{118}\u{11c}\
	\u{120}\u{129}\u{131}\u{134}\u{13b}\u{141}\u{144}\u{14c}\u{153}";

