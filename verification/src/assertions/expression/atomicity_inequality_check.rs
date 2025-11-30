
use tbl_structures::path_composites::{ExpressionInProof, OwnedExpressionInProof};

pub struct ExpressionAtomicityInequalityError {
    expr1: OwnedExpressionInProof,
    expr2: OwnedExpressionInProof,
}
impl ExpressionAtomicityInequalityError {
    pub fn new(expr1: OwnedExpressionInProof, expr2: OwnedExpressionInProof) -> Self
        { Self { expr1, expr2 } }
}

pub fn format_expression_atomicity_inequality_error(err: ExpressionAtomicityInequalityError) -> String {
    format!("Atomicity of expressions {expr1} and {expr2} expected to be inequal, but both were {value}",
            expr1 = err.expr1.0.path(),
            expr2 = err.expr2.0.path(),
            value = err.expr1.0.obj().as_atom().is_ok()
        )
}

/// Check that the provided [Expressions](OwnedExpressionInProof) have inequal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_inequality<'a>(expr1: &ExpressionInProof, expr2: &ExpressionInProof) -> Result<(), ExpressionAtomicityInequalityError> {
    let first_atomicity = expr1.0.obj().as_atom().is_ok();
    let second_atomicity = expr2.0.obj().as_atom().is_ok();
    if first_atomicity == second_atomicity { Ok(()) }
    else { Err(ExpressionAtomicityInequalityError::new(expr1.clone().into_owned(), expr2.clone().into_owned()).into()) }
}
