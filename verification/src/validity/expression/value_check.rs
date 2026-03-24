use tbl_structures::{expressions::TblExpression, path_composites::{ExpressionInInference, OwnedExpressionInInference}};

#[derive(Clone)]
pub struct ExpressionValueCheckError {
    pub expected_value: TblExpression,
    pub expression: OwnedExpressionInInference,
}
impl ExpressionValueCheckError {
    pub fn get_actual_value(&self) -> &TblExpression { &self.expression.obj }
    pub fn into_expression(self) -> TblExpression { self.expression.obj }
}

/// Check that the provided [Expression](ExpressionInInference) has an value equal to expected_value, returning an error otherwise
pub fn assert_expression_value<'a>(expr: &ExpressionInInference, expected_value: &TblExpression) -> Result<(), ExpressionValueCheckError> {
    if expr.obj == expected_value { Ok(()) }
    else { Err(ExpressionValueCheckError{
        expected_value: expected_value.clone(),
        expression: expr.clone().into()
    }) }
}
