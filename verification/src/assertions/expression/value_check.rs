use tbl_structures::{expressions::Expression, path_composites::{ExpressionInInference, OwnedExpressionInInference}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct ExpressionValueCheckError {
    pub expected_value: Expression,
    pub expression: OwnedExpressionInInference,
}

pub fn format_expression_value_check_error(err: ExpressionValueCheckError, style: ExpressionStyle) -> String {
    format!("Expression at {path} has wrong value (expected {value_expected}; found {value_actual})",
        path=err.expression.0.path(),
        value_expected=style.stringify(&err.expected_value),
        value_actual=style.stringify(err.expression.0.obj())
    )
}

/// Check that the provided [Expression](ExpressionInInference) has an value equal to expected_value, returning an error otherwise
pub fn assert_expression_value<'a>(expr: &ExpressionInInference, expected_value: &Expression) -> Result<(), ExpressionValueCheckError> {
    if expr.0.obj() == expected_value { Ok(()) }
    else { Err(ExpressionValueCheckError{
        expected_value: expected_value.clone(),
        expression: expr.clone().into_owned()
    }) }
}
