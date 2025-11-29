use std::marker::PhantomData;

use tbl_structures::inference::InferenceRule;

use crate::structures::expressions::ExpressionStyle;

#[derive(Clone)]
pub struct InferenceStyle<'a,Rule: InferenceRule> {
    expression_style: ExpressionStyle<'a>,
    rule: PhantomData<Rule>
}

impl <'a,Rule: InferenceRule> InferenceStyle<'a,Rule> {
    pub fn expression_style(&self) -> &ExpressionStyle<'a> { &self.expression_style }
}
