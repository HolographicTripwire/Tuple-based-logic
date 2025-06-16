use std::marker::PhantomData;

use parsertools::parsers::Parser;
use tbl_structures::propositions::Expression;

use crate::structures::expressions::{patterns::{expr_pattern_matcher, ExprPattern}, SpecialCase};

pub struct ExprPatternPair<'a> {
    left: ExprPattern,
    right: ExprPattern,
    phantom: PhantomData<&'a ()>
}
impl <'a> ExprPatternPair<'a> {
    fn left_to_right(&'a self) -> Parser<'a,char,String> {
        expr_pattern_translator(&self.left, &self.right)
    }
    fn right_to_left(&'a self) -> Parser<'a,char,String> {
        expr_pattern_translator(&self.right, &self.left)
    }
    pub fn special_case(&'a self) -> impl SpecialCase<'a> {
        move |expr_parser: Parser<'a, char,Expression>|
            self.left_to_right().clone()
                .split_map(move |s| expr_parser.parse_all(s.chars()) 
        )
    }
    
}

fn expr_pattern_translator<'a>(before: &'a ExprPattern, after: &'a ExprPattern) -> Parser<'a,char,String> {
    expr_pattern_matcher(before)
        .map(|assignments| after.assign(&assignments).unwrap().try_into().unwrap())
}
