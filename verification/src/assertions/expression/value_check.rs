use std::fmt::Display;

use tbl_structures::{expressions::Expression, path_composites::{ExpressionInProof, OwnedExpressionInProof}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct ExpressionValueCheckError<'a> {
    expected_value: Expression,
    expression: OwnedExpressionInProof,
    expression_style: ExpressionStyle<'a>
}
impl <'a> ExpressionValueCheckError<'a> {
    pub fn new(expected_value: Expression, expression: OwnedExpressionInProof, style: ExpressionStyle<'a>) -> Self
        { Self { expected_value, expression, expression_style: style } }
    
}
impl <'a> Display for ExpressionValueCheckError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Expression at {path} has wrong value (expected {value_expected}; found {value_actual})",
            path=self.expression.0.path(),
            value_expected=self.expression_style.stringify(&self.expected_value),
            value_actual=self.expression_style.stringify(self.expression.0.obj())
        )
    }
}

/// Check that the provided [Expression](OwnedExpressionInProof) has an value equal to expected_value, returning an error otherwise
pub fn assert_expression_value<'a>(expr: &ExpressionInProof, expected_value: &Expression, style: ExpressionStyle<'a>) -> Result<(), ExpressionValueCheckError<'a>> {
    if expr.0.obj() == expected_value { Ok(()) }
    else { Err(ExpressionValueCheckError::new(
        expected_value.clone(),
        expr.into_owned(),
        style
    )) }
}
