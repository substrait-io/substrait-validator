// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]
// Generated from SubstraitType.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr4rust::atn::ATN;
use antlr4rust::char_stream::CharStream;
use antlr4rust::int_stream::IntStream;
use antlr4rust::tree::ParseTree;
use antlr4rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr4rust::PredictionContextCache;
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::TokenSource;
use antlr4rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr4rust::token::*;
use antlr4rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr4rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr4rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const LineComment:i32=1; 
	pub const BlockComment:i32=2; 
	pub const Whitespace:i32=3; 
	pub const Newline:i32=4; 
	pub const EscNewline:i32=5; 
	pub const Assert:i32=6; 
	pub const Matches:i32=7; 
	pub const If:i32=8; 
	pub const Then:i32=9; 
	pub const Else:i32=10; 
	pub const Null:i32=11; 
	pub const True:i32=12; 
	pub const False:i32=13; 
	pub const Metabool:i32=14; 
	pub const Metaint:i32=15; 
	pub const Metaenum:i32=16; 
	pub const Metastr:i32=17; 
	pub const Typename:i32=18; 
	pub const Period:i32=19; 
	pub const Comma:i32=20; 
	pub const Colon:i32=21; 
	pub const Semicolon:i32=22; 
	pub const Question:i32=23; 
	pub const Bang:i32=24; 
	pub const OpenParen:i32=25; 
	pub const CloseParen:i32=26; 
	pub const OpenCurly:i32=27; 
	pub const CloseCurly:i32=28; 
	pub const OpenSquare:i32=29; 
	pub const CloseSquare:i32=30; 
	pub const Assign:i32=31; 
	pub const BooleanOr:i32=32; 
	pub const BooleanAnd:i32=33; 
	pub const Equal:i32=34; 
	pub const NotEqual:i32=35; 
	pub const LessThan:i32=36; 
	pub const LessEqual:i32=37; 
	pub const GreaterThan:i32=38; 
	pub const GreaterEqual:i32=39; 
	pub const Plus:i32=40; 
	pub const Minus:i32=41; 
	pub const Multiply:i32=42; 
	pub const Divide:i32=43; 
	pub const Range:i32=44; 
	pub const Nonzero:i32=45; 
	pub const Zero:i32=46; 
	pub const String:i32=47; 
	pub const Identifier:i32=48;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;74] = [
		"LineComment", "BlockComment", "Whitespace", "Newline", "EscNewline", 
		"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", 
		"O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "Assert", 
		"Matches", "If", "Then", "Else", "Null", "True", "False", "Metabool", 
		"Metaint", "Metaenum", "Metastr", "Typename", "Period", "Comma", "Colon", 
		"Semicolon", "Question", "Bang", "OpenParen", "CloseParen", "OpenCurly", 
		"CloseCurly", "OpenSquare", "CloseSquare", "Assign", "BooleanOr", "BooleanAnd", 
		"Equal", "NotEqual", "LessThan", "LessEqual", "GreaterThan", "GreaterEqual", 
		"Plus", "Minus", "Multiply", "Divide", "Range", "Nonzero", "Zero", "String", 
		"Identifier"
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


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct SubstraitTypeLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,SubstraitTypeLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr4rust::tid! { impl<'input,Input> TidAble<'input> for SubstraitTypeLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for SubstraitTypeLexer<'input,Input>{
	type Target = BaseLexer<'input,SubstraitTypeLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for SubstraitTypeLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> SubstraitTypeLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "SubstraitTypeLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr4rust::recognizer::check_version("0","5");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				SubstraitTypeLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> SubstraitTypeLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		SubstraitTypeLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct SubstraitTypeLexerActions {
}

impl SubstraitTypeLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,SubstraitTypeLexerActions,Input,LocalTokenFactory<'input>>> for SubstraitTypeLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> SubstraitTypeLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,SubstraitTypeLexerActions,Input,LocalTokenFactory<'input>>> for SubstraitTypeLexerActions{
}
impl<'input> TokenAware<'input> for SubstraitTypeLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for SubstraitTypeLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
    }

    fn get_dfa_string(&self) -> String {
        self.base.get_dfa_string()
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
			4, 0, 48, 421, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
			2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
			9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 
			14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 
			19, 2, 20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 
			24, 2, 25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 
			29, 2, 30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 
			34, 2, 35, 7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 
			39, 2, 40, 7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 
			44, 2, 45, 7, 45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 48, 2, 49, 7, 
			49, 2, 50, 7, 50, 2, 51, 7, 51, 2, 52, 7, 52, 2, 53, 7, 53, 2, 54, 7, 
			54, 2, 55, 7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 2, 58, 7, 58, 2, 59, 7, 
			59, 2, 60, 7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 63, 7, 63, 2, 64, 7, 
			64, 2, 65, 7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 2, 68, 7, 68, 2, 69, 7, 
			69, 2, 70, 7, 70, 2, 71, 7, 71, 2, 72, 7, 72, 2, 73, 7, 73, 1, 0, 1, 
			0, 1, 0, 1, 0, 5, 0, 154, 8, 0, 10, 0, 12, 0, 157, 9, 0, 1, 0, 1, 0, 
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1, 166, 8, 1, 11, 1, 12, 1, 167, 1, 
			1, 3, 1, 171, 8, 1, 1, 1, 5, 1, 174, 8, 1, 10, 1, 12, 1, 177, 9, 1, 1, 
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 4, 2, 185, 8, 2, 11, 2, 12, 2, 186, 
			1, 2, 1, 2, 1, 3, 4, 3, 192, 8, 3, 11, 3, 12, 3, 193, 1, 4, 1, 4, 4, 
			4, 198, 8, 4, 11, 4, 12, 4, 199, 1, 4, 1, 4, 1, 5, 1, 5, 1, 6, 1, 6, 
			1, 7, 1, 7, 1, 8, 1, 8, 1, 9, 1, 9, 1, 10, 1, 10, 1, 11, 1, 11, 1, 12, 
			1, 12, 1, 13, 1, 13, 1, 14, 1, 14, 1, 15, 1, 15, 1, 16, 1, 16, 1, 17, 
			1, 17, 1, 18, 1, 18, 1, 19, 1, 19, 1, 20, 1, 20, 1, 21, 1, 21, 1, 22, 
			1, 22, 1, 23, 1, 23, 1, 24, 1, 24, 1, 25, 1, 25, 1, 26, 1, 26, 1, 27, 
			1, 27, 1, 28, 1, 28, 1, 29, 1, 29, 1, 30, 1, 30, 1, 31, 1, 31, 1, 31, 
			1, 31, 1, 31, 1, 31, 1, 31, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 
			1, 32, 1, 32, 1, 33, 1, 33, 1, 33, 1, 34, 1, 34, 1, 34, 1, 34, 1, 34, 
			1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 1, 36, 1, 36, 1, 36, 1, 36, 1, 36, 
			1, 37, 1, 37, 1, 37, 1, 37, 1, 37, 1, 38, 1, 38, 1, 38, 1, 38, 1, 38, 
			1, 38, 1, 39, 1, 39, 1, 39, 1, 39, 1, 39, 1, 39, 1, 39, 1, 39, 1, 39, 
			1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 41, 1, 41, 
			1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 41, 1, 42, 1, 42, 1, 42, 
			1, 42, 1, 42, 1, 42, 1, 42, 1, 42, 1, 43, 1, 43, 1, 43, 1, 43, 1, 43, 
			1, 43, 1, 43, 1, 43, 1, 43, 1, 44, 1, 44, 1, 45, 1, 45, 1, 46, 1, 46, 
			1, 47, 1, 47, 1, 48, 1, 48, 1, 49, 1, 49, 1, 50, 1, 50, 1, 51, 1, 51, 
			1, 52, 1, 52, 1, 53, 1, 53, 1, 54, 1, 54, 1, 55, 1, 55, 1, 56, 1, 56, 
			1, 57, 1, 57, 1, 57, 1, 58, 1, 58, 1, 58, 1, 59, 1, 59, 1, 59, 1, 60, 
			1, 60, 1, 60, 1, 61, 1, 61, 1, 62, 1, 62, 1, 62, 1, 63, 1, 63, 1, 64, 
			1, 64, 1, 64, 1, 65, 1, 65, 1, 66, 1, 66, 1, 67, 1, 67, 1, 68, 1, 68, 
			1, 69, 1, 69, 1, 69, 1, 70, 1, 70, 5, 70, 404, 8, 70, 10, 70, 12, 70, 
			407, 9, 70, 1, 71, 1, 71, 1, 72, 1, 72, 1, 72, 1, 72, 1, 73, 1, 73, 5, 
			73, 417, 8, 73, 10, 73, 12, 73, 420, 9, 73, 0, 0, 74, 1, 1, 3, 2, 5, 
			3, 7, 4, 9, 5, 11, 0, 13, 0, 15, 0, 17, 0, 19, 0, 21, 0, 23, 0, 25, 0, 
			27, 0, 29, 0, 31, 0, 33, 0, 35, 0, 37, 0, 39, 0, 41, 0, 43, 0, 45, 0, 
			47, 0, 49, 0, 51, 0, 53, 0, 55, 0, 57, 0, 59, 0, 61, 0, 63, 6, 65, 7, 
			67, 8, 69, 9, 71, 10, 73, 11, 75, 12, 77, 13, 79, 14, 81, 15, 83, 16, 
			85, 17, 87, 18, 89, 19, 91, 20, 93, 21, 95, 22, 97, 23, 99, 24, 101, 
			25, 103, 26, 105, 27, 107, 28, 109, 29, 111, 30, 113, 31, 115, 32, 117, 
			33, 119, 34, 121, 35, 123, 36, 125, 37, 127, 38, 129, 39, 131, 40, 133, 
			41, 135, 42, 137, 43, 139, 44, 141, 45, 143, 46, 145, 47, 147, 48, 1, 
			0, 35, 2, 0, 10, 10, 13, 13, 1, 0, 42, 42, 2, 0, 42, 42, 47, 47, 2, 0, 
			9, 9, 32, 32, 2, 0, 65, 65, 97, 97, 2, 0, 66, 66, 98, 98, 2, 0, 67, 67, 
			99, 99, 2, 0, 68, 68, 100, 100, 2, 0, 69, 69, 101, 101, 2, 0, 70, 70, 
			102, 102, 2, 0, 71, 71, 103, 103, 2, 0, 72, 72, 104, 104, 2, 0, 73, 73, 
			105, 105, 2, 0, 74, 74, 106, 106, 2, 0, 75, 75, 107, 107, 2, 0, 76, 76, 
			108, 108, 2, 0, 77, 77, 109, 109, 2, 0, 78, 78, 110, 110, 2, 0, 79, 79, 
			111, 111, 2, 0, 80, 80, 112, 112, 2, 0, 81, 81, 113, 113, 2, 0, 82, 82, 
			114, 114, 2, 0, 83, 83, 115, 115, 2, 0, 84, 84, 116, 116, 2, 0, 85, 85, 
			117, 117, 2, 0, 86, 86, 118, 118, 2, 0, 87, 87, 119, 119, 2, 0, 88, 88, 
			120, 120, 2, 0, 89, 89, 121, 121, 2, 0, 90, 90, 122, 122, 1, 0, 49, 57, 
			1, 0, 48, 57, 1, 0, 34, 34, 4, 0, 36, 36, 65, 90, 95, 95, 97, 122, 5, 
			0, 36, 36, 48, 57, 65, 90, 95, 95, 97, 122, 403, 0, 1, 1, 0, 0, 0, 0, 
			3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 
			0, 63, 1, 0, 0, 0, 0, 65, 1, 0, 0, 0, 0, 67, 1, 0, 0, 0, 0, 69, 1, 0, 
			0, 0, 0, 71, 1, 0, 0, 0, 0, 73, 1, 0, 0, 0, 0, 75, 1, 0, 0, 0, 0, 77, 
			1, 0, 0, 0, 0, 79, 1, 0, 0, 0, 0, 81, 1, 0, 0, 0, 0, 83, 1, 0, 0, 0, 
			0, 85, 1, 0, 0, 0, 0, 87, 1, 0, 0, 0, 0, 89, 1, 0, 0, 0, 0, 91, 1, 0, 
			0, 0, 0, 93, 1, 0, 0, 0, 0, 95, 1, 0, 0, 0, 0, 97, 1, 0, 0, 0, 0, 99, 
			1, 0, 0, 0, 0, 101, 1, 0, 0, 0, 0, 103, 1, 0, 0, 0, 0, 105, 1, 0, 0, 
			0, 0, 107, 1, 0, 0, 0, 0, 109, 1, 0, 0, 0, 0, 111, 1, 0, 0, 0, 0, 113, 
			1, 0, 0, 0, 0, 115, 1, 0, 0, 0, 0, 117, 1, 0, 0, 0, 0, 119, 1, 0, 0, 
			0, 0, 121, 1, 0, 0, 0, 0, 123, 1, 0, 0, 0, 0, 125, 1, 0, 0, 0, 0, 127, 
			1, 0, 0, 0, 0, 129, 1, 0, 0, 0, 0, 131, 1, 0, 0, 0, 0, 133, 1, 0, 0, 
			0, 0, 135, 1, 0, 0, 0, 0, 137, 1, 0, 0, 0, 0, 139, 1, 0, 0, 0, 0, 141, 
			1, 0, 0, 0, 0, 143, 1, 0, 0, 0, 0, 145, 1, 0, 0, 0, 0, 147, 1, 0, 0, 
			0, 1, 149, 1, 0, 0, 0, 3, 160, 1, 0, 0, 0, 5, 184, 1, 0, 0, 0, 7, 191, 
			1, 0, 0, 0, 9, 195, 1, 0, 0, 0, 11, 203, 1, 0, 0, 0, 13, 205, 1, 0, 0, 
			0, 15, 207, 1, 0, 0, 0, 17, 209, 1, 0, 0, 0, 19, 211, 1, 0, 0, 0, 21, 
			213, 1, 0, 0, 0, 23, 215, 1, 0, 0, 0, 25, 217, 1, 0, 0, 0, 27, 219, 1, 
			0, 0, 0, 29, 221, 1, 0, 0, 0, 31, 223, 1, 0, 0, 0, 33, 225, 1, 0, 0, 
			0, 35, 227, 1, 0, 0, 0, 37, 229, 1, 0, 0, 0, 39, 231, 1, 0, 0, 0, 41, 
			233, 1, 0, 0, 0, 43, 235, 1, 0, 0, 0, 45, 237, 1, 0, 0, 0, 47, 239, 1, 
			0, 0, 0, 49, 241, 1, 0, 0, 0, 51, 243, 1, 0, 0, 0, 53, 245, 1, 0, 0, 
			0, 55, 247, 1, 0, 0, 0, 57, 249, 1, 0, 0, 0, 59, 251, 1, 0, 0, 0, 61, 
			253, 1, 0, 0, 0, 63, 255, 1, 0, 0, 0, 65, 262, 1, 0, 0, 0, 67, 270, 1, 
			0, 0, 0, 69, 273, 1, 0, 0, 0, 71, 278, 1, 0, 0, 0, 73, 283, 1, 0, 0, 
			0, 75, 288, 1, 0, 0, 0, 77, 293, 1, 0, 0, 0, 79, 299, 1, 0, 0, 0, 81, 
			308, 1, 0, 0, 0, 83, 316, 1, 0, 0, 0, 85, 325, 1, 0, 0, 0, 87, 333, 1, 
			0, 0, 0, 89, 342, 1, 0, 0, 0, 91, 344, 1, 0, 0, 0, 93, 346, 1, 0, 0, 
			0, 95, 348, 1, 0, 0, 0, 97, 350, 1, 0, 0, 0, 99, 352, 1, 0, 0, 0, 101, 
			354, 1, 0, 0, 0, 103, 356, 1, 0, 0, 0, 105, 358, 1, 0, 0, 0, 107, 360, 
			1, 0, 0, 0, 109, 362, 1, 0, 0, 0, 111, 364, 1, 0, 0, 0, 113, 366, 1, 
			0, 0, 0, 115, 368, 1, 0, 0, 0, 117, 371, 1, 0, 0, 0, 119, 374, 1, 0, 
			0, 0, 121, 377, 1, 0, 0, 0, 123, 380, 1, 0, 0, 0, 125, 382, 1, 0, 0, 
			0, 127, 385, 1, 0, 0, 0, 129, 387, 1, 0, 0, 0, 131, 390, 1, 0, 0, 0, 
			133, 392, 1, 0, 0, 0, 135, 394, 1, 0, 0, 0, 137, 396, 1, 0, 0, 0, 139, 
			398, 1, 0, 0, 0, 141, 401, 1, 0, 0, 0, 143, 408, 1, 0, 0, 0, 145, 410, 
			1, 0, 0, 0, 147, 414, 1, 0, 0, 0, 149, 150, 5, 47, 0, 0, 150, 151, 5, 
			47, 0, 0, 151, 155, 1, 0, 0, 0, 152, 154, 8, 0, 0, 0, 153, 152, 1, 0, 
			0, 0, 154, 157, 1, 0, 0, 0, 155, 153, 1, 0, 0, 0, 155, 156, 1, 0, 0, 
			0, 156, 158, 1, 0, 0, 0, 157, 155, 1, 0, 0, 0, 158, 159, 6, 0, 0, 0, 
			159, 2, 1, 0, 0, 0, 160, 161, 5, 47, 0, 0, 161, 162, 5, 42, 0, 0, 162, 
			170, 1, 0, 0, 0, 163, 171, 8, 1, 0, 0, 164, 166, 5, 42, 0, 0, 165, 164, 
			1, 0, 0, 0, 166, 167, 1, 0, 0, 0, 167, 165, 1, 0, 0, 0, 167, 168, 1, 
			0, 0, 0, 168, 169, 1, 0, 0, 0, 169, 171, 8, 2, 0, 0, 170, 163, 1, 0, 
			0, 0, 170, 165, 1, 0, 0, 0, 171, 175, 1, 0, 0, 0, 172, 174, 5, 42, 0, 
			0, 173, 172, 1, 0, 0, 0, 174, 177, 1, 0, 0, 0, 175, 173, 1, 0, 0, 0, 
			175, 176, 1, 0, 0, 0, 176, 178, 1, 0, 0, 0, 177, 175, 1, 0, 0, 0, 178, 
			179, 5, 42, 0, 0, 179, 180, 5, 47, 0, 0, 180, 181, 1, 0, 0, 0, 181, 182, 
			6, 1, 0, 0, 182, 4, 1, 0, 0, 0, 183, 185, 7, 3, 0, 0, 184, 183, 1, 0, 
			0, 0, 185, 186, 1, 0, 0, 0, 186, 184, 1, 0, 0, 0, 186, 187, 1, 0, 0, 
			0, 187, 188, 1, 0, 0, 0, 188, 189, 6, 2, 0, 0, 189, 6, 1, 0, 0, 0, 190, 
			192, 7, 0, 0, 0, 191, 190, 1, 0, 0, 0, 192, 193, 1, 0, 0, 0, 193, 191, 
			1, 0, 0, 0, 193, 194, 1, 0, 0, 0, 194, 8, 1, 0, 0, 0, 195, 197, 5, 92, 
			0, 0, 196, 198, 7, 0, 0, 0, 197, 196, 1, 0, 0, 0, 198, 199, 1, 0, 0, 
			0, 199, 197, 1, 0, 0, 0, 199, 200, 1, 0, 0, 0, 200, 201, 1, 0, 0, 0, 
			201, 202, 6, 4, 0, 0, 202, 10, 1, 0, 0, 0, 203, 204, 7, 4, 0, 0, 204, 
			12, 1, 0, 0, 0, 205, 206, 7, 5, 0, 0, 206, 14, 1, 0, 0, 0, 207, 208, 
			7, 6, 0, 0, 208, 16, 1, 0, 0, 0, 209, 210, 7, 7, 0, 0, 210, 18, 1, 0, 
			0, 0, 211, 212, 7, 8, 0, 0, 212, 20, 1, 0, 0, 0, 213, 214, 7, 9, 0, 0, 
			214, 22, 1, 0, 0, 0, 215, 216, 7, 10, 0, 0, 216, 24, 1, 0, 0, 0, 217, 
			218, 7, 11, 0, 0, 218, 26, 1, 0, 0, 0, 219, 220, 7, 12, 0, 0, 220, 28, 
			1, 0, 0, 0, 221, 222, 7, 13, 0, 0, 222, 30, 1, 0, 0, 0, 223, 224, 7, 
			14, 0, 0, 224, 32, 1, 0, 0, 0, 225, 226, 7, 15, 0, 0, 226, 34, 1, 0, 
			0, 0, 227, 228, 7, 16, 0, 0, 228, 36, 1, 0, 0, 0, 229, 230, 7, 17, 0, 
			0, 230, 38, 1, 0, 0, 0, 231, 232, 7, 18, 0, 0, 232, 40, 1, 0, 0, 0, 233, 
			234, 7, 19, 0, 0, 234, 42, 1, 0, 0, 0, 235, 236, 7, 20, 0, 0, 236, 44, 
			1, 0, 0, 0, 237, 238, 7, 21, 0, 0, 238, 46, 1, 0, 0, 0, 239, 240, 7, 
			22, 0, 0, 240, 48, 1, 0, 0, 0, 241, 242, 7, 23, 0, 0, 242, 50, 1, 0, 
			0, 0, 243, 244, 7, 24, 0, 0, 244, 52, 1, 0, 0, 0, 245, 246, 7, 25, 0, 
			0, 246, 54, 1, 0, 0, 0, 247, 248, 7, 26, 0, 0, 248, 56, 1, 0, 0, 0, 249, 
			250, 7, 27, 0, 0, 250, 58, 1, 0, 0, 0, 251, 252, 7, 28, 0, 0, 252, 60, 
			1, 0, 0, 0, 253, 254, 7, 29, 0, 0, 254, 62, 1, 0, 0, 0, 255, 256, 3, 
			11, 5, 0, 256, 257, 3, 47, 23, 0, 257, 258, 3, 47, 23, 0, 258, 259, 3, 
			19, 9, 0, 259, 260, 3, 45, 22, 0, 260, 261, 3, 49, 24, 0, 261, 64, 1, 
			0, 0, 0, 262, 263, 3, 35, 17, 0, 263, 264, 3, 11, 5, 0, 264, 265, 3, 
			49, 24, 0, 265, 266, 3, 15, 7, 0, 266, 267, 3, 25, 12, 0, 267, 268, 3, 
			19, 9, 0, 268, 269, 3, 47, 23, 0, 269, 66, 1, 0, 0, 0, 270, 271, 3, 27, 
			13, 0, 271, 272, 3, 21, 10, 0, 272, 68, 1, 0, 0, 0, 273, 274, 3, 49, 
			24, 0, 274, 275, 3, 25, 12, 0, 275, 276, 3, 19, 9, 0, 276, 277, 3, 37, 
			18, 0, 277, 70, 1, 0, 0, 0, 278, 279, 3, 19, 9, 0, 279, 280, 3, 33, 16, 
			0, 280, 281, 3, 47, 23, 0, 281, 282, 3, 19, 9, 0, 282, 72, 1, 0, 0, 0, 
			283, 284, 3, 37, 18, 0, 284, 285, 3, 51, 25, 0, 285, 286, 3, 33, 16, 
			0, 286, 287, 3, 33, 16, 0, 287, 74, 1, 0, 0, 0, 288, 289, 3, 49, 24, 
			0, 289, 290, 3, 45, 22, 0, 290, 291, 3, 51, 25, 0, 291, 292, 3, 19, 9, 
			0, 292, 76, 1, 0, 0, 0, 293, 294, 3, 21, 10, 0, 294, 295, 3, 11, 5, 0, 
			295, 296, 3, 33, 16, 0, 296, 297, 3, 47, 23, 0, 297, 298, 3, 19, 9, 0, 
			298, 78, 1, 0, 0, 0, 299, 300, 3, 35, 17, 0, 300, 301, 3, 19, 9, 0, 301, 
			302, 3, 49, 24, 0, 302, 303, 3, 11, 5, 0, 303, 304, 3, 13, 6, 0, 304, 
			305, 3, 39, 19, 0, 305, 306, 3, 39, 19, 0, 306, 307, 3, 33, 16, 0, 307, 
			80, 1, 0, 0, 0, 308, 309, 3, 35, 17, 0, 309, 310, 3, 19, 9, 0, 310, 311, 
			3, 49, 24, 0, 311, 312, 3, 11, 5, 0, 312, 313, 3, 27, 13, 0, 313, 314, 
			3, 37, 18, 0, 314, 315, 3, 49, 24, 0, 315, 82, 1, 0, 0, 0, 316, 317, 
			3, 35, 17, 0, 317, 318, 3, 19, 9, 0, 318, 319, 3, 49, 24, 0, 319, 320, 
			3, 11, 5, 0, 320, 321, 3, 19, 9, 0, 321, 322, 3, 37, 18, 0, 322, 323, 
			3, 51, 25, 0, 323, 324, 3, 35, 17, 0, 324, 84, 1, 0, 0, 0, 325, 326, 
			3, 35, 17, 0, 326, 327, 3, 19, 9, 0, 327, 328, 3, 49, 24, 0, 328, 329, 
			3, 11, 5, 0, 329, 330, 3, 47, 23, 0, 330, 331, 3, 49, 24, 0, 331, 332, 
			3, 45, 22, 0, 332, 86, 1, 0, 0, 0, 333, 334, 3, 49, 24, 0, 334, 335, 
			3, 59, 29, 0, 335, 336, 3, 41, 20, 0, 336, 337, 3, 19, 9, 0, 337, 338, 
			3, 37, 18, 0, 338, 339, 3, 11, 5, 0, 339, 340, 3, 35, 17, 0, 340, 341, 
			3, 19, 9, 0, 341, 88, 1, 0, 0, 0, 342, 343, 5, 46, 0, 0, 343, 90, 1, 
			0, 0, 0, 344, 345, 5, 44, 0, 0, 345, 92, 1, 0, 0, 0, 346, 347, 5, 58, 
			0, 0, 347, 94, 1, 0, 0, 0, 348, 349, 5, 59, 0, 0, 349, 96, 1, 0, 0, 0, 
			350, 351, 5, 63, 0, 0, 351, 98, 1, 0, 0, 0, 352, 353, 5, 33, 0, 0, 353, 
			100, 1, 0, 0, 0, 354, 355, 5, 40, 0, 0, 355, 102, 1, 0, 0, 0, 356, 357, 
			5, 41, 0, 0, 357, 104, 1, 0, 0, 0, 358, 359, 5, 123, 0, 0, 359, 106, 
			1, 0, 0, 0, 360, 361, 5, 125, 0, 0, 361, 108, 1, 0, 0, 0, 362, 363, 5, 
			91, 0, 0, 363, 110, 1, 0, 0, 0, 364, 365, 5, 93, 0, 0, 365, 112, 1, 0, 
			0, 0, 366, 367, 5, 61, 0, 0, 367, 114, 1, 0, 0, 0, 368, 369, 5, 124, 
			0, 0, 369, 370, 5, 124, 0, 0, 370, 116, 1, 0, 0, 0, 371, 372, 5, 38, 
			0, 0, 372, 373, 5, 38, 0, 0, 373, 118, 1, 0, 0, 0, 374, 375, 5, 61, 0, 
			0, 375, 376, 5, 61, 0, 0, 376, 120, 1, 0, 0, 0, 377, 378, 5, 33, 0, 0, 
			378, 379, 5, 61, 0, 0, 379, 122, 1, 0, 0, 0, 380, 381, 5, 60, 0, 0, 381, 
			124, 1, 0, 0, 0, 382, 383, 5, 60, 0, 0, 383, 384, 5, 61, 0, 0, 384, 126, 
			1, 0, 0, 0, 385, 386, 5, 62, 0, 0, 386, 128, 1, 0, 0, 0, 387, 388, 5, 
			62, 0, 0, 388, 389, 5, 61, 0, 0, 389, 130, 1, 0, 0, 0, 390, 391, 5, 43, 
			0, 0, 391, 132, 1, 0, 0, 0, 392, 393, 5, 45, 0, 0, 393, 134, 1, 0, 0, 
			0, 394, 395, 5, 42, 0, 0, 395, 136, 1, 0, 0, 0, 396, 397, 5, 47, 0, 0, 
			397, 138, 1, 0, 0, 0, 398, 399, 5, 46, 0, 0, 399, 400, 5, 46, 0, 0, 400, 
			140, 1, 0, 0, 0, 401, 405, 7, 30, 0, 0, 402, 404, 7, 31, 0, 0, 403, 402, 
			1, 0, 0, 0, 404, 407, 1, 0, 0, 0, 405, 403, 1, 0, 0, 0, 405, 406, 1, 
			0, 0, 0, 406, 142, 1, 0, 0, 0, 407, 405, 1, 0, 0, 0, 408, 409, 5, 48, 
			0, 0, 409, 144, 1, 0, 0, 0, 410, 411, 5, 34, 0, 0, 411, 412, 8, 32, 0, 
			0, 412, 413, 5, 34, 0, 0, 413, 146, 1, 0, 0, 0, 414, 418, 7, 33, 0, 0, 
			415, 417, 7, 34, 0, 0, 416, 415, 1, 0, 0, 0, 417, 420, 1, 0, 0, 0, 418, 
			416, 1, 0, 0, 0, 418, 419, 1, 0, 0, 0, 419, 148, 1, 0, 0, 0, 420, 418, 
			1, 0, 0, 0, 10, 0, 155, 167, 170, 175, 186, 193, 199, 405, 418, 1, 0, 
			1, 0
		];
	}