// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]
// Generated from SubstraitType.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::substraittypelistener::*;
use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const SubstraitType_LineComment:i32=1; 
		pub const SubstraitType_BlockComment:i32=2; 
		pub const SubstraitType_Whitespace:i32=3; 
		pub const SubstraitType_Newline:i32=4; 
		pub const SubstraitType_EscNewline:i32=5; 
		pub const SubstraitType_Assert:i32=6; 
		pub const SubstraitType_Matches:i32=7; 
		pub const SubstraitType_If:i32=8; 
		pub const SubstraitType_Then:i32=9; 
		pub const SubstraitType_Else:i32=10; 
		pub const SubstraitType_Null:i32=11; 
		pub const SubstraitType_True:i32=12; 
		pub const SubstraitType_False:i32=13; 
		pub const SubstraitType_Metabool:i32=14; 
		pub const SubstraitType_Metaint:i32=15; 
		pub const SubstraitType_Metaenum:i32=16; 
		pub const SubstraitType_Metastr:i32=17; 
		pub const SubstraitType_Typename:i32=18; 
		pub const SubstraitType_Period:i32=19; 
		pub const SubstraitType_Comma:i32=20; 
		pub const SubstraitType_Colon:i32=21; 
		pub const SubstraitType_Semicolon:i32=22; 
		pub const SubstraitType_Question:i32=23; 
		pub const SubstraitType_Bang:i32=24; 
		pub const SubstraitType_OpenParen:i32=25; 
		pub const SubstraitType_CloseParen:i32=26; 
		pub const SubstraitType_OpenCurly:i32=27; 
		pub const SubstraitType_CloseCurly:i32=28; 
		pub const SubstraitType_OpenSquare:i32=29; 
		pub const SubstraitType_CloseSquare:i32=30; 
		pub const SubstraitType_Assign:i32=31; 
		pub const SubstraitType_BooleanOr:i32=32; 
		pub const SubstraitType_BooleanAnd:i32=33; 
		pub const SubstraitType_Equal:i32=34; 
		pub const SubstraitType_NotEqual:i32=35; 
		pub const SubstraitType_LessThan:i32=36; 
		pub const SubstraitType_LessEqual:i32=37; 
		pub const SubstraitType_GreaterThan:i32=38; 
		pub const SubstraitType_GreaterEqual:i32=39; 
		pub const SubstraitType_Plus:i32=40; 
		pub const SubstraitType_Minus:i32=41; 
		pub const SubstraitType_Multiply:i32=42; 
		pub const SubstraitType_Divide:i32=43; 
		pub const SubstraitType_Range:i32=44; 
		pub const SubstraitType_Nonzero:i32=45; 
		pub const SubstraitType_Zero:i32=46; 
		pub const SubstraitType_String:i32=47; 
		pub const SubstraitType_Identifier:i32=48;
	pub const SubstraitType_EOF:i32=EOF;
	pub const RULE_startPattern:usize = 0; 
	pub const RULE_startProgram:usize = 1; 
	pub const RULE_program:usize = 2; 
	pub const RULE_statementSeparator:usize = 3; 
	pub const RULE_statement:usize = 4; 
	pub const RULE_pattern:usize = 5; 
	pub const RULE_patternInvalidIfThenElse:usize = 6; 
	pub const RULE_patternOr:usize = 7; 
	pub const RULE_operatorOr:usize = 8; 
	pub const RULE_patternAnd:usize = 9; 
	pub const RULE_operatorAnd:usize = 10; 
	pub const RULE_patternEqNeq:usize = 11; 
	pub const RULE_operatorEqNeq:usize = 12; 
	pub const RULE_patternIneq:usize = 13; 
	pub const RULE_operatorIneq:usize = 14; 
	pub const RULE_patternAddSub:usize = 15; 
	pub const RULE_operatorAddSub:usize = 16; 
	pub const RULE_patternMulDiv:usize = 17; 
	pub const RULE_operatorMulDiv:usize = 18; 
	pub const RULE_patternMisc:usize = 19; 
	pub const RULE_nullability:usize = 20; 
	pub const RULE_variation:usize = 21; 
	pub const RULE_variationBody:usize = 22; 
	pub const RULE_parameters:usize = 23; 
	pub const RULE_parameter:usize = 24; 
	pub const RULE_parameterValue:usize = 25; 
	pub const RULE_integer:usize = 26; 
	pub const RULE_identifierPath:usize = 27; 
	pub const RULE_identifierOrString:usize = 28;
	pub const ruleNames: [&'static str; 29] =  [
		"startPattern", "startProgram", "program", "statementSeparator", "statement", 
		"pattern", "patternInvalidIfThenElse", "patternOr", "operatorOr", "patternAnd", 
		"operatorAnd", "patternEqNeq", "operatorEqNeq", "patternIneq", "operatorIneq", 
		"patternAddSub", "operatorAddSub", "patternMulDiv", "operatorMulDiv", 
		"patternMisc", "nullability", "variation", "variationBody", "parameters", 
		"parameter", "parameterValue", "integer", "identifierPath", "identifierOrString"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;47] = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, Some("'.'"), Some("','"), Some("':'"), 
		Some("';'"), Some("'?'"), Some("'!'"), Some("'('"), Some("')'"), Some("'{'"), 
		Some("'}'"), Some("'['"), Some("']'"), Some("'='"), Some("'||'"), Some("'&&'"), 
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
		Some("Bang"), Some("OpenParen"), Some("CloseParen"), Some("OpenCurly"), 
		Some("CloseCurly"), Some("OpenSquare"), Some("CloseSquare"), Some("Assign"), 
		Some("BooleanOr"), Some("BooleanAnd"), Some("Equal"), Some("NotEqual"), 
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
pub struct SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SubstraitTypeParser
pub trait SubstraitTypeParserContext<'input>:
	for<'x> Listenable<dyn SubstraitTypeListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=SubstraitTypeParserContextType>
{}

antlr4rust::coerce_from!{ 'input : SubstraitTypeParserContext<'input> }

impl<'input> SubstraitTypeParserContext<'input> for TerminalNode<'input,SubstraitTypeParserContextType> {}
impl<'input> SubstraitTypeParserContext<'input> for ErrorNode<'input,SubstraitTypeParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn SubstraitTypeParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn SubstraitTypeListener<'input> + 'input }

pub struct SubstraitTypeParserContextType;
antlr4rust::tid!{SubstraitTypeParserContextType}

impl<'input> ParserNodeType<'input> for SubstraitTypeParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn SubstraitTypeParserContext<'input> + 'input;
}

impl<'input, I> Deref for SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
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
antlr4rust::tid! { SubstraitTypeParserExt<'a> }

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_startPattern(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_startPattern(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StartPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_startPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_startPattern }
}
antlr4rust::tid!{StartPatternContextExt<'a>}

impl<'input> StartPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StartPatternContextAll<'input>> {
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
	self.get_token(SubstraitType_EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token Whitespace in current rule
fn Whitespace_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Whitespace, starting from 0.
/// Returns `None` if number of children corresponding to token Whitespace is less or equal than `i`.
fn Whitespace(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Whitespace, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Newline in current rule
fn Newline_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Newline, starting from 0.
/// Returns `None` if number of children corresponding to token Newline is less or equal than `i`.
fn Newline(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Newline, i)
}

}

impl<'input> StartPatternContextAttrs<'input> for StartPatternContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn startPattern(&mut self,)
	-> Result<Rc<StartPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StartPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_startPattern);
        let mut _localctx: Rc<StartPatternContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(61);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SubstraitType_Whitespace {
				{
				{
				recog.base.set_state(58);
				recog.base.match_token(SubstraitType_Whitespace,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(63);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(67);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SubstraitType_Newline {
				{
				{
				recog.base.set_state(64);
				recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(69);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule pattern*/
			recog.base.set_state(70);
			recog.pattern()?;

			recog.base.set_state(74);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SubstraitType_Newline {
				{
				{
				recog.base.set_state(71);
				recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(76);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(77);
			recog.base.match_token(SubstraitType_EOF,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_startProgram(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_startProgram(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StartProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_startProgram }
	//fn type_rule_index() -> usize where Self: Sized { RULE_startProgram }
}
antlr4rust::tid!{StartProgramContextExt<'a>}

impl<'input> StartProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StartProgramContextAll<'input>> {
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
	self.get_token(SubstraitType_EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token Whitespace in current rule
fn Whitespace_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Whitespace, starting from 0.
/// Returns `None` if number of children corresponding to token Whitespace is less or equal than `i`.
fn Whitespace(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Whitespace, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Newline in current rule
fn Newline_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Newline, starting from 0.
/// Returns `None` if number of children corresponding to token Newline is less or equal than `i`.
fn Newline(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Newline, i)
}

}

impl<'input> StartProgramContextAttrs<'input> for StartProgramContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn startProgram(&mut self,)
	-> Result<Rc<StartProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StartProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_startProgram);
        let mut _localctx: Rc<StartProgramContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(82);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SubstraitType_Whitespace {
				{
				{
				recog.base.set_state(79);
				recog.base.match_token(SubstraitType_Whitespace,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(84);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(88);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SubstraitType_Newline {
				{
				{
				recog.base.set_state(85);
				recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(90);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule program*/
			recog.base.set_state(91);
			recog.program()?;

			recog.base.set_state(95);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SubstraitType_Newline {
				{
				{
				recog.base.set_state(92);
				recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(97);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(98);
			recog.base.match_token(SubstraitType_EOF,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_program(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_program(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr4rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ProgramContextAll<'input>> {
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(105);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule statement*/
					recog.base.set_state(100);
					recog.statement()?;

					/*InvokeRule statementSeparator*/
					recog.base.set_state(101);
					recog.statementSeparator()?;

					}
					} 
				}
				recog.base.set_state(107);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			}
			/*InvokeRule pattern*/
			recog.base.set_state(108);
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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_statementSeparator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_statementSeparator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StatementSeparatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statementSeparator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statementSeparator }
}
antlr4rust::tid!{StatementSeparatorContextExt<'a>}

impl<'input> StatementSeparatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StatementSeparatorContextAll<'input>> {
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
	self.get_token(SubstraitType_Newline, i)
}
/// Retrieves first TerminalNode corresponding to token Semicolon
/// Returns `None` if there is no child corresponding to token Semicolon
fn Semicolon(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Semicolon, 0)
}

}

impl<'input> StatementSeparatorContextAttrs<'input> for StatementSeparatorContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn statementSeparator(&mut self,)
	-> Result<Rc<StatementSeparatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementSeparatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_statementSeparator);
        let mut _localctx: Rc<StatementSeparatorContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(113);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(110);
					recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(115);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
			}
			recog.base.set_state(124);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_Newline 
				=> {
					{
					recog.base.set_state(116);
					recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Semicolon 
				=> {
					{
					recog.base.set_state(117);
					recog.base.match_token(SubstraitType_Semicolon,&mut recog.err_handler)?;

					recog.base.set_state(121);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==SubstraitType_Newline {
						{
						{
						recog.base.set_state(118);
						recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(123);
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
		recog.base.exit_rule()?;

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
antlr4rust::tid!{StatementContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for StatementContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StatementContextAll<'input>> {
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
		self.get_token(SubstraitType_Assert, 0)
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

antlr4rust::tid!{AssertContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AssertContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AssertContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Assert(self);
		Ok(())
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
		self.get_token(SubstraitType_Assign, 0)
	}
}

impl<'input> NormalContextAttrs<'input> for NormalContext<'input>{}

pub struct NormalContextExt<'input>{
	base:StatementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NormalContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NormalContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NormalContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Normal(self);
		Ok(())
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
		self.get_token(SubstraitType_Assert, 0)
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
		self.get_token(SubstraitType_Matches, 0)
	}
}

impl<'input> MatchContextAttrs<'input> for MatchContext<'input>{}

pub struct MatchContextExt<'input>{
	base:StatementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MatchContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for MatchContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for MatchContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Match(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(137);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
				1 =>{
					let tmp = NormalContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule pattern*/
					recog.base.set_state(126);
					recog.pattern()?;

					recog.base.set_state(127);
					recog.base.match_token(SubstraitType_Assign,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(128);
					recog.pattern()?;

					}
				}
			,
				2 =>{
					let tmp = MatchContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(130);
					recog.base.match_token(SubstraitType_Assert,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(131);
					recog.pattern()?;

					recog.base.set_state(132);
					recog.base.match_token(SubstraitType_Matches,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(133);
					recog.pattern()?;

					}
				}
			,
				3 =>{
					let tmp = AssertContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(135);
					recog.base.match_token(SubstraitType_Assert,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(136);
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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_pattern(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_pattern(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern }
}
antlr4rust::tid!{PatternContextExt<'a>}

impl<'input> PatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PatternContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternContextExt<'input>>{

fn patternInvalidIfThenElse(&self) -> Option<Rc<PatternInvalidIfThenElseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PatternContextAttrs<'input> for PatternContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn pattern(&mut self,)
	-> Result<Rc<PatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_pattern);
        let mut _localctx: Rc<PatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule patternInvalidIfThenElse*/
			recog.base.set_state(139);
			recog.patternInvalidIfThenElse()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- patternInvalidIfThenElse ----------------
#[derive(Debug)]
pub enum PatternInvalidIfThenElseContextAll<'input>{
	InvalidIfThenElseContext(InvalidIfThenElseContext<'input>),
	ValidPatternContext(ValidPatternContext<'input>),
Error(PatternInvalidIfThenElseContext<'input>)
}
antlr4rust::tid!{PatternInvalidIfThenElseContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for PatternInvalidIfThenElseContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for PatternInvalidIfThenElseContextAll<'input>{}

impl<'input> Deref for PatternInvalidIfThenElseContextAll<'input>{
	type Target = dyn PatternInvalidIfThenElseContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use PatternInvalidIfThenElseContextAll::*;
		match self{
			InvalidIfThenElseContext(inner) => inner,
			ValidPatternContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternInvalidIfThenElseContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type PatternInvalidIfThenElseContext<'input> = BaseParserRuleContext<'input,PatternInvalidIfThenElseContextExt<'input>>;

#[derive(Clone)]
pub struct PatternInvalidIfThenElseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for PatternInvalidIfThenElseContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PatternInvalidIfThenElseContext<'input>{
}

impl<'input> CustomRuleContext<'input> for PatternInvalidIfThenElseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternInvalidIfThenElse }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternInvalidIfThenElse }
}
antlr4rust::tid!{PatternInvalidIfThenElseContextExt<'a>}

impl<'input> PatternInvalidIfThenElseContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PatternInvalidIfThenElseContextAll<'input>> {
		Rc::new(
		PatternInvalidIfThenElseContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternInvalidIfThenElseContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait PatternInvalidIfThenElseContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<PatternInvalidIfThenElseContextExt<'input>>{


}

impl<'input> PatternInvalidIfThenElseContextAttrs<'input> for PatternInvalidIfThenElseContext<'input>{}

pub type InvalidIfThenElseContext<'input> = BaseParserRuleContext<'input,InvalidIfThenElseContextExt<'input>>;

pub trait InvalidIfThenElseContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn patternOr_all(&self) ->  Vec<Rc<PatternOrContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn patternOr(&self, i: usize) -> Option<Rc<PatternOrContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Question
	/// Returns `None` if there is no child corresponding to token Question
	fn Question(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Question, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Colon
	/// Returns `None` if there is no child corresponding to token Colon
	fn Colon(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Colon, 0)
	}
	fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> InvalidIfThenElseContextAttrs<'input> for InvalidIfThenElseContext<'input>{}

pub struct InvalidIfThenElseContextExt<'input>{
	base:PatternInvalidIfThenElseContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{InvalidIfThenElseContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for InvalidIfThenElseContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for InvalidIfThenElseContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_InvalidIfThenElse(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for InvalidIfThenElseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternInvalidIfThenElse }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternInvalidIfThenElse }
}

impl<'input> Borrow<PatternInvalidIfThenElseContextExt<'input>> for InvalidIfThenElseContext<'input>{
	fn borrow(&self) -> &PatternInvalidIfThenElseContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternInvalidIfThenElseContextExt<'input>> for InvalidIfThenElseContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternInvalidIfThenElseContextExt<'input> { &mut self.base }
}

impl<'input> PatternInvalidIfThenElseContextAttrs<'input> for InvalidIfThenElseContext<'input> {}

impl<'input> InvalidIfThenElseContextExt<'input>{
	fn new(ctx: &dyn PatternInvalidIfThenElseContextAttrs<'input>) -> Rc<PatternInvalidIfThenElseContextAll<'input>>  {
		Rc::new(
			PatternInvalidIfThenElseContextAll::InvalidIfThenElseContext(
				BaseParserRuleContext::copy_from(ctx,InvalidIfThenElseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ValidPatternContext<'input> = BaseParserRuleContext<'input,ValidPatternContextExt<'input>>;

pub trait ValidPatternContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn patternOr(&self) -> Option<Rc<PatternOrContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ValidPatternContextAttrs<'input> for ValidPatternContext<'input>{}

pub struct ValidPatternContextExt<'input>{
	base:PatternInvalidIfThenElseContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ValidPatternContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for ValidPatternContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ValidPatternContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_ValidPattern(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for ValidPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternInvalidIfThenElse }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternInvalidIfThenElse }
}

impl<'input> Borrow<PatternInvalidIfThenElseContextExt<'input>> for ValidPatternContext<'input>{
	fn borrow(&self) -> &PatternInvalidIfThenElseContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternInvalidIfThenElseContextExt<'input>> for ValidPatternContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternInvalidIfThenElseContextExt<'input> { &mut self.base }
}

impl<'input> PatternInvalidIfThenElseContextAttrs<'input> for ValidPatternContext<'input> {}

impl<'input> ValidPatternContextExt<'input>{
	fn new(ctx: &dyn PatternInvalidIfThenElseContextAttrs<'input>) -> Rc<PatternInvalidIfThenElseContextAll<'input>>  {
		Rc::new(
			PatternInvalidIfThenElseContextAll::ValidPatternContext(
				BaseParserRuleContext::copy_from(ctx,ValidPatternContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn patternInvalidIfThenElse(&mut self,)
	-> Result<Rc<PatternInvalidIfThenElseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternInvalidIfThenElseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_patternInvalidIfThenElse);
        let mut _localctx: Rc<PatternInvalidIfThenElseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(148);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				1 =>{
					let tmp = ValidPatternContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule patternOr*/
					recog.base.set_state(141);
					recog.patternOr()?;

					}
				}
			,
				2 =>{
					let tmp = InvalidIfThenElseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule patternOr*/
					recog.base.set_state(142);
					recog.patternOr()?;

					recog.base.set_state(143);
					recog.base.match_token(SubstraitType_Question,&mut recog.err_handler)?;

					/*InvokeRule patternOr*/
					recog.base.set_state(144);
					recog.patternOr()?;

					recog.base.set_state(145);
					recog.base.match_token(SubstraitType_Colon,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(146);
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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_patternOr(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_patternOr(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PatternOrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternOr }
}
antlr4rust::tid!{PatternOrContextExt<'a>}

impl<'input> PatternOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PatternOrContextAll<'input>> {
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn patternOr(&mut self,)
	-> Result<Rc<PatternOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_patternOr);
        let mut _localctx: Rc<PatternOrContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule patternAnd*/
			recog.base.set_state(150);
			recog.patternAnd()?;

			recog.base.set_state(156);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorOr*/
					recog.base.set_state(151);
					recog.operatorOr()?;

					/*InvokeRule patternAnd*/
					recog.base.set_state(152);
					recog.patternAnd()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- operatorOr ----------------
#[derive(Debug)]
pub enum OperatorOrContextAll<'input>{
	OrContext(OrContext<'input>),
Error(OperatorOrContext<'input>)
}
antlr4rust::tid!{OperatorOrContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for OperatorOrContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{OperatorOrContextExt<'a>}

impl<'input> OperatorOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OperatorOrContextAll<'input>> {
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
		self.get_token(SubstraitType_BooleanOr, 0)
	}
}

impl<'input> OrContextAttrs<'input> for OrContext<'input>{}

pub struct OrContextExt<'input>{
	base:OperatorOrContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{OrContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for OrContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OrContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Or(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn operatorOr(&mut self,)
	-> Result<Rc<OperatorOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_operatorOr);
        let mut _localctx: Rc<OperatorOrContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = OrContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
			_localctx = tmp;
			{
			recog.base.set_state(159);
			recog.base.match_token(SubstraitType_BooleanOr,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_patternAnd(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_patternAnd(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PatternAndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternAnd }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternAnd }
}
antlr4rust::tid!{PatternAndContextExt<'a>}

impl<'input> PatternAndContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PatternAndContextAll<'input>> {
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn patternAnd(&mut self,)
	-> Result<Rc<PatternAndContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternAndContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_patternAnd);
        let mut _localctx: Rc<PatternAndContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule patternEqNeq*/
			recog.base.set_state(161);
			recog.patternEqNeq()?;

			recog.base.set_state(167);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorAnd*/
					recog.base.set_state(162);
					recog.operatorAnd()?;

					/*InvokeRule patternEqNeq*/
					recog.base.set_state(163);
					recog.patternEqNeq()?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- operatorAnd ----------------
#[derive(Debug)]
pub enum OperatorAndContextAll<'input>{
	AndContext(AndContext<'input>),
Error(OperatorAndContext<'input>)
}
antlr4rust::tid!{OperatorAndContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for OperatorAndContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{OperatorAndContextExt<'a>}

impl<'input> OperatorAndContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OperatorAndContextAll<'input>> {
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
		self.get_token(SubstraitType_BooleanAnd, 0)
	}
}

impl<'input> AndContextAttrs<'input> for AndContext<'input>{}

pub struct AndContextExt<'input>{
	base:OperatorAndContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AndContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AndContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AndContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_And(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn operatorAnd(&mut self,)
	-> Result<Rc<OperatorAndContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorAndContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_operatorAnd);
        let mut _localctx: Rc<OperatorAndContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = AndContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
			_localctx = tmp;
			{
			recog.base.set_state(170);
			recog.base.match_token(SubstraitType_BooleanAnd,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_patternEqNeq(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_patternEqNeq(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PatternEqNeqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternEqNeq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternEqNeq }
}
antlr4rust::tid!{PatternEqNeqContextExt<'a>}

impl<'input> PatternEqNeqContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PatternEqNeqContextAll<'input>> {
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn patternEqNeq(&mut self,)
	-> Result<Rc<PatternEqNeqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternEqNeqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_patternEqNeq);
        let mut _localctx: Rc<PatternEqNeqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule patternIneq*/
			recog.base.set_state(172);
			recog.patternIneq()?;

			recog.base.set_state(178);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorEqNeq*/
					recog.base.set_state(173);
					recog.operatorEqNeq()?;

					/*InvokeRule patternIneq*/
					recog.base.set_state(174);
					recog.patternIneq()?;

					}
					} 
				}
				recog.base.set_state(180);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
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
		recog.base.exit_rule()?;

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
antlr4rust::tid!{OperatorEqNeqContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for OperatorEqNeqContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{OperatorEqNeqContextExt<'a>}

impl<'input> OperatorEqNeqContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OperatorEqNeqContextAll<'input>> {
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
		self.get_token(SubstraitType_NotEqual, 0)
	}
}

impl<'input> NeqContextAttrs<'input> for NeqContext<'input>{}

pub struct NeqContextExt<'input>{
	base:OperatorEqNeqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NeqContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NeqContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NeqContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Neq(self);
		Ok(())
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
		self.get_token(SubstraitType_Equal, 0)
	}
}

impl<'input> EqContextAttrs<'input> for EqContext<'input>{}

pub struct EqContextExt<'input>{
	base:OperatorEqNeqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{EqContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for EqContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for EqContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Eq(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn operatorEqNeq(&mut self,)
	-> Result<Rc<OperatorEqNeqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorEqNeqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_operatorEqNeq);
        let mut _localctx: Rc<OperatorEqNeqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(183);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_Equal 
				=> {
					let tmp = EqContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(181);
					recog.base.match_token(SubstraitType_Equal,&mut recog.err_handler)?;

					}
				}

			SubstraitType_NotEqual 
				=> {
					let tmp = NeqContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(182);
					recog.base.match_token(SubstraitType_NotEqual,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_patternIneq(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_patternIneq(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PatternIneqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternIneq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternIneq }
}
antlr4rust::tid!{PatternIneqContextExt<'a>}

impl<'input> PatternIneqContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PatternIneqContextAll<'input>> {
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn patternIneq(&mut self,)
	-> Result<Rc<PatternIneqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternIneqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_patternIneq);
        let mut _localctx: Rc<PatternIneqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule patternAddSub*/
			recog.base.set_state(185);
			recog.patternAddSub()?;

			recog.base.set_state(191);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorIneq*/
					recog.base.set_state(186);
					recog.operatorIneq()?;

					/*InvokeRule patternAddSub*/
					recog.base.set_state(187);
					recog.patternAddSub()?;

					}
					} 
				}
				recog.base.set_state(193);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
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
		recog.base.exit_rule()?;

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
antlr4rust::tid!{OperatorIneqContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for OperatorIneqContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{OperatorIneqContextExt<'a>}

impl<'input> OperatorIneqContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OperatorIneqContextAll<'input>> {
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
		self.get_token(SubstraitType_LessThan, 0)
	}
}

impl<'input> LtContextAttrs<'input> for LtContext<'input>{}

pub struct LtContextExt<'input>{
	base:OperatorIneqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{LtContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for LtContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for LtContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Lt(self);
		Ok(())
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
		self.get_token(SubstraitType_LessEqual, 0)
	}
}

impl<'input> LeContextAttrs<'input> for LeContext<'input>{}

pub struct LeContextExt<'input>{
	base:OperatorIneqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{LeContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for LeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for LeContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Le(self);
		Ok(())
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
		self.get_token(SubstraitType_GreaterThan, 0)
	}
}

impl<'input> GtContextAttrs<'input> for GtContext<'input>{}

pub struct GtContextExt<'input>{
	base:OperatorIneqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{GtContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for GtContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for GtContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Gt(self);
		Ok(())
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
		self.get_token(SubstraitType_GreaterEqual, 0)
	}
}

impl<'input> GeContextAttrs<'input> for GeContext<'input>{}

pub struct GeContextExt<'input>{
	base:OperatorIneqContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{GeContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for GeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for GeContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Ge(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn operatorIneq(&mut self,)
	-> Result<Rc<OperatorIneqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorIneqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_operatorIneq);
        let mut _localctx: Rc<OperatorIneqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(198);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_LessThan 
				=> {
					let tmp = LtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(194);
					recog.base.match_token(SubstraitType_LessThan,&mut recog.err_handler)?;

					}
				}

			SubstraitType_LessEqual 
				=> {
					let tmp = LeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(195);
					recog.base.match_token(SubstraitType_LessEqual,&mut recog.err_handler)?;

					}
				}

			SubstraitType_GreaterThan 
				=> {
					let tmp = GtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(196);
					recog.base.match_token(SubstraitType_GreaterThan,&mut recog.err_handler)?;

					}
				}

			SubstraitType_GreaterEqual 
				=> {
					let tmp = GeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					recog.base.set_state(197);
					recog.base.match_token(SubstraitType_GreaterEqual,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_patternAddSub(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_patternAddSub(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PatternAddSubContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternAddSub }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternAddSub }
}
antlr4rust::tid!{PatternAddSubContextExt<'a>}

impl<'input> PatternAddSubContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PatternAddSubContextAll<'input>> {
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn patternAddSub(&mut self,)
	-> Result<Rc<PatternAddSubContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternAddSubContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_patternAddSub);
        let mut _localctx: Rc<PatternAddSubContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule patternMulDiv*/
			recog.base.set_state(200);
			recog.patternMulDiv()?;

			recog.base.set_state(206);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorAddSub*/
					recog.base.set_state(201);
					recog.operatorAddSub()?;

					/*InvokeRule patternMulDiv*/
					recog.base.set_state(202);
					recog.patternMulDiv()?;

					}
					} 
				}
				recog.base.set_state(208);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
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
		recog.base.exit_rule()?;

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
antlr4rust::tid!{OperatorAddSubContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for OperatorAddSubContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{OperatorAddSubContextExt<'a>}

impl<'input> OperatorAddSubContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OperatorAddSubContextAll<'input>> {
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
		self.get_token(SubstraitType_Plus, 0)
	}
}

impl<'input> AddContextAttrs<'input> for AddContext<'input>{}

pub struct AddContextExt<'input>{
	base:OperatorAddSubContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AddContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AddContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AddContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Add(self);
		Ok(())
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
		self.get_token(SubstraitType_Minus, 0)
	}
}

impl<'input> SubContextAttrs<'input> for SubContext<'input>{}

pub struct SubContextExt<'input>{
	base:OperatorAddSubContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{SubContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for SubContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for SubContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Sub(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn operatorAddSub(&mut self,)
	-> Result<Rc<OperatorAddSubContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorAddSubContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_operatorAddSub);
        let mut _localctx: Rc<OperatorAddSubContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(211);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_Plus 
				=> {
					let tmp = AddContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(209);
					recog.base.match_token(SubstraitType_Plus,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Minus 
				=> {
					let tmp = SubContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(210);
					recog.base.match_token(SubstraitType_Minus,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_patternMulDiv(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_patternMulDiv(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PatternMulDivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMulDiv }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMulDiv }
}
antlr4rust::tid!{PatternMulDivContextExt<'a>}

impl<'input> PatternMulDivContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PatternMulDivContextAll<'input>> {
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn patternMulDiv(&mut self,)
	-> Result<Rc<PatternMulDivContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternMulDivContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_patternMulDiv);
        let mut _localctx: Rc<PatternMulDivContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule patternMisc*/
			recog.base.set_state(213);
			recog.patternMisc()?;

			recog.base.set_state(219);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(20,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule operatorMulDiv*/
					recog.base.set_state(214);
					recog.operatorMulDiv()?;

					/*InvokeRule patternMisc*/
					recog.base.set_state(215);
					recog.patternMisc()?;

					}
					} 
				}
				recog.base.set_state(221);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(20,&mut recog.base)?;
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
		recog.base.exit_rule()?;

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
antlr4rust::tid!{OperatorMulDivContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for OperatorMulDivContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{OperatorMulDivContextExt<'a>}

impl<'input> OperatorMulDivContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OperatorMulDivContextAll<'input>> {
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
		self.get_token(SubstraitType_Divide, 0)
	}
}

impl<'input> DivContextAttrs<'input> for DivContext<'input>{}

pub struct DivContextExt<'input>{
	base:OperatorMulDivContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{DivContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for DivContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for DivContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Div(self);
		Ok(())
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
		self.get_token(SubstraitType_Multiply, 0)
	}
}

impl<'input> MulContextAttrs<'input> for MulContext<'input>{}

pub struct MulContextExt<'input>{
	base:OperatorMulDivContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MulContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for MulContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for MulContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Mul(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn operatorMulDiv(&mut self,)
	-> Result<Rc<OperatorMulDivContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OperatorMulDivContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_operatorMulDiv);
        let mut _localctx: Rc<OperatorMulDivContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(224);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_Multiply 
				=> {
					let tmp = MulContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(222);
					recog.base.match_token(SubstraitType_Multiply,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Divide 
				=> {
					let tmp = DivContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(223);
					recog.base.match_token(SubstraitType_Divide,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
	InconsistentContext(InconsistentContext<'input>),
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
antlr4rust::tid!{PatternMiscContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for PatternMiscContextAll<'input>{}

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
			InconsistentContext(inner) => inner,
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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{PatternMiscContextExt<'a>}

impl<'input> PatternMiscContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PatternMiscContextAll<'input>> {
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
		self.get_token(SubstraitType_OpenParen, 0)
	}
	fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token CloseParen
	/// Returns `None` if there is no child corresponding to token CloseParen
	fn CloseParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_CloseParen, 0)
	}
}

impl<'input> ParenthesesContextAttrs<'input> for ParenthesesContext<'input>{}

pub struct ParenthesesContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ParenthesesContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for ParenthesesContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParenthesesContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_parentheses(self);
		Ok(())
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
		self.get_token(SubstraitType_Range, 0)
	}
}

impl<'input> IntRangeContextAttrs<'input> for IntRangeContext<'input>{}

pub struct IntRangeContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IntRangeContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntRangeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntRangeContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_intRange(self);
		Ok(())
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
		self.get_token(SubstraitType_Minus, 0)
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

antlr4rust::tid!{UnaryNegateContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for UnaryNegateContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for UnaryNegateContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_unaryNegate(self);
		Ok(())
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
		self.get_token(SubstraitType_String, 0)
	}
}

impl<'input> StrExactlyContextAttrs<'input> for StrExactlyContext<'input>{}

pub struct StrExactlyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StrExactlyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for StrExactlyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StrExactlyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_strExactly(self);
		Ok(())
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
		self.get_token(SubstraitType_If, 0)
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
		self.get_token(SubstraitType_Then, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Else
	/// Returns `None` if there is no child corresponding to token Else
	fn Else(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Else, 0)
	}
}

impl<'input> IfThenElseContextAttrs<'input> for IfThenElseContext<'input>{}

pub struct IfThenElseContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IfThenElseContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IfThenElseContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IfThenElseContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_ifThenElse(self);
		Ok(())
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
		self.get_token(SubstraitType_False, 0)
	}
}

impl<'input> BoolFalseContextAttrs<'input> for BoolFalseContext<'input>{}

pub struct BoolFalseContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{BoolFalseContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for BoolFalseContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for BoolFalseContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_boolFalse(self);
		Ok(())
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
		self.get_token(SubstraitType_Metaenum, 0)
	}
}

impl<'input> EnumAnyContextAttrs<'input> for EnumAnyContext<'input>{}

pub struct EnumAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{EnumAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for EnumAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for EnumAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_enumAny(self);
		Ok(())
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
		self.get_token(SubstraitType_Typename, 0)
	}
	fn nullability(&self) -> Option<Rc<NullabilityContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DtAnyContextAttrs<'input> for DtAnyContext<'input>{}

pub struct DtAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{DtAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for DtAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for DtAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_dtAny(self);
		Ok(())
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
		self.get_token(SubstraitType_Question, 0)
	}
}

impl<'input> AnyContextAttrs<'input> for AnyContext<'input>{}

pub struct AnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_any(self);
		Ok(())
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
		self.get_token(SubstraitType_Metaint, 0)
	}
}

impl<'input> IntAnyContextAttrs<'input> for IntAnyContext<'input>{}

pub struct IntAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IntAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_intAny(self);
		Ok(())
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

pub type InconsistentContext<'input> = BaseParserRuleContext<'input,InconsistentContextExt<'input>>;

pub trait InconsistentContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Question
	/// Returns `None` if there is no child corresponding to token Question
	fn Question(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Question, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Identifier
	/// Returns `None` if there is no child corresponding to token Identifier
	fn Identifier(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Identifier, 0)
	}
	fn nullability(&self) -> Option<Rc<NullabilityContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> InconsistentContextAttrs<'input> for InconsistentContext<'input>{}

pub struct InconsistentContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{InconsistentContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for InconsistentContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for InconsistentContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_inconsistent(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for InconsistentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternMisc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternMisc }
}

impl<'input> Borrow<PatternMiscContextExt<'input>> for InconsistentContext<'input>{
	fn borrow(&self) -> &PatternMiscContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PatternMiscContextExt<'input>> for InconsistentContext<'input>{
	fn borrow_mut(&mut self) -> &mut PatternMiscContextExt<'input> { &mut self.base }
}

impl<'input> PatternMiscContextAttrs<'input> for InconsistentContext<'input> {}

impl<'input> InconsistentContextExt<'input>{
	fn new(ctx: &dyn PatternMiscContextAttrs<'input>) -> Rc<PatternMiscContextAll<'input>>  {
		Rc::new(
			PatternMiscContextAll::InconsistentContext(
				BaseParserRuleContext::copy_from(ctx,InconsistentContextExt{
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

antlr4rust::tid!{DatatypeBindingOrConstantContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for DatatypeBindingOrConstantContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for DatatypeBindingOrConstantContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_datatypeBindingOrConstant(self);
		Ok(())
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
		self.get_token(SubstraitType_OpenCurly, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
	fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
	/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
	fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Identifier, i)
	}
	/// Retrieves first TerminalNode corresponding to token CloseCurly
	/// Returns `None` if there is no child corresponding to token CloseCurly
	fn CloseCurly(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_CloseCurly, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Comma, i)
	}
}

impl<'input> EnumSetContextAttrs<'input> for EnumSetContext<'input>{}

pub struct EnumSetContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{EnumSetContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for EnumSetContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for EnumSetContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_enumSet(self);
		Ok(())
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
		self.get_token(SubstraitType_Metastr, 0)
	}
}

impl<'input> StrAnyContextAttrs<'input> for StrAnyContext<'input>{}

pub struct StrAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StrAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for StrAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StrAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_strAny(self);
		Ok(())
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
		self.get_token(SubstraitType_True, 0)
	}
}

impl<'input> BoolTrueContextAttrs<'input> for BoolTrueContext<'input>{}

pub struct BoolTrueContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{BoolTrueContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for BoolTrueContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for BoolTrueContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_boolTrue(self);
		Ok(())
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
		self.get_token(SubstraitType_Range, 0)
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

antlr4rust::tid!{IntAtMostContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntAtMostContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntAtMostContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_intAtMost(self);
		Ok(())
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
		self.get_token(SubstraitType_Range, 0)
	}
}

impl<'input> IntAtLeastContextAttrs<'input> for IntAtLeastContext<'input>{}

pub struct IntAtLeastContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IntAtLeastContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntAtLeastContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntAtLeastContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_intAtLeast(self);
		Ok(())
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

antlr4rust::tid!{IntExactlyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntExactlyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntExactlyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_intExactly(self);
		Ok(())
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
		self.get_token(SubstraitType_Identifier, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OpenParen
	/// Returns `None` if there is no child corresponding to token OpenParen
	fn OpenParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_OpenParen, 0)
	}
	/// Retrieves first TerminalNode corresponding to token CloseParen
	/// Returns `None` if there is no child corresponding to token CloseParen
	fn CloseParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_CloseParen, 0)
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
		self.get_token(SubstraitType_Comma, i)
	}
}

impl<'input> FunctionContextAttrs<'input> for FunctionContext<'input>{}

pub struct FunctionContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{FunctionContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for FunctionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for FunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_function(self);
		Ok(())
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
		self.get_token(SubstraitType_Metabool, 0)
	}
}

impl<'input> BoolAnyContextAttrs<'input> for BoolAnyContext<'input>{}

pub struct BoolAnyContextExt<'input>{
	base:PatternMiscContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{BoolAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for BoolAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for BoolAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_boolAny(self);
		Ok(())
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
	/// Retrieves first TerminalNode corresponding to token Bang
	/// Returns `None` if there is no child corresponding to token Bang
	fn Bang(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Bang, 0)
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

antlr4rust::tid!{UnaryNotContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for UnaryNotContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for UnaryNotContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_unaryNot(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn patternMisc(&mut self,)
	-> Result<Rc<PatternMiscContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternMiscContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_patternMisc);
        let mut _localctx: Rc<PatternMiscContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(301);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(30,&mut recog.base)? {
				1 =>{
					let tmp = ParenthesesContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(226);
					recog.base.match_token(SubstraitType_OpenParen,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(227);
					recog.pattern()?;

					recog.base.set_state(228);
					recog.base.match_token(SubstraitType_CloseParen,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = IfThenElseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(230);
					recog.base.match_token(SubstraitType_If,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(231);
					recog.pattern()?;

					recog.base.set_state(232);
					recog.base.match_token(SubstraitType_Then,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(233);
					recog.pattern()?;

					recog.base.set_state(234);
					recog.base.match_token(SubstraitType_Else,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(235);
					recog.pattern()?;

					}
				}
			,
				3 =>{
					let tmp = UnaryNotContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(237);
					recog.base.match_token(SubstraitType_Bang,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(238);
					recog.pattern()?;

					}
				}
			,
				4 =>{
					let tmp = AnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					recog.base.set_state(239);
					recog.base.match_token(SubstraitType_Question,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					let tmp = BoolAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					recog.base.set_state(240);
					recog.base.match_token(SubstraitType_Metabool,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					let tmp = BoolTrueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6)?;
					_localctx = tmp;
					{
					recog.base.set_state(241);
					recog.base.match_token(SubstraitType_True,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					let tmp = BoolFalseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7)?;
					_localctx = tmp;
					{
					recog.base.set_state(242);
					recog.base.match_token(SubstraitType_False,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					let tmp = IntAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8)?;
					_localctx = tmp;
					{
					recog.base.set_state(243);
					recog.base.match_token(SubstraitType_Metaint,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					let tmp = IntRangeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9)?;
					_localctx = tmp;
					{
					/*InvokeRule integer*/
					recog.base.set_state(244);
					recog.integer()?;

					recog.base.set_state(245);
					recog.base.match_token(SubstraitType_Range,&mut recog.err_handler)?;

					/*InvokeRule integer*/
					recog.base.set_state(246);
					recog.integer()?;

					}
				}
			,
				10 =>{
					let tmp = IntAtLeastContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10)?;
					_localctx = tmp;
					{
					/*InvokeRule integer*/
					recog.base.set_state(248);
					recog.integer()?;

					recog.base.set_state(249);
					recog.base.match_token(SubstraitType_Range,&mut recog.err_handler)?;

					}
				}
			,
				11 =>{
					let tmp = IntAtMostContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11)?;
					_localctx = tmp;
					{
					recog.base.set_state(251);
					recog.base.match_token(SubstraitType_Range,&mut recog.err_handler)?;

					/*InvokeRule integer*/
					recog.base.set_state(252);
					recog.integer()?;

					}
				}
			,
				12 =>{
					let tmp = IntExactlyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12)?;
					_localctx = tmp;
					{
					/*InvokeRule integer*/
					recog.base.set_state(253);
					recog.integer()?;

					}
				}
			,
				13 =>{
					let tmp = EnumAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 13)?;
					_localctx = tmp;
					{
					recog.base.set_state(254);
					recog.base.match_token(SubstraitType_Metaenum,&mut recog.err_handler)?;

					}
				}
			,
				14 =>{
					let tmp = EnumSetContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 14)?;
					_localctx = tmp;
					{
					recog.base.set_state(255);
					recog.base.match_token(SubstraitType_OpenCurly,&mut recog.err_handler)?;

					recog.base.set_state(256);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(261);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==SubstraitType_Comma {
						{
						{
						recog.base.set_state(257);
						recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

						recog.base.set_state(258);
						recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(263);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(264);
					recog.base.match_token(SubstraitType_CloseCurly,&mut recog.err_handler)?;

					}
				}
			,
				15 =>{
					let tmp = StrAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 15)?;
					_localctx = tmp;
					{
					recog.base.set_state(265);
					recog.base.match_token(SubstraitType_Metastr,&mut recog.err_handler)?;

					}
				}
			,
				16 =>{
					let tmp = StrExactlyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 16)?;
					_localctx = tmp;
					{
					recog.base.set_state(266);
					recog.base.match_token(SubstraitType_String,&mut recog.err_handler)?;

					}
				}
			,
				17 =>{
					let tmp = DtAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 17)?;
					_localctx = tmp;
					{
					recog.base.set_state(267);
					recog.base.match_token(SubstraitType_Typename,&mut recog.err_handler)?;

					recog.base.set_state(269);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule nullability*/
							recog.base.set_state(268);
							recog.nullability()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				18 =>{
					let tmp = FunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 18)?;
					_localctx = tmp;
					{
					recog.base.set_state(271);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(272);
					recog.base.match_token(SubstraitType_OpenParen,&mut recog.err_handler)?;

					recog.base.set_state(281);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 193458432) != 0) || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 499) != 0) {
						{
						/*InvokeRule pattern*/
						recog.base.set_state(273);
						recog.pattern()?;

						recog.base.set_state(278);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==SubstraitType_Comma {
							{
							{
							recog.base.set_state(274);
							recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

							/*InvokeRule pattern*/
							recog.base.set_state(275);
							recog.pattern()?;

							}
							}
							recog.base.set_state(280);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(283);
					recog.base.match_token(SubstraitType_CloseParen,&mut recog.err_handler)?;

					}
				}
			,
				19 =>{
					let tmp = DatatypeBindingOrConstantContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 19)?;
					_localctx = tmp;
					{
					/*InvokeRule identifierPath*/
					recog.base.set_state(284);
					recog.identifierPath()?;

					recog.base.set_state(286);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule nullability*/
							recog.base.set_state(285);
							recog.nullability()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(289);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule variation*/
							recog.base.set_state(288);
							recog.variation()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(292);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule parameters*/
							recog.base.set_state(291);
							recog.parameters()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				20 =>{
					let tmp = InconsistentContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 20)?;
					_localctx = tmp;
					{
					recog.base.set_state(294);
					recog.base.match_token(SubstraitType_Question,&mut recog.err_handler)?;

					recog.base.set_state(295);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(297);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(29,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule nullability*/
							recog.base.set_state(296);
							recog.nullability()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				21 =>{
					let tmp = UnaryNegateContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 21)?;
					_localctx = tmp;
					{
					recog.base.set_state(299);
					recog.base.match_token(SubstraitType_Minus,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(300);
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nullability ----------------
#[derive(Debug)]
pub enum NullabilityContextAll<'input>{
	NullableContext(NullableContext<'input>),
	NonNullableContext(NonNullableContext<'input>),
	NullableIfContext(NullableIfContext<'input>),
Error(NullabilityContext<'input>)
}
antlr4rust::tid!{NullabilityContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for NullabilityContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for NullabilityContextAll<'input>{}

impl<'input> Deref for NullabilityContextAll<'input>{
	type Target = dyn NullabilityContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use NullabilityContextAll::*;
		match self{
			NullableContext(inner) => inner,
			NonNullableContext(inner) => inner,
			NullableIfContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NullabilityContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type NullabilityContext<'input> = BaseParserRuleContext<'input,NullabilityContextExt<'input>>;

#[derive(Clone)]
pub struct NullabilityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for NullabilityContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NullabilityContext<'input>{
}

impl<'input> CustomRuleContext<'input> for NullabilityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nullability }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nullability }
}
antlr4rust::tid!{NullabilityContextExt<'a>}

impl<'input> NullabilityContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NullabilityContextAll<'input>> {
		Rc::new(
		NullabilityContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NullabilityContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait NullabilityContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<NullabilityContextExt<'input>>{


}

impl<'input> NullabilityContextAttrs<'input> for NullabilityContext<'input>{}

pub type NullableContext<'input> = BaseParserRuleContext<'input,NullableContextExt<'input>>;

pub trait NullableContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Question
	/// Returns `None` if there is no child corresponding to token Question
	fn Question(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Question, 0)
	}
}

impl<'input> NullableContextAttrs<'input> for NullableContext<'input>{}

pub struct NullableContextExt<'input>{
	base:NullabilityContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NullableContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NullableContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NullableContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_nullable(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for NullableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nullability }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nullability }
}

impl<'input> Borrow<NullabilityContextExt<'input>> for NullableContext<'input>{
	fn borrow(&self) -> &NullabilityContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<NullabilityContextExt<'input>> for NullableContext<'input>{
	fn borrow_mut(&mut self) -> &mut NullabilityContextExt<'input> { &mut self.base }
}

impl<'input> NullabilityContextAttrs<'input> for NullableContext<'input> {}

impl<'input> NullableContextExt<'input>{
	fn new(ctx: &dyn NullabilityContextAttrs<'input>) -> Rc<NullabilityContextAll<'input>>  {
		Rc::new(
			NullabilityContextAll::NullableContext(
				BaseParserRuleContext::copy_from(ctx,NullableContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NonNullableContext<'input> = BaseParserRuleContext<'input,NonNullableContextExt<'input>>;

pub trait NonNullableContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Bang
	/// Returns `None` if there is no child corresponding to token Bang
	fn Bang(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Bang, 0)
	}
}

impl<'input> NonNullableContextAttrs<'input> for NonNullableContext<'input>{}

pub struct NonNullableContextExt<'input>{
	base:NullabilityContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NonNullableContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NonNullableContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NonNullableContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_nonNullable(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for NonNullableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nullability }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nullability }
}

impl<'input> Borrow<NullabilityContextExt<'input>> for NonNullableContext<'input>{
	fn borrow(&self) -> &NullabilityContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<NullabilityContextExt<'input>> for NonNullableContext<'input>{
	fn borrow_mut(&mut self) -> &mut NullabilityContextExt<'input> { &mut self.base }
}

impl<'input> NullabilityContextAttrs<'input> for NonNullableContext<'input> {}

impl<'input> NonNullableContextExt<'input>{
	fn new(ctx: &dyn NullabilityContextAttrs<'input>) -> Rc<NullabilityContextAll<'input>>  {
		Rc::new(
			NullabilityContextAll::NonNullableContext(
				BaseParserRuleContext::copy_from(ctx,NonNullableContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NullableIfContext<'input> = BaseParserRuleContext<'input,NullableIfContextExt<'input>>;

pub trait NullableIfContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Question
	/// Returns `None` if there is no child corresponding to token Question
	fn Question(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Question, 0)
	}
	fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> NullableIfContextAttrs<'input> for NullableIfContext<'input>{}

pub struct NullableIfContextExt<'input>{
	base:NullabilityContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NullableIfContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NullableIfContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NullableIfContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_nullableIf(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for NullableIfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nullability }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nullability }
}

impl<'input> Borrow<NullabilityContextExt<'input>> for NullableIfContext<'input>{
	fn borrow(&self) -> &NullabilityContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<NullabilityContextExt<'input>> for NullableIfContext<'input>{
	fn borrow_mut(&mut self) -> &mut NullabilityContextExt<'input> { &mut self.base }
}

impl<'input> NullabilityContextAttrs<'input> for NullableIfContext<'input> {}

impl<'input> NullableIfContextExt<'input>{
	fn new(ctx: &dyn NullabilityContextAttrs<'input>) -> Rc<NullabilityContextAll<'input>>  {
		Rc::new(
			NullabilityContextAll::NullableIfContext(
				BaseParserRuleContext::copy_from(ctx,NullableIfContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nullability(&mut self,)
	-> Result<Rc<NullabilityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NullabilityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_nullability);
        let mut _localctx: Rc<NullabilityContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(307);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				1 =>{
					let tmp = NonNullableContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(303);
					recog.base.match_token(SubstraitType_Bang,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = NullableContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(304);
					recog.base.match_token(SubstraitType_Question,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = NullableIfContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(305);
					recog.base.match_token(SubstraitType_Question,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(306);
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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_variation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_variation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for VariationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variation }
}
antlr4rust::tid!{VariationContextExt<'a>}

impl<'input> VariationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VariationContextAll<'input>> {
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
	self.get_token(SubstraitType_OpenSquare, 0)
}
fn variationBody(&self) -> Option<Rc<VariationBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CloseSquare
/// Returns `None` if there is no child corresponding to token CloseSquare
fn CloseSquare(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_CloseSquare, 0)
}

}

impl<'input> VariationContextAttrs<'input> for VariationContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn variation(&mut self,)
	-> Result<Rc<VariationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_variation);
        let mut _localctx: Rc<VariationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(309);
			recog.base.match_token(SubstraitType_OpenSquare,&mut recog.err_handler)?;

			/*InvokeRule variationBody*/
			recog.base.set_state(310);
			recog.variationBody()?;

			recog.base.set_state(311);
			recog.base.match_token(SubstraitType_CloseSquare,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
antlr4rust::tid!{VariationBodyContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for VariationBodyContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{VariationBodyContextExt<'a>}

impl<'input> VariationBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VariationBodyContextAll<'input>> {
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
		self.get_token(SubstraitType_Question, 0)
	}
}

impl<'input> VarAnyContextAttrs<'input> for VarAnyContext<'input>{}

pub struct VarAnyContextExt<'input>{
	base:VariationBodyContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{VarAnyContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for VarAnyContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VarAnyContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_varAny(self);
		Ok(())
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
		self.get_token(SubstraitType_Zero, 0)
	}
}

impl<'input> VarSystemPreferredContextAttrs<'input> for VarSystemPreferredContext<'input>{}

pub struct VarSystemPreferredContextExt<'input>{
	base:VariationBodyContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{VarSystemPreferredContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for VarSystemPreferredContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VarSystemPreferredContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_varSystemPreferred(self);
		Ok(())
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

antlr4rust::tid!{VarUserDefinedContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for VarUserDefinedContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VarUserDefinedContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_varUserDefined(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn variationBody(&mut self,)
	-> Result<Rc<VariationBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariationBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_variationBody);
        let mut _localctx: Rc<VariationBodyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(316);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_Question 
				=> {
					let tmp = VarAnyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(313);
					recog.base.match_token(SubstraitType_Question,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Zero 
				=> {
					let tmp = VarSystemPreferredContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(314);
					recog.base.match_token(SubstraitType_Zero,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Identifier 
				=> {
					let tmp = VarUserDefinedContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					/*InvokeRule identifierPath*/
					recog.base.set_state(315);
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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parameters(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parameters(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ParametersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameters }
}
antlr4rust::tid!{ParametersContextExt<'a>}

impl<'input> ParametersContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParametersContextAll<'input>> {
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
	self.get_token(SubstraitType_LessThan, 0)
}
/// Retrieves first TerminalNode corresponding to token GreaterThan
/// Returns `None` if there is no child corresponding to token GreaterThan
fn GreaterThan(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_GreaterThan, 0)
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
	self.get_token(SubstraitType_Comma, i)
}

}

impl<'input> ParametersContextAttrs<'input> for ParametersContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parameters(&mut self,)
	-> Result<Rc<ParametersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_parameters);
        let mut _localctx: Rc<ParametersContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(318);
			recog.base.match_token(SubstraitType_LessThan,&mut recog.err_handler)?;

			recog.base.set_state(327);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 193460480) != 0) || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 499) != 0) {
				{
				/*InvokeRule parameter*/
				recog.base.set_state(319);
				recog.parameter()?;

				recog.base.set_state(324);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==SubstraitType_Comma {
					{
					{
					recog.base.set_state(320);
					recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

					/*InvokeRule parameter*/
					recog.base.set_state(321);
					recog.parameter()?;

					}
					}
					recog.base.set_state(326);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(329);
			recog.base.match_token(SubstraitType_GreaterThan,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parameter(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parameter(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameter }
}
antlr4rust::tid!{ParameterContextExt<'a>}

impl<'input> ParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParameterContextAll<'input>> {
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
	self.get_token(SubstraitType_Colon, 0)
}

}

impl<'input> ParameterContextAttrs<'input> for ParameterContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parameter(&mut self,)
	-> Result<Rc<ParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_parameter);
        let mut _localctx: Rc<ParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(334);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(35,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule identifierOrString*/
					recog.base.set_state(331);
					recog.identifierOrString()?;

					recog.base.set_state(332);
					recog.base.match_token(SubstraitType_Colon,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule parameterValue*/
			recog.base.set_state(336);
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
		recog.base.exit_rule()?;

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
antlr4rust::tid!{ParameterValueContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ParameterValueContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{ParameterValueContextExt<'a>}

impl<'input> ParameterValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParameterValueContextAll<'input>> {
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

antlr4rust::tid!{SpecifiedContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for SpecifiedContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for SpecifiedContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Specified(self);
		Ok(())
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
		self.get_token(SubstraitType_Null, 0)
	}
}

impl<'input> NullContextAttrs<'input> for NullContext<'input>{}

pub struct NullContextExt<'input>{
	base:ParameterValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NullContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NullContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NullContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Null(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parameterValue(&mut self,)
	-> Result<Rc<ParameterValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_parameterValue);
        let mut _localctx: Rc<ParameterValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(340);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_Null 
				=> {
					let tmp = NullContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(338);
					recog.base.match_token(SubstraitType_Null,&mut recog.err_handler)?;

					}
				}

			SubstraitType_If |SubstraitType_True |SubstraitType_False |SubstraitType_Metabool |
			SubstraitType_Metaint |SubstraitType_Metaenum |SubstraitType_Metastr |
			SubstraitType_Typename |SubstraitType_Question |SubstraitType_Bang |SubstraitType_OpenParen |
			SubstraitType_OpenCurly |SubstraitType_Plus |SubstraitType_Minus |SubstraitType_Range |
			SubstraitType_Nonzero |SubstraitType_Zero |SubstraitType_String |SubstraitType_Identifier 
				=> {
					let tmp = SpecifiedContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule pattern*/
					recog.base.set_state(339);
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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_integer(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_integer(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntegerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_integer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_integer }
}
antlr4rust::tid!{IntegerContextExt<'a>}

impl<'input> IntegerContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntegerContextAll<'input>> {
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
	self.get_token(SubstraitType_Zero, 0)
}
/// Retrieves first TerminalNode corresponding to token Nonzero
/// Returns `None` if there is no child corresponding to token Nonzero
fn Nonzero(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Nonzero, 0)
}
/// Retrieves first TerminalNode corresponding to token Plus
/// Returns `None` if there is no child corresponding to token Plus
fn Plus(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Plus, 0)
}
/// Retrieves first TerminalNode corresponding to token Minus
/// Returns `None` if there is no child corresponding to token Minus
fn Minus(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Minus, 0)
}

}

impl<'input> IntegerContextAttrs<'input> for IntegerContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn integer(&mut self,)
	-> Result<Rc<IntegerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntegerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_integer);
        let mut _localctx: Rc<IntegerContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(343);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==SubstraitType_Plus || _la==SubstraitType_Minus {
				{
				recog.base.set_state(342);
				_la = recog.base.input.la(1);
				if { !(_la==SubstraitType_Plus || _la==SubstraitType_Minus) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				}
			}

			recog.base.set_state(345);
			_la = recog.base.input.la(1);
			if { !(_la==SubstraitType_Nonzero || _la==SubstraitType_Zero) } {
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
		recog.base.exit_rule()?;

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
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_identifierPath(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_identifierPath(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IdentifierPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierPath }
}
antlr4rust::tid!{IdentifierPathContextExt<'a>}

impl<'input> IdentifierPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IdentifierPathContextAll<'input>> {
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
	self.get_token(SubstraitType_Identifier, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Period in current rule
fn Period_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Period, starting from 0.
/// Returns `None` if number of children corresponding to token Period is less or equal than `i`.
fn Period(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Period, i)
}

}

impl<'input> IdentifierPathContextAttrs<'input> for IdentifierPathContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn identifierPath(&mut self,)
	-> Result<Rc<IdentifierPathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierPathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_identifierPath);
        let mut _localctx: Rc<IdentifierPathContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(351);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(38,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(347);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(348);
					recog.base.match_token(SubstraitType_Period,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(353);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(38,&mut recog.base)?;
			}
			recog.base.set_state(354);
			recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

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
antlr4rust::tid!{IdentifierOrStringContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for IdentifierOrStringContextAll<'input>{}

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
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
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
antlr4rust::tid!{IdentifierOrStringContextExt<'a>}

impl<'input> IdentifierOrStringContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IdentifierOrStringContextAll<'input>> {
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
		self.get_token(SubstraitType_String, 0)
	}
}

impl<'input> StrContextAttrs<'input> for StrContext<'input>{}

pub struct StrContextExt<'input>{
	base:IdentifierOrStringContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StrContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for StrContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StrContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Str(self);
		Ok(())
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
		self.get_token(SubstraitType_Identifier, 0)
	}
}

impl<'input> IdentContextAttrs<'input> for IdentContext<'input>{}

pub struct IdentContextExt<'input>{
	base:IdentifierOrStringContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IdentContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IdentContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IdentContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Ident(self);
		Ok(())
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn identifierOrString(&mut self,)
	-> Result<Rc<IdentifierOrStringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierOrStringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_identifierOrString);
        let mut _localctx: Rc<IdentifierOrStringContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(358);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_String 
				=> {
					let tmp = StrContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(356);
					recog.base.match_token(SubstraitType_String,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Identifier 
				=> {
					let tmp = IdentContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(357);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 48, 361, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 7, 
		4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 
		7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 
		7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 20, 
		7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 
		7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 1, 0, 5, 0, 60, 8, 0, 
		10, 0, 12, 0, 63, 9, 0, 1, 0, 5, 0, 66, 8, 0, 10, 0, 12, 0, 69, 9, 0, 
		1, 0, 1, 0, 5, 0, 73, 8, 0, 10, 0, 12, 0, 76, 9, 0, 1, 0, 1, 0, 1, 1, 
		5, 1, 81, 8, 1, 10, 1, 12, 1, 84, 9, 1, 1, 1, 5, 1, 87, 8, 1, 10, 1, 12, 
		1, 90, 9, 1, 1, 1, 1, 1, 5, 1, 94, 8, 1, 10, 1, 12, 1, 97, 9, 1, 1, 1, 
		1, 1, 1, 2, 1, 2, 1, 2, 5, 2, 104, 8, 2, 10, 2, 12, 2, 107, 9, 2, 1, 2, 
		1, 2, 1, 3, 5, 3, 112, 8, 3, 10, 3, 12, 3, 115, 9, 3, 1, 3, 1, 3, 1, 3, 
		5, 3, 120, 8, 3, 10, 3, 12, 3, 123, 9, 3, 3, 3, 125, 8, 3, 1, 4, 1, 4, 
		1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 3, 4, 138, 8, 4, 
		1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 3, 6, 149, 8, 6, 
		1, 7, 1, 7, 1, 7, 1, 7, 5, 7, 155, 8, 7, 10, 7, 12, 7, 158, 9, 7, 1, 8, 
		1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 5, 9, 166, 8, 9, 10, 9, 12, 9, 169, 9, 9, 
		1, 10, 1, 10, 1, 11, 1, 11, 1, 11, 1, 11, 5, 11, 177, 8, 11, 10, 11, 12, 
		11, 180, 9, 11, 1, 12, 1, 12, 3, 12, 184, 8, 12, 1, 13, 1, 13, 1, 13, 
		1, 13, 5, 13, 190, 8, 13, 10, 13, 12, 13, 193, 9, 13, 1, 14, 1, 14, 1, 
		14, 1, 14, 3, 14, 199, 8, 14, 1, 15, 1, 15, 1, 15, 1, 15, 5, 15, 205, 
		8, 15, 10, 15, 12, 15, 208, 9, 15, 1, 16, 1, 16, 3, 16, 212, 8, 16, 1, 
		17, 1, 17, 1, 17, 1, 17, 5, 17, 218, 8, 17, 10, 17, 12, 17, 221, 9, 17, 
		1, 18, 1, 18, 3, 18, 225, 8, 18, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 
		19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 
		19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 
		19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 5, 19, 260, 8, 19, 
		10, 19, 12, 19, 263, 9, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 3, 19, 
		270, 8, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 5, 19, 277, 8, 19, 10, 
		19, 12, 19, 280, 9, 19, 3, 19, 282, 8, 19, 1, 19, 1, 19, 1, 19, 3, 19, 
		287, 8, 19, 1, 19, 3, 19, 290, 8, 19, 1, 19, 3, 19, 293, 8, 19, 1, 19, 
		1, 19, 1, 19, 3, 19, 298, 8, 19, 1, 19, 1, 19, 3, 19, 302, 8, 19, 1, 20, 
		1, 20, 1, 20, 1, 20, 3, 20, 308, 8, 20, 1, 21, 1, 21, 1, 21, 1, 21, 1, 
		22, 1, 22, 1, 22, 3, 22, 317, 8, 22, 1, 23, 1, 23, 1, 23, 1, 23, 5, 23, 
		323, 8, 23, 10, 23, 12, 23, 326, 9, 23, 3, 23, 328, 8, 23, 1, 23, 1, 23, 
		1, 24, 1, 24, 1, 24, 3, 24, 335, 8, 24, 1, 24, 1, 24, 1, 25, 1, 25, 3, 
		25, 341, 8, 25, 1, 26, 3, 26, 344, 8, 26, 1, 26, 1, 26, 1, 27, 1, 27, 
		5, 27, 350, 8, 27, 10, 27, 12, 27, 353, 9, 27, 1, 27, 1, 27, 1, 28, 1, 
		28, 3, 28, 359, 8, 28, 1, 28, 0, 0, 29, 0, 2, 4, 6, 8, 10, 12, 14, 16, 
		18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 
		54, 56, 0, 2, 1, 0, 40, 41, 1, 0, 45, 46, 395, 0, 61, 1, 0, 0, 0, 2, 82, 
		1, 0, 0, 0, 4, 105, 1, 0, 0, 0, 6, 113, 1, 0, 0, 0, 8, 137, 1, 0, 0, 0, 
		10, 139, 1, 0, 0, 0, 12, 148, 1, 0, 0, 0, 14, 150, 1, 0, 0, 0, 16, 159, 
		1, 0, 0, 0, 18, 161, 1, 0, 0, 0, 20, 170, 1, 0, 0, 0, 22, 172, 1, 0, 0, 
		0, 24, 183, 1, 0, 0, 0, 26, 185, 1, 0, 0, 0, 28, 198, 1, 0, 0, 0, 30, 
		200, 1, 0, 0, 0, 32, 211, 1, 0, 0, 0, 34, 213, 1, 0, 0, 0, 36, 224, 1, 
		0, 0, 0, 38, 301, 1, 0, 0, 0, 40, 307, 1, 0, 0, 0, 42, 309, 1, 0, 0, 0, 
		44, 316, 1, 0, 0, 0, 46, 318, 1, 0, 0, 0, 48, 334, 1, 0, 0, 0, 50, 340, 
		1, 0, 0, 0, 52, 343, 1, 0, 0, 0, 54, 351, 1, 0, 0, 0, 56, 358, 1, 0, 0, 
		0, 58, 60, 5, 3, 0, 0, 59, 58, 1, 0, 0, 0, 60, 63, 1, 0, 0, 0, 61, 59, 
		1, 0, 0, 0, 61, 62, 1, 0, 0, 0, 62, 67, 1, 0, 0, 0, 63, 61, 1, 0, 0, 0, 
		64, 66, 5, 4, 0, 0, 65, 64, 1, 0, 0, 0, 66, 69, 1, 0, 0, 0, 67, 65, 1, 
		0, 0, 0, 67, 68, 1, 0, 0, 0, 68, 70, 1, 0, 0, 0, 69, 67, 1, 0, 0, 0, 70, 
		74, 3, 10, 5, 0, 71, 73, 5, 4, 0, 0, 72, 71, 1, 0, 0, 0, 73, 76, 1, 0, 
		0, 0, 74, 72, 1, 0, 0, 0, 74, 75, 1, 0, 0, 0, 75, 77, 1, 0, 0, 0, 76, 
		74, 1, 0, 0, 0, 77, 78, 5, 0, 0, 1, 78, 1, 1, 0, 0, 0, 79, 81, 5, 3, 0, 
		0, 80, 79, 1, 0, 0, 0, 81, 84, 1, 0, 0, 0, 82, 80, 1, 0, 0, 0, 82, 83, 
		1, 0, 0, 0, 83, 88, 1, 0, 0, 0, 84, 82, 1, 0, 0, 0, 85, 87, 5, 4, 0, 0, 
		86, 85, 1, 0, 0, 0, 87, 90, 1, 0, 0, 0, 88, 86, 1, 0, 0, 0, 88, 89, 1, 
		0, 0, 0, 89, 91, 1, 0, 0, 0, 90, 88, 1, 0, 0, 0, 91, 95, 3, 4, 2, 0, 92, 
		94, 5, 4, 0, 0, 93, 92, 1, 0, 0, 0, 94, 97, 1, 0, 0, 0, 95, 93, 1, 0, 
		0, 0, 95, 96, 1, 0, 0, 0, 96, 98, 1, 0, 0, 0, 97, 95, 1, 0, 0, 0, 98, 
		99, 5, 0, 0, 1, 99, 3, 1, 0, 0, 0, 100, 101, 3, 8, 4, 0, 101, 102, 3, 
		6, 3, 0, 102, 104, 1, 0, 0, 0, 103, 100, 1, 0, 0, 0, 104, 107, 1, 0, 0, 
		0, 105, 103, 1, 0, 0, 0, 105, 106, 1, 0, 0, 0, 106, 108, 1, 0, 0, 0, 107, 
		105, 1, 0, 0, 0, 108, 109, 3, 10, 5, 0, 109, 5, 1, 0, 0, 0, 110, 112, 
		5, 4, 0, 0, 111, 110, 1, 0, 0, 0, 112, 115, 1, 0, 0, 0, 113, 111, 1, 0, 
		0, 0, 113, 114, 1, 0, 0, 0, 114, 124, 1, 0, 0, 0, 115, 113, 1, 0, 0, 0, 
		116, 125, 5, 4, 0, 0, 117, 121, 5, 22, 0, 0, 118, 120, 5, 4, 0, 0, 119, 
		118, 1, 0, 0, 0, 120, 123, 1, 0, 0, 0, 121, 119, 1, 0, 0, 0, 121, 122, 
		1, 0, 0, 0, 122, 125, 1, 0, 0, 0, 123, 121, 1, 0, 0, 0, 124, 116, 1, 0, 
		0, 0, 124, 117, 1, 0, 0, 0, 125, 7, 1, 0, 0, 0, 126, 127, 3, 10, 5, 0, 
		127, 128, 5, 31, 0, 0, 128, 129, 3, 10, 5, 0, 129, 138, 1, 0, 0, 0, 130, 
		131, 5, 6, 0, 0, 131, 132, 3, 10, 5, 0, 132, 133, 5, 7, 0, 0, 133, 134, 
		3, 10, 5, 0, 134, 138, 1, 0, 0, 0, 135, 136, 5, 6, 0, 0, 136, 138, 3, 
		10, 5, 0, 137, 126, 1, 0, 0, 0, 137, 130, 1, 0, 0, 0, 137, 135, 1, 0, 
		0, 0, 138, 9, 1, 0, 0, 0, 139, 140, 3, 12, 6, 0, 140, 11, 1, 0, 0, 0, 
		141, 149, 3, 14, 7, 0, 142, 143, 3, 14, 7, 0, 143, 144, 5, 23, 0, 0, 144, 
		145, 3, 14, 7, 0, 145, 146, 5, 21, 0, 0, 146, 147, 3, 10, 5, 0, 147, 149, 
		1, 0, 0, 0, 148, 141, 1, 0, 0, 0, 148, 142, 1, 0, 0, 0, 149, 13, 1, 0, 
		0, 0, 150, 156, 3, 18, 9, 0, 151, 152, 3, 16, 8, 0, 152, 153, 3, 18, 9, 
		0, 153, 155, 1, 0, 0, 0, 154, 151, 1, 0, 0, 0, 155, 158, 1, 0, 0, 0, 156, 
		154, 1, 0, 0, 0, 156, 157, 1, 0, 0, 0, 157, 15, 1, 0, 0, 0, 158, 156, 
		1, 0, 0, 0, 159, 160, 5, 32, 0, 0, 160, 17, 1, 0, 0, 0, 161, 167, 3, 22, 
		11, 0, 162, 163, 3, 20, 10, 0, 163, 164, 3, 22, 11, 0, 164, 166, 1, 0, 
		0, 0, 165, 162, 1, 0, 0, 0, 166, 169, 1, 0, 0, 0, 167, 165, 1, 0, 0, 0, 
		167, 168, 1, 0, 0, 0, 168, 19, 1, 0, 0, 0, 169, 167, 1, 0, 0, 0, 170, 
		171, 5, 33, 0, 0, 171, 21, 1, 0, 0, 0, 172, 178, 3, 26, 13, 0, 173, 174, 
		3, 24, 12, 0, 174, 175, 3, 26, 13, 0, 175, 177, 1, 0, 0, 0, 176, 173, 
		1, 0, 0, 0, 177, 180, 1, 0, 0, 0, 178, 176, 1, 0, 0, 0, 178, 179, 1, 0, 
		0, 0, 179, 23, 1, 0, 0, 0, 180, 178, 1, 0, 0, 0, 181, 184, 5, 34, 0, 0, 
		182, 184, 5, 35, 0, 0, 183, 181, 1, 0, 0, 0, 183, 182, 1, 0, 0, 0, 184, 
		25, 1, 0, 0, 0, 185, 191, 3, 30, 15, 0, 186, 187, 3, 28, 14, 0, 187, 188, 
		3, 30, 15, 0, 188, 190, 1, 0, 0, 0, 189, 186, 1, 0, 0, 0, 190, 193, 1, 
		0, 0, 0, 191, 189, 1, 0, 0, 0, 191, 192, 1, 0, 0, 0, 192, 27, 1, 0, 0, 
		0, 193, 191, 1, 0, 0, 0, 194, 199, 5, 36, 0, 0, 195, 199, 5, 37, 0, 0, 
		196, 199, 5, 38, 0, 0, 197, 199, 5, 39, 0, 0, 198, 194, 1, 0, 0, 0, 198, 
		195, 1, 0, 0, 0, 198, 196, 1, 0, 0, 0, 198, 197, 1, 0, 0, 0, 199, 29, 
		1, 0, 0, 0, 200, 206, 3, 34, 17, 0, 201, 202, 3, 32, 16, 0, 202, 203, 
		3, 34, 17, 0, 203, 205, 1, 0, 0, 0, 204, 201, 1, 0, 0, 0, 205, 208, 1, 
		0, 0, 0, 206, 204, 1, 0, 0, 0, 206, 207, 1, 0, 0, 0, 207, 31, 1, 0, 0, 
		0, 208, 206, 1, 0, 0, 0, 209, 212, 5, 40, 0, 0, 210, 212, 5, 41, 0, 0, 
		211, 209, 1, 0, 0, 0, 211, 210, 1, 0, 0, 0, 212, 33, 1, 0, 0, 0, 213, 
		219, 3, 38, 19, 0, 214, 215, 3, 36, 18, 0, 215, 216, 3, 38, 19, 0, 216, 
		218, 1, 0, 0, 0, 217, 214, 1, 0, 0, 0, 218, 221, 1, 0, 0, 0, 219, 217, 
		1, 0, 0, 0, 219, 220, 1, 0, 0, 0, 220, 35, 1, 0, 0, 0, 221, 219, 1, 0, 
		0, 0, 222, 225, 5, 42, 0, 0, 223, 225, 5, 43, 0, 0, 224, 222, 1, 0, 0, 
		0, 224, 223, 1, 0, 0, 0, 225, 37, 1, 0, 0, 0, 226, 227, 5, 25, 0, 0, 227, 
		228, 3, 10, 5, 0, 228, 229, 5, 26, 0, 0, 229, 302, 1, 0, 0, 0, 230, 231, 
		5, 8, 0, 0, 231, 232, 3, 10, 5, 0, 232, 233, 5, 9, 0, 0, 233, 234, 3, 
		10, 5, 0, 234, 235, 5, 10, 0, 0, 235, 236, 3, 10, 5, 0, 236, 302, 1, 0, 
		0, 0, 237, 238, 5, 24, 0, 0, 238, 302, 3, 10, 5, 0, 239, 302, 5, 23, 0, 
		0, 240, 302, 5, 14, 0, 0, 241, 302, 5, 12, 0, 0, 242, 302, 5, 13, 0, 0, 
		243, 302, 5, 15, 0, 0, 244, 245, 3, 52, 26, 0, 245, 246, 5, 44, 0, 0, 
		246, 247, 3, 52, 26, 0, 247, 302, 1, 0, 0, 0, 248, 249, 3, 52, 26, 0, 
		249, 250, 5, 44, 0, 0, 250, 302, 1, 0, 0, 0, 251, 252, 5, 44, 0, 0, 252, 
		302, 3, 52, 26, 0, 253, 302, 3, 52, 26, 0, 254, 302, 5, 16, 0, 0, 255, 
		256, 5, 27, 0, 0, 256, 261, 5, 48, 0, 0, 257, 258, 5, 20, 0, 0, 258, 260, 
		5, 48, 0, 0, 259, 257, 1, 0, 0, 0, 260, 263, 1, 0, 0, 0, 261, 259, 1, 
		0, 0, 0, 261, 262, 1, 0, 0, 0, 262, 264, 1, 0, 0, 0, 263, 261, 1, 0, 0, 
		0, 264, 302, 5, 28, 0, 0, 265, 302, 5, 17, 0, 0, 266, 302, 5, 47, 0, 0, 
		267, 269, 5, 18, 0, 0, 268, 270, 3, 40, 20, 0, 269, 268, 1, 0, 0, 0, 269, 
		270, 1, 0, 0, 0, 270, 302, 1, 0, 0, 0, 271, 272, 5, 48, 0, 0, 272, 281, 
		5, 25, 0, 0, 273, 278, 3, 10, 5, 0, 274, 275, 5, 20, 0, 0, 275, 277, 3, 
		10, 5, 0, 276, 274, 1, 0, 0, 0, 277, 280, 1, 0, 0, 0, 278, 276, 1, 0, 
		0, 0, 278, 279, 1, 0, 0, 0, 279, 282, 1, 0, 0, 0, 280, 278, 1, 0, 0, 0, 
		281, 273, 1, 0, 0, 0, 281, 282, 1, 0, 0, 0, 282, 283, 1, 0, 0, 0, 283, 
		302, 5, 26, 0, 0, 284, 286, 3, 54, 27, 0, 285, 287, 3, 40, 20, 0, 286, 
		285, 1, 0, 0, 0, 286, 287, 1, 0, 0, 0, 287, 289, 1, 0, 0, 0, 288, 290, 
		3, 42, 21, 0, 289, 288, 1, 0, 0, 0, 289, 290, 1, 0, 0, 0, 290, 292, 1, 
		0, 0, 0, 291, 293, 3, 46, 23, 0, 292, 291, 1, 0, 0, 0, 292, 293, 1, 0, 
		0, 0, 293, 302, 1, 0, 0, 0, 294, 295, 5, 23, 0, 0, 295, 297, 5, 48, 0, 
		0, 296, 298, 3, 40, 20, 0, 297, 296, 1, 0, 0, 0, 297, 298, 1, 0, 0, 0, 
		298, 302, 1, 0, 0, 0, 299, 300, 5, 41, 0, 0, 300, 302, 3, 10, 5, 0, 301, 
		226, 1, 0, 0, 0, 301, 230, 1, 0, 0, 0, 301, 237, 1, 0, 0, 0, 301, 239, 
		1, 0, 0, 0, 301, 240, 1, 0, 0, 0, 301, 241, 1, 0, 0, 0, 301, 242, 1, 0, 
		0, 0, 301, 243, 1, 0, 0, 0, 301, 244, 1, 0, 0, 0, 301, 248, 1, 0, 0, 0, 
		301, 251, 1, 0, 0, 0, 301, 253, 1, 0, 0, 0, 301, 254, 1, 0, 0, 0, 301, 
		255, 1, 0, 0, 0, 301, 265, 1, 0, 0, 0, 301, 266, 1, 0, 0, 0, 301, 267, 
		1, 0, 0, 0, 301, 271, 1, 0, 0, 0, 301, 284, 1, 0, 0, 0, 301, 294, 1, 0, 
		0, 0, 301, 299, 1, 0, 0, 0, 302, 39, 1, 0, 0, 0, 303, 308, 5, 24, 0, 0, 
		304, 308, 5, 23, 0, 0, 305, 306, 5, 23, 0, 0, 306, 308, 3, 10, 5, 0, 307, 
		303, 1, 0, 0, 0, 307, 304, 1, 0, 0, 0, 307, 305, 1, 0, 0, 0, 308, 41, 
		1, 0, 0, 0, 309, 310, 5, 29, 0, 0, 310, 311, 3, 44, 22, 0, 311, 312, 5, 
		30, 0, 0, 312, 43, 1, 0, 0, 0, 313, 317, 5, 23, 0, 0, 314, 317, 5, 46, 
		0, 0, 315, 317, 3, 54, 27, 0, 316, 313, 1, 0, 0, 0, 316, 314, 1, 0, 0, 
		0, 316, 315, 1, 0, 0, 0, 317, 45, 1, 0, 0, 0, 318, 327, 5, 36, 0, 0, 319, 
		324, 3, 48, 24, 0, 320, 321, 5, 20, 0, 0, 321, 323, 3, 48, 24, 0, 322, 
		320, 1, 0, 0, 0, 323, 326, 1, 0, 0, 0, 324, 322, 1, 0, 0, 0, 324, 325, 
		1, 0, 0, 0, 325, 328, 1, 0, 0, 0, 326, 324, 1, 0, 0, 0, 327, 319, 1, 0, 
		0, 0, 327, 328, 1, 0, 0, 0, 328, 329, 1, 0, 0, 0, 329, 330, 5, 38, 0, 
		0, 330, 47, 1, 0, 0, 0, 331, 332, 3, 56, 28, 0, 332, 333, 5, 21, 0, 0, 
		333, 335, 1, 0, 0, 0, 334, 331, 1, 0, 0, 0, 334, 335, 1, 0, 0, 0, 335, 
		336, 1, 0, 0, 0, 336, 337, 3, 50, 25, 0, 337, 49, 1, 0, 0, 0, 338, 341, 
		5, 11, 0, 0, 339, 341, 3, 10, 5, 0, 340, 338, 1, 0, 0, 0, 340, 339, 1, 
		0, 0, 0, 341, 51, 1, 0, 0, 0, 342, 344, 7, 0, 0, 0, 343, 342, 1, 0, 0, 
		0, 343, 344, 1, 0, 0, 0, 344, 345, 1, 0, 0, 0, 345, 346, 7, 1, 0, 0, 346, 
		53, 1, 0, 0, 0, 347, 348, 5, 48, 0, 0, 348, 350, 5, 19, 0, 0, 349, 347, 
		1, 0, 0, 0, 350, 353, 1, 0, 0, 0, 351, 349, 1, 0, 0, 0, 351, 352, 1, 0, 
		0, 0, 352, 354, 1, 0, 0, 0, 353, 351, 1, 0, 0, 0, 354, 355, 5, 48, 0, 
		0, 355, 55, 1, 0, 0, 0, 356, 359, 5, 47, 0, 0, 357, 359, 5, 48, 0, 0, 
		358, 356, 1, 0, 0, 0, 358, 357, 1, 0, 0, 0, 359, 57, 1, 0, 0, 0, 40, 61, 
		67, 74, 82, 88, 95, 105, 113, 121, 124, 137, 148, 156, 167, 178, 183, 
		191, 198, 206, 211, 219, 224, 261, 269, 278, 281, 286, 289, 292, 297, 
		301, 307, 316, 324, 327, 334, 340, 343, 351, 358
	];
}
