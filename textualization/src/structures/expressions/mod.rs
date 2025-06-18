use dyn_clone::DynClone;
use parsertools::parsers::{helpers::lazy, tokens::pred, Parser};
use tbl_structures::propositions::Expression;

use crate::structures::expressions::{raw::{raw_expression_parser, RawExpressionStyle}};

pub mod raw;
pub mod patterns;
pub mod functional;

pub struct ExpressionControls<'a> {
    raw_expr_style: RawExpressionStyle,
    special_cases: SpecialCases<'a>
}
impl <'a> ExpressionControls<'a> {
    pub fn new(raw_expr_style: RawExpressionStyle, special_cases: SpecialCases<'a>) -> Self { Self { raw_expr_style, special_cases } }

    pub fn raw_expr_style(&self) -> &RawExpressionStyle { &self.raw_expr_style }
    pub fn special_cases(&self) -> &SpecialCases<'a> { &self.special_cases }
}

pub trait SpecialCase<'a>: Sync + Send + DynClone {
    fn parser(&'a self, expr_parser: Parser<'a,char,Expression>) -> Parser<'a,char,Expression>;
}
impl <'a> Clone for Box<dyn SpecialCase<'a>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) } }
pub type SpecialCases<'a> = Vec<Box<dyn SpecialCase<'a>>>;

pub fn expression_parser<'a>(style: &'a ExpressionControls<'a>) -> Parser<'a,char,Expression> {
    raw_expression_parser(style.raw_expr_style())
        .or(processed_expression_parser(style))
}

fn processed_expression_parser<'a>(style: &'a ExpressionControls<'a>) -> Parser<'a,char,Expression> {
    let expression_parser = lazy(|| expression_parser(style));
    let iter = style.special_cases().iter().map(move |case| case.parser(expression_parser.clone()));
    iter.reduce(|acc, next| acc.or(next))
        .unwrap_or(pred(|_| None))
}
