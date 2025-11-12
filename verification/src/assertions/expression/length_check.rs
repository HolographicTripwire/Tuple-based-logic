use std::fmt::Display;

use tbl_structures::path_composites::{ExpressionInProof, OwnedExpressionInProof};

use crate::assertions::stringify_length;

pub struct ExpressionLengthCheckError {
    expected_length: usize,
    expression: OwnedExpressionInProof
}
impl ExpressionLengthCheckError {
    pub fn new(expected_length: usize, expression: OwnedExpressionInProof) -> Self
        { Self { expected_length, expression } }
    
}
impl Display for ExpressionLengthCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expression = self.expression.0.obj();
        write!(f,"Expression at {path} has wrong length (expected {length_expected}; found {length_actual})",
            path=self.expression.0.path(),
            length_expected=stringify_length(expression),
            length_actual=stringify_length(expression)
        )
    }
}

/// Check that the provided [Expression](OwnedExpressionInProof) has an length equal to expected_length, returning an error otherwise
pub fn assert_expression_length<'a,T: From<ExpressionLengthCheckError>>(expr: &ExpressionInProof, expected_length: usize) -> Result<(), T> {
    match expr.0.obj().as_slice() {
        Ok(tuple) => if tuple.len() == expected_length { Ok(()) }
        else {     Err(ExpressionLengthCheckError::new(expected_length, expr).into()) },
        Err(()) => Err(ExpressionLengthCheckError::new(expected_length, expr).into())
    }
}
