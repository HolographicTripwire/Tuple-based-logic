use tbl_structures::{expressions::Expression, path_composites::{ExpressionInInference, OwnedExpressionInInference}};

#[derive(Clone)]
pub struct ExpressionAtomicityCheckError {
    pub expected_atomicity: bool,
    pub expression: OwnedExpressionInInference
}
impl ExpressionAtomicityCheckError {
    pub fn get_actual_atomicity(&self) -> bool { self.expression.obj().len().is_none() }
    pub fn into_expression(self) -> Expression { self.expression.into_obj_and_path().0 }
}

/// Check that the provided [Expression](ExpressionInInference) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_expression_atomicity<'a>(expr: &ExpressionInInference, expected_atomicity: bool) -> Result<(), ExpressionAtomicityCheckError> {
    if expr.obj().as_atom().is_ok() == expected_atomicity { Ok(()) }
    else { Err(ExpressionAtomicityCheckError{
        expected_atomicity, 
        expression: expr.clone().into_owned() 
    }) }
}
