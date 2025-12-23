use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

use crate::assertions::utils::stringify_atomicity;

pub struct ExpressionAtomicityCheckError {
    pub expected_atomicity: bool,
    pub expression: OwnedExpressionInInference
}
impl ExpressionAtomicityCheckError {
    pub fn get_actual_atomicity(&self) -> bool { self.expression.0.obj().len().is_none() }
}

pub fn format_expression_atomicity_check_error(err: ExpressionAtomicityCheckError) -> String {
    format!("Expression at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
        path=err.expression.0.path(),
        atomicity_expected=stringify_atomicity(err.expected_atomicity),
        atomicity_actual=stringify_atomicity(err.expression.0.obj().as_atom().is_ok())
    )
}

/// Check that the provided [Expression](ExpressionInInference) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_expression_atomicity<'a>(expr: &ExpressionInInference, expected_atomicity: bool) -> Result<(), ExpressionAtomicityCheckError> {
    if expr.0.obj().as_atom().is_ok() == expected_atomicity { Ok(()) }
    else { Err(ExpressionAtomicityCheckError{
        expected_atomicity, 
        expression: expr.clone().into_owned() 
    }) }
}
