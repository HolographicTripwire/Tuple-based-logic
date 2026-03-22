use std::collections::HashSet;

use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

pub struct ExpressionLengthInequalityError {
    pub expressions: Vec<OwnedExpressionInInference>
}
/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_expression_length_inequality<'a>(exprs: &[&'a ExpressionInInference<'a>]) -> Result<(), ExpressionLengthInequalityError> {
    if exprs.len() == 0 { panic!("Cannot check length inequality for zero expressions") } 
    let iter = exprs.iter().map(|o| o.obj.len());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(ExpressionLengthInequalityError {
            expressions: exprs.into_iter().map(|x| (*x).clone().into()).collect()
        }); } }
    Ok(())
}

pub struct FixedLengthExpressionLengthInequalityError<const N: usize> {
    pub expressions: [OwnedExpressionInInference; N]
}
/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_fixed_length_expression_length_inequality<'a,const N: usize>(exprs: &[&'a ExpressionInInference<'a>; N]) -> Result<(), FixedLengthExpressionLengthInequalityError<N>> {
    if N == 0 { panic!("Cannot check length inequality for zero expressions") } 
    let iter = exprs.iter().map(|o| o.obj.len());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthExpressionLengthInequalityError {
            expressions: exprs.clone().map(|x| x.clone().into())
        }); } }
    Ok(())
}
