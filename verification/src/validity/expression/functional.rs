use itertools::Either;
use tbl_structures::path_composites::OwnedExpressionInInference;
use tbl_structures::{expressions::Expression, path_composites::ExpressionInInference};

use crate::validity::ExpressionLengthCheckError;
use crate::validity::ExpressionValueCheckError;
use crate::validity::expression_as_slice;
use crate::validity::assert_expression_value;

#[derive(Clone)]
pub enum UnwrapInvocationExpressionError {
    ExpressionAtomic(OwnedExpressionInInference),
    NoFirstElement(OwnedExpressionInInference),
    WrongHead(ExpressionValueCheckError)
}

/// Take an expression, and if it is in the form (expected_head, e1, e2, ..., en) return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_invocation_expression<'a>(invocation: &'a ExpressionInInference<'a>, expected_head: &Expression) -> Result<Vec<ExpressionInInference<'a>>,UnwrapInvocationExpressionError>{
    let vec = expression_as_slice(invocation)
        .map_err(|_| UnwrapInvocationExpressionError::ExpressionAtomic(invocation.clone().into_owned()))?;
    let (head, tail) = vec.split_first().ok_or_else(|| UnwrapInvocationExpressionError::NoFirstElement(invocation.clone().into_owned()))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| UnwrapInvocationExpressionError::WrongHead(e))?;
    Ok(tail.to_vec())
}

/// Take an expression, and if it is in the form (expected_head, e1, e2, ..., en) where n = N return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_fixed_length_invocation_expression<'a,const N: usize>(invocation: &'a ExpressionInInference<'a>, expected_head: &Expression) -> Result<Box<[ExpressionInInference<'a>; N]>,Either<UnwrapInvocationExpressionError,ExpressionLengthCheckError>>{
    let vec = expression_as_slice(invocation)
        .map_err(|_| Either::Left(UnwrapInvocationExpressionError::ExpressionAtomic(invocation.clone().into_owned())))?;
    let (head, tail) = vec.split_first().ok_or_else(|| Either::Left(UnwrapInvocationExpressionError::NoFirstElement(invocation.clone().into_owned())))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| Either::Left(UnwrapInvocationExpressionError::WrongHead(e)))?;
    Ok(tail.to_vec().try_into().map_err(|_| Either::Right(ExpressionLengthCheckError { expected_length: N, expression: invocation.clone().into_owned() }))?)
}
