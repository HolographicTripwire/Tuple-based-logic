use itertools::Either;
use tbl_structures::{expressions::Expression, path_composites::ExpressionInInference};

use crate::assertions::ExpressionAtomicityCheckError;
use crate::assertions::ExpressionLengthCheckError;
use crate::assertions::ExpressionValueCheckError;
use crate::assertions::expression_as_slice;
use crate::assertions::assert_expression_value;

pub enum UnwrapInvocationExpressionError {
    WrongAtomicity(ExpressionAtomicityCheckError),
    NoFirstElement(Expression),
    WrongHead(ExpressionValueCheckError)
}

/// Take an expression, and if it is in the form (expected_head, e1, e2, ..., en) return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_invocation_expression<'a>(invocation: &'a ExpressionInInference<'a>, expected_head: &Expression) -> Result<Vec<ExpressionInInference<'a>>,UnwrapInvocationExpressionError>{
    let vec = expression_as_slice(invocation)
        .map_err(|e| UnwrapInvocationExpressionError::WrongAtomicity(e))?;
    let (head, tail) = vec.split_first().ok_or_else(|| UnwrapInvocationExpressionError::NoFirstElement(invocation.0.obj().clone()))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| UnwrapInvocationExpressionError::WrongHead(e))?;
    Ok(tail.to_vec())
}

/// Take an expression, and if it is in the form (expected_head, e1, e2, ..., en) where n = N return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_fixed_length_invocation_expression<'a,const N: usize>(invocation: &'a ExpressionInInference<'a>, expected_head: &Expression) -> Result<Box<[ExpressionInInference<'a>; N]>,Either<UnwrapInvocationExpressionError,ExpressionLengthCheckError>>{
    let vec = expression_as_slice(invocation)
        .map_err(|e| Either::Left(UnwrapInvocationExpressionError::WrongAtomicity(e)))?;
    let (head, tail) = vec.split_first().ok_or_else(|| Either::Left(UnwrapInvocationExpressionError::NoFirstElement(invocation.0.obj().clone())))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| Either::Left(UnwrapInvocationExpressionError::WrongHead(e)))?;
    Ok(tail.to_vec().try_into().map_err(|_| Either::Right(ExpressionLengthCheckError { expected_length: N, expression: invocation.clone().into_owned() }))?)
}
