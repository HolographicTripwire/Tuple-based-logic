use std::collections::HashSet;

use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

use crate::validity::utils::stringify_length;

pub struct ExpressionLengthInequalityError {
    pub expressions: Vec<OwnedExpressionInInference>
}

pub fn format_expression_length_inequality_error(err: ExpressionLengthInequalityError) -> String {
    format!("Expression lengths expected to all be inequal, but weren't; {lengths}",
        lengths = err.expressions.iter().map(|o|
            o.path().to_string()
            + " -> " +
            &stringify_length(o.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}


/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_expression_length_inequality<'a>(exprs: &[&'a ExpressionInInference<'a>]) -> Result<(), ExpressionLengthInequalityError> {
    if exprs.len() == 0 { panic!("Cannot check length inequality for zero expressions") } 
    let iter = exprs.iter().map(|o| o.obj().len());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(ExpressionLengthInequalityError {
            expressions: exprs.into_iter().map(|x| (*x).clone().into_owned()).collect()
        }); } }
    Ok(())
}





pub struct FixedLengthExpressionLengthInequalityError<const N: usize> {
    pub expressions: [OwnedExpressionInInference; N]
}
pub fn format_fixed_length_expression_length_inequality_error<const N: usize>(err: FixedLengthExpressionLengthInequalityError<N>) -> String {
    format!("Expression lengths expected to all be equal, but weren't; {atomicities}",
        atomicities = err.expressions.iter().map(|o|
            o.path().to_string()
            + " -> " +
            &stringify_length(o.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}
/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_fixed_length_expression_length_inequality<'a,const N: usize>(exprs: &[&'a ExpressionInInference<'a>; N]) -> Result<(), FixedLengthExpressionLengthInequalityError<N>> {
    if N == 0 { panic!("Cannot check length inequality for zero expressions") } 
    let iter = exprs.iter().map(|o| o.obj().len());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthExpressionLengthInequalityError {
            expressions: exprs.clone().map(|x| x.clone().into_owned())
        }); } }
    Ok(())
}
