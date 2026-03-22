
use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

pub struct ExpressionAtomicityInequalityError {
    pub expr1: OwnedExpressionInInference,
    pub expr2: OwnedExpressionInInference,
}

/// Check that the provided [Expressions](ExpressionInInference) have inequal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_inequality<'a>(expr1: &ExpressionInInference, expr2: &ExpressionInInference) -> Result<(), ExpressionAtomicityInequalityError> {
    let first_atomicity = expr1.obj.as_atom().is_ok();
    let second_atomicity = expr2.obj.as_atom().is_ok();
    if first_atomicity == second_atomicity { Ok(()) }
    else { Err(ExpressionAtomicityInequalityError{
        expr1: expr1.clone().into(), 
        expr2: expr2.clone().into()
    }) }
}
