use std::marker::PhantomData;

use parsertools::parsers::Parser;
use tbl_structures::propositions::Expression;

use crate::structures::expressions::{patterns::{expr_pattern_matcher, ExprPattern}, SpecialCase};

#[derive(Clone)]
pub struct ExprPatternPair {
    left: ExprPattern,
    right: ExprPattern,
}
impl ExprPatternPair {
    pub fn new(left: ExprPattern, right: ExprPattern) -> Self
        { Self { left, right } }

    fn left_to_right<'a>(&'a self) -> Parser<'a,char,String> {
        expr_pattern_translator(&self.left, &self.right)
    }
    fn right_to_left<'a>(&'a self) -> Parser<'a,char,String> {
        expr_pattern_translator(&self.right, &self.left)
    }
}
impl <'a> SpecialCase<'a> for ExprPatternPair {
    fn parser(&'a self, expr_parser: Parser<'a,char,Expression>) -> Parser<'a,char,Expression> {
        self.right_to_left().clone()
            .split_map(move |s| expr_parser.parse_all(s.chars()))
    }
}

fn expr_pattern_translator<'a>(before: &'a ExprPattern, after: &'a ExprPattern) -> Parser<'a,char,String> {
    expr_pattern_matcher(before)
        .map(|assignments| after.assign(&assignments).unwrap().try_into().unwrap())
}
