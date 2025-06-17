use dyn_clone::DynClone;
use parsertools::parsers::{helpers::lazy, tokens::pred, Parser};
use tbl_structures::propositions::Expression;

use crate::structures::expressions::raw::{raw_expression_parser, RawExpressionControls};

pub mod raw;
pub mod patterns;
pub mod functional;

pub struct ExpressionControls<'a> {
    raw_controls: RawExpressionControls,
    special_cases: SpecialCases<'a>
}
impl <'a> ExpressionControls<'a> {
    pub fn raw_controls(&self) -> &RawExpressionControls { &self.raw_controls }
    pub fn special_cases(&self) -> &SpecialCases<'a> { &self.special_cases }
}

pub trait SpecialCase<'a>: Sync + Send + DynClone {
    fn parser(&'a self, expr_parser: Parser<'a,char,Expression>) -> Parser<'a,char,Expression>;
}
impl <'a> Clone for Box<dyn SpecialCase<'a>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) } }
pub type SpecialCases<'a> = Vec<Box<dyn SpecialCase<'a>>>;

pub fn expression_parser<'a>(controls: &'a ExpressionControls<'a>) -> Parser<'a,char,Expression> {
    raw_expression_parser(controls.raw_controls())
        .or(processed_expression_parser(controls))
}

fn processed_expression_parser<'a>(controls: &'a ExpressionControls<'a>) -> Parser<'a,char,Expression> {
    let expression_parser = lazy(|| expression_parser(controls));
    let iter = controls.special_cases().iter().map(move |case| case.parser(expression_parser.clone()));
    iter.reduce(|acc, next| acc.or(next))
        .unwrap_or(pred(|_| None))
}
