use std::marker::PhantomData;

use parsertools::parsers::{helpers::lazy, tokens::pred, Parser};
use tbl_structures::propositions::Expression;

use crate::structures::expressions::raw::{raw_expression_parser, RawExpressionControls};

pub mod raw;
pub mod patterns;

pub struct ExpressionControls<'a,S: SpecialCase<'a>> {
    raw_controls: RawExpressionControls,
    special_cases: Vec<S>,
    phantom: PhantomData<&'a ()>
}
impl <'a, S: SpecialCase<'a>> ExpressionControls<'a,S> {
    pub fn raw_controls(&self) -> &RawExpressionControls { &self.raw_controls }
    pub fn special_cases(&self) -> &Vec<S> { &self.special_cases }
}

pub trait SpecialCase<'a>: Sync + Send + Fn(Parser<'a,char,Expression>) -> Parser<'a,char,Expression> {}
impl <'a, S: Sync + Send + Fn(Parser<'a,char,Expression>) -> Parser<'a,char,Expression>> SpecialCase<'a> for S {}

pub fn expression_parser<'a,S: SpecialCase<'a>>(controls: &'a ExpressionControls<'a,S>) -> Parser<'a,char,Expression> {
    raw_expression_parser(controls.raw_controls())
        .or(processed_expression_parser(controls))
}

fn processed_expression_parser<'a,S: SpecialCase<'a>>(controls: &'a ExpressionControls<'a,S>) -> Parser<'a,char,Expression> {
    let expression_parser = lazy(|| expression_parser(controls));
    let iter = controls.special_cases().iter().map(move |case| (case)(expression_parser.clone()));
    iter.reduce(|acc, next| acc.or(next))
        .unwrap_or(pred(|_| None))
}
