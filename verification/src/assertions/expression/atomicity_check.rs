use std::fmt::Display;

use tbl_structures::path_composites::{ExpressionInProof, OwnedExpressionInProof};

use crate::assertions::utils::stringify_atomicity;

pub struct ExpressionAtomicityCheckError {
    expected_atomicity: bool,
    expression: OwnedExpressionInProof
}
impl ExpressionAtomicityCheckError {
    pub fn new(expected_atomicity: bool, expression: OwnedExpressionInProof) -> Self
        { Self { expected_atomicity, expression } }
    
}
impl Display for ExpressionAtomicityCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Expression at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
            path=self.expression.0.path(),
            atomicity_expected=stringify_atomicity(self.expected_atomicity),
            atomicity_actual=stringify_atomicity(self.expression.0.obj().as_atom().is_ok())
        )
    }
}

/// Check that the provided [Expression](OwnedExpressionInProof) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_expression_atomicity<'a,T: From<ExpressionAtomicityCheckError>>(expr: &ExpressionInProof, expected_atomicity: bool) -> Result<(), T> {
    if expr.0.obj().as_atom().is_ok() == expected_atomicity { Ok(()) }
    else { Err(ExpressionAtomicityCheckError::new(
        expected_atomicity, 
        expr.into_owned() 
    ).into()) }
}
