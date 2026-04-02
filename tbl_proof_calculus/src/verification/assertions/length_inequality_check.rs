use std::collections::HashSet;

use path_lib::obj_at_path::OwnedObjAtPath;

use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression, located::{OwnedTblExpressionAtPath, TblExpressionAtPath}};

pub struct ExpressionLengthInequalityError<C: CompoundTblExpression, Path> {
    pub expressions: Vec<OwnedObjAtPath<TblExpression<C>,Path>>
}
/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_expression_length_inequality<'a,C: CompoundTblExpression, Path>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>]) -> Result<(), ExpressionLengthInequalityError<C,Path>> {
    if exprs.len() == 0 { panic!("Cannot check length inequality for zero expressions") } 
    let iter = exprs.iter().map(|o| o.obj.len());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(ExpressionLengthInequalityError {
            expressions: exprs.into_iter().map(|x| (*x).clone().into()).collect()
        }); } }
    Ok(())
}

pub struct FixedLengthExpressionLengthInequalityError<const N: usize,C: CompoundTblExpression, Path> {
    pub expressions: [OwnedTblExpressionAtPath<C,Path>; N]
}
/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_fixed_length_expression_length_inequality<'a,const N: usize, C: CompoundTblExpression, Path>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>; N]) -> Result<(), FixedLengthExpressionLengthInequalityError<N,C,Path>> {
    if N == 0 { panic!("Cannot check length inequality for zero expressions") } 
    let iter = exprs.iter().map(|o| o.obj.len());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthExpressionLengthInequalityError {
            expressions: exprs.clone().map(|x| x.clone().into())
        }); } }
    Ok(())
}
