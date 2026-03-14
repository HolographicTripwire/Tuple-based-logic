use tbl_structures::{expressions::Expression, path_composites::{ExpressionInInference, OwnedExpressionInInference}};

#[derive(Clone)]
pub struct ExpressionLengthCheckError {
    pub expected_length: usize,
    pub expression: OwnedExpressionInInference
}
impl ExpressionLengthCheckError {
    pub fn get_actual_length(&self) -> Option<usize> { self.expression.obj().len() }
    pub fn into_expression(self) -> Expression { self.expression.into_obj_and_path().0 }
}

/// Check that the provided [Expression](ExpressionInInference) has an length equal to expected_length, returning an error otherwise
pub fn assert_expression_length<'a>(expr: &ExpressionInInference, expected_length: usize) -> Result<(), ExpressionLengthCheckError> {
    match expr.obj().as_slice() {
        Ok(tuple) => if tuple.len() == expected_length { Ok(()) }
        else { Err(ExpressionLengthCheckError {
            expected_length,
            expression: expr.clone().into_owned()
        }) },
        Err(()) => Err(ExpressionLengthCheckError {
            expected_length, 
            expression: expr.clone().into_owned()
        })
    }
}
