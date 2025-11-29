use std::{collections::HashSet, sync::Arc};

use dyn_clone::DynClone;
use parsertools::{helpers::lazy, tokens::pred, Parser};
use tbl_structures::expressions::Expression;

use crate::{helpers::{parsers::controlled::ControlStrings, styles::Style}, structures::expressions::raw::RawExpressionStyle};

pub mod raw;
pub mod patterns;
pub mod functional;

#[derive(Clone)]
pub struct ExpressionStyle<'a> {
    raw_expr_style: RawExpressionStyle,
    special_cases: Arc<SpecialCases<'a>>
}
impl <'a> ExpressionStyle<'a> {
    pub fn new(raw_expr_style: RawExpressionStyle, special_cases: Arc<SpecialCases<'a>>) -> Self { Self { raw_expr_style, special_cases } }

    pub fn raw_expr_style(&self) -> &RawExpressionStyle { &self.raw_expr_style }
    pub fn special_cases(&self) -> Arc<SpecialCases<'a>> { self.special_cases.clone() }

    pub fn controls(&self) -> HashSet<&str> { HashSet::from_iter(self.raw_expr_style.controls())}

    fn apply_special_cases(&self, string: String) -> String {
        todo!()
        /*for case in self.special_cases.iter() {
            string = case.
        }*/
    }
}
impl <'a> Style<Expression> for ExpressionStyle<'a> {
    type ParseParams = ControlStrings;

    fn stringify(&self, expression: &Expression) -> String {
        let raw_expression = self.raw_expr_style.stringify(expression);
        self.apply_special_cases(raw_expression)
    }
    
    fn parser<'b>(&self, params: Self::ParseParams) -> Parser<'b,char,Expression> {
        todo!()
    }
    
}

pub trait SpecialCase<'a>: Sync + Send + DynClone {
    fn parser(&self, expr_parser: Parser<'a,char,Expression>) -> Parser<'a,char,Expression>;
}
impl <'a> Clone for Box<dyn SpecialCase<'a>>
    { fn clone(&self) -> Self { dyn_clone::clone_box(&**self) } }
pub type SpecialCases<'a> = Vec<Box<dyn SpecialCase<'a>>>;

pub fn expression_parser<'a>(style: ExpressionStyle<'a>) -> Parser<'a,char,Expression> {
    style.raw_expr_style().parser(())
        .or(processed_expression_parser(style))
}

fn processed_expression_parser<'a>(style: ExpressionStyle<'a>) -> Parser<'a,char,Expression> {
    let cloned_style = style.clone();
    let expression_parser = lazy(move || expression_parser(cloned_style.clone()));
    let binding = style.special_cases();
    let iter = binding.iter().map(move |case| case.parser(expression_parser.clone()));
    iter.reduce(|acc, next| acc.or(next))
        .unwrap_or(pred(|_| None))
}

#[cfg(test)]
mod tests {
    
}
