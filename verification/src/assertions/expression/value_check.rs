use tbl_structures::{expressions::Expression, path_composites::{ExpressionInProof, OwnedExpressionInProof}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct ExpressionValueCheckError {
    expected_value: Expression,
    expression: OwnedExpressionInProof,
}
impl ExpressionValueCheckError {
    pub fn new(expected_value: Expression, expression: OwnedExpressionInProof) -> Self
        { Self { expected_value, expression } }
}

pub fn format_expression_value_check_error(err: ExpressionValueCheckError, style: ExpressionStyle) -> String {
    format!("Expression at {path} has wrong value (expected {value_expected}; found {value_actual})",
        path=err.expression.0.path(),
        value_expected=style.stringify(&err.expected_value),
        value_actual=style.stringify(err.expression.0.obj())
    )
}

/// Check that the provided [Expression](OwnedExpressionInProof) has an value equal to expected_value, returning an error otherwise
pub fn assert_expression_value<'a>(expr: &ExpressionInProof, expected_value: &Expression) -> Result<(), ExpressionValueCheckError> {
    if expr.0.obj() == expected_value { Ok(()) }
    else { Err(ExpressionValueCheckError::new(
        expected_value.clone(),
        expr.clone().into_owned()
    )) }
}
