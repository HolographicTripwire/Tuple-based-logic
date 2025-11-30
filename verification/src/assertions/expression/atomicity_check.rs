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

pub fn format_expression_atomicity_check_error(err: ExpressionAtomicityCheckError) -> String {
    format!("Expression at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
        path=err.expression.0.path(),
        atomicity_expected=stringify_atomicity(err.expected_atomicity),
        atomicity_actual=stringify_atomicity(err.expression.0.obj().as_atom().is_ok())
    )
}

/// Check that the provided [Expression](OwnedExpressionInProof) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_expression_atomicity<'a,T: From<ExpressionAtomicityCheckError>>(expr: &ExpressionInProof, expected_atomicity: bool) -> Result<(), T> {
    if expr.0.obj().as_atom().is_ok() == expected_atomicity { Ok(()) }
    else { Err(ExpressionAtomicityCheckError::new(
        expected_atomicity, 
        expr.clone().into_owned() 
    ).into()) }
}
