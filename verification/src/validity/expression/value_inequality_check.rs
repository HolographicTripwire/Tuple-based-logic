use std::collections::HashSet;

use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

pub struct ExpressionValueInequalityError {
    pub expressions: Vec<OwnedExpressionInInference>,
}
/// Check that the provided [Propositions](PropositionInProofStep) have inequal value, returning an error otherwise
pub fn assert_expression_value_inequality<'a>(exprs: &[&'a ExpressionInInference<'a>]) -> Result<(), ExpressionValueInequalityError> {
    let iter = exprs.iter().map(|o| o.obj());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(ExpressionValueInequalityError{
            expressions: exprs.into_iter().map(|x| (*x).clone().into_owned()).collect()
        }); } }
    Ok(())
}

pub struct FixedLengthExpressionValueInequalityError<const N: usize> {
    pub expressions: [OwnedExpressionInInference; N]
}
/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_fixed_length_expression_value_inequality<'a,const N: usize>(exprs: &[&'a ExpressionInInference<'a>; N]) -> Result<(), FixedLengthExpressionValueInequalityError<N>> {
    if N == 0 { panic!("Cannot check length inequality for zero expressions") } 
    let iter = exprs.iter().map(|o| o.obj());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthExpressionValueInequalityError {
            expressions: exprs.clone().map(|x| (*x).clone().into_owned())
        }); } }
    Ok(())
}
