use itertools::Either;
use path_lib::obj_at_path::ObjAtPath;

use crate::{structures::expressions::{TblExpression, compound::CompoundTblExpression, located::{OwnedTblExpressionAtPath, TblExpressionAtPath}}, verification::assertions::{ExpressionLengthCheckError, ExpressionValueCheckError, assert_expression_value, expression_as_slice}};

#[derive(Clone)]
pub enum UnwrapInvocationExpressionError<C1: CompoundTblExpression, Path, C2: CompoundTblExpression> {
    ExpressionAtomic(OwnedTblExpressionAtPath<C1, Path>),
    NoFirstElement(OwnedTblExpressionAtPath<C1, Path>),
    WrongHead(ExpressionValueCheckError<C1, Path, C2>)
}

/// Take an expression, and if it is in the form (expected_head, e1, e2, ..., en) return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_invocation_expression<'a,C1: CompoundTblExpression, Path, C2: CompoundTblExpression>(invocation: &'a ObjAtPath<'a,TblExpression<C1>, Path>, expected_head: &TblExpression<C2>) -> Result<Box<[ObjAtPath<'a,TblExpression<C1>,Path>]>,UnwrapInvocationExpressionError<C1,Path,C2>>{
    let vec = expression_as_slice(invocation)
        .map_err(|e| UnwrapInvocationExpressionError::ExpressionAtomic(e.expression))?;
    let (head, tail) = vec.split_first().ok_or_else(|| UnwrapInvocationExpressionError::NoFirstElement(invocation.clone().into()))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| UnwrapInvocationExpressionError::WrongHead(e))?;
    Ok(tail.into())
}

/// Take an expression, and if it is in the form (expected_head, e1, e2, ..., en) where n = N return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_fixed_length_invocation_expression<'a,const N: usize,C1:CompoundTblExpression,Path,C2:CompoundTblExpression>(invocation: &'a TblExpressionAtPath<'a,C1,Path>, expected_head: &TblExpression<C2>) -> Result<Box<[TblExpressionAtPath<'a,C1,Path>; N]>,Either<UnwrapInvocationExpressionError<C1,Path,C2>,ExpressionLengthCheckError<C1,Path>>>{
    let vec = expression_as_slice(invocation)
        .map_err(|e| Either::Left(UnwrapInvocationExpressionError::ExpressionAtomic(e.expression)))?;
    let (head, tail) = vec.split_first().ok_or_else(|| Either::Left(UnwrapInvocationExpressionError::NoFirstElement(invocation.clone().into())))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| Either::Left(UnwrapInvocationExpressionError::WrongHead(e)))?;
    Ok(tail.to_vec().try_into().map_err(|_| Either::Right(ExpressionLengthCheckError { expected_length: N, expression: invocation.clone().into() }))?)
}
