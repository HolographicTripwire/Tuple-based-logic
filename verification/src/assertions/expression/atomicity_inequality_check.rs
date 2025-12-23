
use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

pub struct ExpressionAtomicityInequalityError {
    pub expr1: OwnedExpressionInInference,
    pub expr2: OwnedExpressionInInference,
}

pub fn format_expression_atomicity_inequality_error(err: ExpressionAtomicityInequalityError) -> String {
    format!("Atomicity of expressions {expr1} and {expr2} expected to be inequal, but both were {value}",
            expr1 = err.expr1.0.path(),
            expr2 = err.expr2.0.path(),
            value = err.expr1.0.obj().as_atom().is_ok()
        )
}

/// Check that the provided [Expressions](ExpressionInInference) have inequal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_inequality<'a>(expr1: &ExpressionInInference, expr2: &ExpressionInInference) -> Result<(), ExpressionAtomicityInequalityError> {
    let first_atomicity = expr1.0.obj().as_atom().is_ok();
    let second_atomicity = expr2.0.obj().as_atom().is_ok();
    if first_atomicity == second_atomicity { Ok(()) }
    else { Err(ExpressionAtomicityInequalityError{
        expr1: expr1.clone().into_owned(), 
        expr2: expr2.clone().into_owned()
    }) }
}
