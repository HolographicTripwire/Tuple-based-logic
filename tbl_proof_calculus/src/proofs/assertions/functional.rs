use itertools::Either;
use path_lib::obj_at_path::ObjAtPath;

use crate::{expressions::{paths::immediate::ImmediateTblSubexpressionInExpressionPath, types::assigned::{OwnedTblExpressionAtPath, TblExpression, TblExpressionAtPath, compound::CompoundTblExpression}}, proofs::assertions::{ExpressionLengthCheckError, ExpressionValueCheckError, assert_expression_value, expression_as_slice}};

#[derive(Clone)]
pub enum UnwrapInvocationExpressionError<C1: CompoundTblExpression, Path, Path2, C2: CompoundTblExpression> {
    ExpressionAtomic(OwnedTblExpressionAtPath<C1, Path>),
    NoFirstElement(OwnedTblExpressionAtPath<C1, Path>),
    WrongHead(ExpressionValueCheckError<C1, Path2, C2>)
}

/// Take an expression, and if it is in the form (expected_head, e1, e2, ..., en) return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_invocation_expression<'a,C1, Path, Path2, C2>(invocation: &'a ObjAtPath<'a,TblExpression<C1>, Path>, expected_head: &TblExpression<C2>) -> Result<Box<[ObjAtPath<'a,TblExpression<C1>,Path2>]>,UnwrapInvocationExpressionError<C1,Path,Path2,C2>> where
C1: CompoundTblExpression + PartialEq<C2>,
C2: CompoundTblExpression + PartialEq<C1>,
Path: Clone,
Path2: Clone + From<(Path,ImmediateTblSubexpressionInExpressionPath)> {
    let vec: Box<[ObjAtPath<'_, TblExpression<C1>, Path2>]> = expression_as_slice(invocation)
        .map_err(|e| UnwrapInvocationExpressionError::ExpressionAtomic(e.expression))?;
    let (head, tail) = vec.split_first().ok_or(UnwrapInvocationExpressionError::NoFirstElement(invocation.clone().into()))?;
    assert_expression_value(head, expected_head)
        .map_err(|e| UnwrapInvocationExpressionError::WrongHead(e))?;
    Ok(tail.into())
}

/// Take an expression, and if it is in the form (expected_head, e1, e2, ..., en) where n = N return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_fixed_length_invocation_expression<'a,const N: usize,C1,Path,Path2,C2>(invocation: &'a TblExpressionAtPath<'a,C1,Path>, expected_head: &TblExpression<C2>) -> Result<Box<[TblExpressionAtPath<'a,C1,Path2>; N]>,Either<UnwrapInvocationExpressionError<C1,Path,Path2,C2>,ExpressionLengthCheckError<C1,Path>>> where
C1: CompoundTblExpression + PartialEq<C2>,
C2: CompoundTblExpression + PartialEq<C1>,
Path: Clone,
Path2: Clone + From<(Path,ImmediateTblSubexpressionInExpressionPath)> {
    let tail = unwrap_invocation_expression(invocation, expected_head)
        .map_err(|err| Either::Left(err))?;
    // let vec = expression_as_slice(invocation)
    //     .map_err(|e| Either::Left(UnwrapInvocationExpressionError::ExpressionAtomic(e.expression)))?;
    // let (head, tail) = vec.split_first().ok_or_else(|| Either::Left(UnwrapInvocationExpressionError::NoFirstElement(invocation.clone().into())))?;
    // assert_expression_value(head, expected_head)
    //     .map_err(|e| Either::Left(UnwrapInvocationExpressionError::WrongHead(e)))?;
    let y: Vec<TblExpressionAtPath<'_,C1,Path2>> = tail.to_vec();
    let x: Box<[TblExpressionAtPath<'a,C1,Path2>; N]> = match y.try_into() {
        Ok(b) => b,
        Err(_) => return Err(Either::Right(ExpressionLengthCheckError { expected_length: N, expression: invocation.clone().into() })),
    };
    Ok(x)
}
