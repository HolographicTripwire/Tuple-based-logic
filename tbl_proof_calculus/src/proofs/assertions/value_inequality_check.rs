use std::collections::HashSet;

use crate::expressions::types::assigned::{compound::CompoundTblExpression, OwnedTblExpressionAtPath, TblExpressionAtPath};

pub struct ExpressionValueInequalityError<C: CompoundTblExpression, Path> {
    pub expressions: Box<[OwnedTblExpressionAtPath<C,Path>]>,
}
/// Check that the provided [Propositions](PropositionInProofStep) have inequal value, returning an error otherwise
pub fn assert_expression_value_inequality<'a,C: CompoundTblExpression,Path:Clone>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>]) -> Result<(), ExpressionValueInequalityError<C,Path>> {
    let iter = exprs.iter().map(|o| o.obj);
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(ExpressionValueInequalityError{
            expressions: exprs.into_iter().map(|x| (*x).clone().into()).collect()
        }); } }
    Ok(())
}

pub struct FixedLengthExpressionValueInequalityError<const N: usize,C: CompoundTblExpression,Path> {
    pub expressions: [OwnedTblExpressionAtPath<C,Path>; N]
}
/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_fixed_length_expression_value_inequality<'a,const N: usize,C:CompoundTblExpression,Path:Clone>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>; N]) -> Result<(), FixedLengthExpressionValueInequalityError<N,C,Path>> {
    if N == 0 { panic!("Cannot check length inequality for zero expressions") } 
    let iter = exprs.iter().map(|o| o.obj);
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthExpressionValueInequalityError {
            expressions: exprs.clone().map(|x| (*x).clone().into())
        }); } }
    Ok(())
}
