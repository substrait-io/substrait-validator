// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(nonstandard_style)]
// Generated from SubstraitType.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::substraittypeparser::*;

pub trait SubstraitTypeListener<'input> : ParseTreeListener<'input,SubstraitTypeParserContextType>{
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#startPattern}.
 * @param ctx the parse tree
 */
fn enter_startPattern(&mut self, _ctx: &StartPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#startPattern}.
 * @param ctx the parse tree
 */
fn exit_startPattern(&mut self, _ctx: &StartPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#startProgram}.
 * @param ctx the parse tree
 */
fn enter_startProgram(&mut self, _ctx: &StartProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#startProgram}.
 * @param ctx the parse tree
 */
fn exit_startProgram(&mut self, _ctx: &StartProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#statementSeparator}.
 * @param ctx the parse tree
 */
fn enter_statementSeparator(&mut self, _ctx: &StatementSeparatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#statementSeparator}.
 * @param ctx the parse tree
 */
fn exit_statementSeparator(&mut self, _ctx: &StatementSeparatorContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Normal}
 * labeled alternative in {@link SubstraitTypeParser#statement}.
 * @param ctx the parse tree
 */
fn enter_Normal(&mut self, _ctx: &NormalContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Normal}
 * labeled alternative in {@link SubstraitTypeParser#statement}.
 * @param ctx the parse tree
 */
fn exit_Normal(&mut self, _ctx: &NormalContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Match}
 * labeled alternative in {@link SubstraitTypeParser#statement}.
 * @param ctx the parse tree
 */
fn enter_Match(&mut self, _ctx: &MatchContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Match}
 * labeled alternative in {@link SubstraitTypeParser#statement}.
 * @param ctx the parse tree
 */
fn exit_Match(&mut self, _ctx: &MatchContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Assert}
 * labeled alternative in {@link SubstraitTypeParser#statement}.
 * @param ctx the parse tree
 */
fn enter_Assert(&mut self, _ctx: &AssertContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Assert}
 * labeled alternative in {@link SubstraitTypeParser#statement}.
 * @param ctx the parse tree
 */
fn exit_Assert(&mut self, _ctx: &AssertContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ValidPattern}
 * labeled alternative in {@link SubstraitTypeParser#patternInvalidIfThenElse}.
 * @param ctx the parse tree
 */
fn enter_ValidPattern(&mut self, _ctx: &ValidPatternContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ValidPattern}
 * labeled alternative in {@link SubstraitTypeParser#patternInvalidIfThenElse}.
 * @param ctx the parse tree
 */
fn exit_ValidPattern(&mut self, _ctx: &ValidPatternContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code InvalidIfThenElse}
 * labeled alternative in {@link SubstraitTypeParser#patternInvalidIfThenElse}.
 * @param ctx the parse tree
 */
fn enter_InvalidIfThenElse(&mut self, _ctx: &InvalidIfThenElseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code InvalidIfThenElse}
 * labeled alternative in {@link SubstraitTypeParser#patternInvalidIfThenElse}.
 * @param ctx the parse tree
 */
fn exit_InvalidIfThenElse(&mut self, _ctx: &InvalidIfThenElseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#patternOr}.
 * @param ctx the parse tree
 */
fn enter_patternOr(&mut self, _ctx: &PatternOrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#patternOr}.
 * @param ctx the parse tree
 */
fn exit_patternOr(&mut self, _ctx: &PatternOrContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Or}
 * labeled alternative in {@link SubstraitTypeParser#operatorOr}.
 * @param ctx the parse tree
 */
fn enter_Or(&mut self, _ctx: &OrContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Or}
 * labeled alternative in {@link SubstraitTypeParser#operatorOr}.
 * @param ctx the parse tree
 */
fn exit_Or(&mut self, _ctx: &OrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#patternAnd}.
 * @param ctx the parse tree
 */
fn enter_patternAnd(&mut self, _ctx: &PatternAndContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#patternAnd}.
 * @param ctx the parse tree
 */
fn exit_patternAnd(&mut self, _ctx: &PatternAndContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code And}
 * labeled alternative in {@link SubstraitTypeParser#operatorAnd}.
 * @param ctx the parse tree
 */
fn enter_And(&mut self, _ctx: &AndContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code And}
 * labeled alternative in {@link SubstraitTypeParser#operatorAnd}.
 * @param ctx the parse tree
 */
fn exit_And(&mut self, _ctx: &AndContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#patternEqNeq}.
 * @param ctx the parse tree
 */
fn enter_patternEqNeq(&mut self, _ctx: &PatternEqNeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#patternEqNeq}.
 * @param ctx the parse tree
 */
fn exit_patternEqNeq(&mut self, _ctx: &PatternEqNeqContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Eq}
 * labeled alternative in {@link SubstraitTypeParser#operatorEqNeq}.
 * @param ctx the parse tree
 */
fn enter_Eq(&mut self, _ctx: &EqContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Eq}
 * labeled alternative in {@link SubstraitTypeParser#operatorEqNeq}.
 * @param ctx the parse tree
 */
fn exit_Eq(&mut self, _ctx: &EqContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Neq}
 * labeled alternative in {@link SubstraitTypeParser#operatorEqNeq}.
 * @param ctx the parse tree
 */
fn enter_Neq(&mut self, _ctx: &NeqContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Neq}
 * labeled alternative in {@link SubstraitTypeParser#operatorEqNeq}.
 * @param ctx the parse tree
 */
fn exit_Neq(&mut self, _ctx: &NeqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#patternIneq}.
 * @param ctx the parse tree
 */
fn enter_patternIneq(&mut self, _ctx: &PatternIneqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#patternIneq}.
 * @param ctx the parse tree
 */
fn exit_patternIneq(&mut self, _ctx: &PatternIneqContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Lt}
 * labeled alternative in {@link SubstraitTypeParser#operatorIneq}.
 * @param ctx the parse tree
 */
fn enter_Lt(&mut self, _ctx: &LtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Lt}
 * labeled alternative in {@link SubstraitTypeParser#operatorIneq}.
 * @param ctx the parse tree
 */
fn exit_Lt(&mut self, _ctx: &LtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Le}
 * labeled alternative in {@link SubstraitTypeParser#operatorIneq}.
 * @param ctx the parse tree
 */
fn enter_Le(&mut self, _ctx: &LeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Le}
 * labeled alternative in {@link SubstraitTypeParser#operatorIneq}.
 * @param ctx the parse tree
 */
fn exit_Le(&mut self, _ctx: &LeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Gt}
 * labeled alternative in {@link SubstraitTypeParser#operatorIneq}.
 * @param ctx the parse tree
 */
fn enter_Gt(&mut self, _ctx: &GtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Gt}
 * labeled alternative in {@link SubstraitTypeParser#operatorIneq}.
 * @param ctx the parse tree
 */
fn exit_Gt(&mut self, _ctx: &GtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Ge}
 * labeled alternative in {@link SubstraitTypeParser#operatorIneq}.
 * @param ctx the parse tree
 */
fn enter_Ge(&mut self, _ctx: &GeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Ge}
 * labeled alternative in {@link SubstraitTypeParser#operatorIneq}.
 * @param ctx the parse tree
 */
fn exit_Ge(&mut self, _ctx: &GeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#patternAddSub}.
 * @param ctx the parse tree
 */
fn enter_patternAddSub(&mut self, _ctx: &PatternAddSubContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#patternAddSub}.
 * @param ctx the parse tree
 */
fn exit_patternAddSub(&mut self, _ctx: &PatternAddSubContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Add}
 * labeled alternative in {@link SubstraitTypeParser#operatorAddSub}.
 * @param ctx the parse tree
 */
fn enter_Add(&mut self, _ctx: &AddContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Add}
 * labeled alternative in {@link SubstraitTypeParser#operatorAddSub}.
 * @param ctx the parse tree
 */
fn exit_Add(&mut self, _ctx: &AddContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Sub}
 * labeled alternative in {@link SubstraitTypeParser#operatorAddSub}.
 * @param ctx the parse tree
 */
fn enter_Sub(&mut self, _ctx: &SubContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Sub}
 * labeled alternative in {@link SubstraitTypeParser#operatorAddSub}.
 * @param ctx the parse tree
 */
fn exit_Sub(&mut self, _ctx: &SubContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#patternMulDiv}.
 * @param ctx the parse tree
 */
fn enter_patternMulDiv(&mut self, _ctx: &PatternMulDivContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#patternMulDiv}.
 * @param ctx the parse tree
 */
fn exit_patternMulDiv(&mut self, _ctx: &PatternMulDivContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Mul}
 * labeled alternative in {@link SubstraitTypeParser#operatorMulDiv}.
 * @param ctx the parse tree
 */
fn enter_Mul(&mut self, _ctx: &MulContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Mul}
 * labeled alternative in {@link SubstraitTypeParser#operatorMulDiv}.
 * @param ctx the parse tree
 */
fn exit_Mul(&mut self, _ctx: &MulContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Div}
 * labeled alternative in {@link SubstraitTypeParser#operatorMulDiv}.
 * @param ctx the parse tree
 */
fn enter_Div(&mut self, _ctx: &DivContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Div}
 * labeled alternative in {@link SubstraitTypeParser#operatorMulDiv}.
 * @param ctx the parse tree
 */
fn exit_Div(&mut self, _ctx: &DivContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parentheses}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_parentheses(&mut self, _ctx: &ParenthesesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parentheses}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_parentheses(&mut self, _ctx: &ParenthesesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ifThenElse}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_ifThenElse(&mut self, _ctx: &IfThenElseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ifThenElse}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_ifThenElse(&mut self, _ctx: &IfThenElseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unaryNot}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_unaryNot(&mut self, _ctx: &UnaryNotContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unaryNot}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_unaryNot(&mut self, _ctx: &UnaryNotContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code any}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_any(&mut self, _ctx: &AnyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code any}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_any(&mut self, _ctx: &AnyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code boolAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_boolAny(&mut self, _ctx: &BoolAnyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code boolAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_boolAny(&mut self, _ctx: &BoolAnyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code boolTrue}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_boolTrue(&mut self, _ctx: &BoolTrueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code boolTrue}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_boolTrue(&mut self, _ctx: &BoolTrueContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code boolFalse}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_boolFalse(&mut self, _ctx: &BoolFalseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code boolFalse}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_boolFalse(&mut self, _ctx: &BoolFalseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code intAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_intAny(&mut self, _ctx: &IntAnyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_intAny(&mut self, _ctx: &IntAnyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code intRange}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_intRange(&mut self, _ctx: &IntRangeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intRange}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_intRange(&mut self, _ctx: &IntRangeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code intAtLeast}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_intAtLeast(&mut self, _ctx: &IntAtLeastContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intAtLeast}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_intAtLeast(&mut self, _ctx: &IntAtLeastContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code intAtMost}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_intAtMost(&mut self, _ctx: &IntAtMostContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intAtMost}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_intAtMost(&mut self, _ctx: &IntAtMostContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code intExactly}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_intExactly(&mut self, _ctx: &IntExactlyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intExactly}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_intExactly(&mut self, _ctx: &IntExactlyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code enumAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_enumAny(&mut self, _ctx: &EnumAnyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code enumAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_enumAny(&mut self, _ctx: &EnumAnyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code enumSet}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_enumSet(&mut self, _ctx: &EnumSetContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code enumSet}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_enumSet(&mut self, _ctx: &EnumSetContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code strAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_strAny(&mut self, _ctx: &StrAnyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code strAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_strAny(&mut self, _ctx: &StrAnyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code strExactly}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_strExactly(&mut self, _ctx: &StrExactlyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code strExactly}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_strExactly(&mut self, _ctx: &StrExactlyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code dtAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_dtAny(&mut self, _ctx: &DtAnyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code dtAny}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_dtAny(&mut self, _ctx: &DtAnyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code function}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_function(&mut self, _ctx: &FunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code function}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_function(&mut self, _ctx: &FunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code datatypeBindingOrConstant}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_datatypeBindingOrConstant(&mut self, _ctx: &DatatypeBindingOrConstantContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code datatypeBindingOrConstant}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_datatypeBindingOrConstant(&mut self, _ctx: &DatatypeBindingOrConstantContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code inconsistent}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_inconsistent(&mut self, _ctx: &InconsistentContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code inconsistent}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_inconsistent(&mut self, _ctx: &InconsistentContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unaryNegate}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn enter_unaryNegate(&mut self, _ctx: &UnaryNegateContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unaryNegate}
 * labeled alternative in {@link SubstraitTypeParser#patternMisc}.
 * @param ctx the parse tree
 */
fn exit_unaryNegate(&mut self, _ctx: &UnaryNegateContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nonNullable}
 * labeled alternative in {@link SubstraitTypeParser#nullability}.
 * @param ctx the parse tree
 */
fn enter_nonNullable(&mut self, _ctx: &NonNullableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nonNullable}
 * labeled alternative in {@link SubstraitTypeParser#nullability}.
 * @param ctx the parse tree
 */
fn exit_nonNullable(&mut self, _ctx: &NonNullableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nullable}
 * labeled alternative in {@link SubstraitTypeParser#nullability}.
 * @param ctx the parse tree
 */
fn enter_nullable(&mut self, _ctx: &NullableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nullable}
 * labeled alternative in {@link SubstraitTypeParser#nullability}.
 * @param ctx the parse tree
 */
fn exit_nullable(&mut self, _ctx: &NullableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nullableIf}
 * labeled alternative in {@link SubstraitTypeParser#nullability}.
 * @param ctx the parse tree
 */
fn enter_nullableIf(&mut self, _ctx: &NullableIfContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nullableIf}
 * labeled alternative in {@link SubstraitTypeParser#nullability}.
 * @param ctx the parse tree
 */
fn exit_nullableIf(&mut self, _ctx: &NullableIfContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#variation}.
 * @param ctx the parse tree
 */
fn enter_variation(&mut self, _ctx: &VariationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#variation}.
 * @param ctx the parse tree
 */
fn exit_variation(&mut self, _ctx: &VariationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code varAny}
 * labeled alternative in {@link SubstraitTypeParser#variationBody}.
 * @param ctx the parse tree
 */
fn enter_varAny(&mut self, _ctx: &VarAnyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code varAny}
 * labeled alternative in {@link SubstraitTypeParser#variationBody}.
 * @param ctx the parse tree
 */
fn exit_varAny(&mut self, _ctx: &VarAnyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code varSystemPreferred}
 * labeled alternative in {@link SubstraitTypeParser#variationBody}.
 * @param ctx the parse tree
 */
fn enter_varSystemPreferred(&mut self, _ctx: &VarSystemPreferredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code varSystemPreferred}
 * labeled alternative in {@link SubstraitTypeParser#variationBody}.
 * @param ctx the parse tree
 */
fn exit_varSystemPreferred(&mut self, _ctx: &VarSystemPreferredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code varUserDefined}
 * labeled alternative in {@link SubstraitTypeParser#variationBody}.
 * @param ctx the parse tree
 */
fn enter_varUserDefined(&mut self, _ctx: &VarUserDefinedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code varUserDefined}
 * labeled alternative in {@link SubstraitTypeParser#variationBody}.
 * @param ctx the parse tree
 */
fn exit_varUserDefined(&mut self, _ctx: &VarUserDefinedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#parameters}.
 * @param ctx the parse tree
 */
fn enter_parameters(&mut self, _ctx: &ParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#parameters}.
 * @param ctx the parse tree
 */
fn exit_parameters(&mut self, _ctx: &ParametersContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#parameter}.
 * @param ctx the parse tree
 */
fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#parameter}.
 * @param ctx the parse tree
 */
fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Null}
 * labeled alternative in {@link SubstraitTypeParser#parameterValue}.
 * @param ctx the parse tree
 */
fn enter_Null(&mut self, _ctx: &NullContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Null}
 * labeled alternative in {@link SubstraitTypeParser#parameterValue}.
 * @param ctx the parse tree
 */
fn exit_Null(&mut self, _ctx: &NullContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Specified}
 * labeled alternative in {@link SubstraitTypeParser#parameterValue}.
 * @param ctx the parse tree
 */
fn enter_Specified(&mut self, _ctx: &SpecifiedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Specified}
 * labeled alternative in {@link SubstraitTypeParser#parameterValue}.
 * @param ctx the parse tree
 */
fn exit_Specified(&mut self, _ctx: &SpecifiedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#integer}.
 * @param ctx the parse tree
 */
fn enter_integer(&mut self, _ctx: &IntegerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#integer}.
 * @param ctx the parse tree
 */
fn exit_integer(&mut self, _ctx: &IntegerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#identifierPath}.
 * @param ctx the parse tree
 */
fn enter_identifierPath(&mut self, _ctx: &IdentifierPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#identifierPath}.
 * @param ctx the parse tree
 */
fn exit_identifierPath(&mut self, _ctx: &IdentifierPathContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Str}
 * labeled alternative in {@link SubstraitTypeParser#identifierOrString}.
 * @param ctx the parse tree
 */
fn enter_Str(&mut self, _ctx: &StrContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Str}
 * labeled alternative in {@link SubstraitTypeParser#identifierOrString}.
 * @param ctx the parse tree
 */
fn exit_Str(&mut self, _ctx: &StrContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Ident}
 * labeled alternative in {@link SubstraitTypeParser#identifierOrString}.
 * @param ctx the parse tree
 */
fn enter_Ident(&mut self, _ctx: &IdentContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Ident}
 * labeled alternative in {@link SubstraitTypeParser#identifierOrString}.
 * @param ctx the parse tree
 */
fn exit_Ident(&mut self, _ctx: &IdentContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : SubstraitTypeListener<'input> }


