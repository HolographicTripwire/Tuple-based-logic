use tbl_structures::{expressions::Expression, path_composites::{ExpressionInInference, OwnedExpressionInInference}};

#[derive(Clone)]
pub struct ExpressionValueCheckError {
    pub expected_value: Expression,
    pub expression: OwnedExpressionInInference,
}
impl ExpressionValueCheckError {
    pub fn get_actual_value(&self) -> &Expression { self.expression.obj() }
    pub fn into_expression(self) -> Expression { self.expression.into_obj_and_path().0 }
}

/// Check that the provided [Expression](ExpressionInInference) has an value equal to expected_value, returning an error otherwise
pub fn assert_expression_value<'a>(expr: &ExpressionInInference, expected_value: &Expression) -> Result<(), ExpressionValueCheckError> {
    if expr.obj() == expected_value { Ok(()) }
    else { Err(ExpressionValueCheckError{
        expected_value: expected_value.clone(),
        expression: expr.clone().into_owned()
    }) }
}
