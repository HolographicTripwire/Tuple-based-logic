use itertools::Either;
use tbl_structures::path_composites::ExpressionInInference;
use tbl_structures::proof::PropositionInInference;
use tbl_structures::{expressions::Proposition};

use crate::assertions::{ExpressionValueCheckError, PropositionAtomicityCheckError, assert_expression_value};
use crate::assertions::PropositionLengthCheckError;
use crate::assertions::proposition_as_slice;

pub enum UnwrapInvocationPropositionError {
    WrongAtomicity(PropositionAtomicityCheckError),
    NoFirstElement(Proposition),
    WrongHead(ExpressionValueCheckError)
}

/// Take an proposition, and if it is in the form (expected_head, e1, e2, ..., en) return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_invocation_proposition<'a>(invocation: &'a PropositionInInference<'a>, expected_head: &Proposition) -> Result<Vec<ExpressionInInference<'a>>,UnwrapInvocationPropositionError>{
    let vec = proposition_as_slice(invocation)
        .map_err(|e| UnwrapInvocationPropositionError::WrongAtomicity(e))?;
    let (head, tail) = vec.split_first().ok_or_else(|| UnwrapInvocationPropositionError::NoFirstElement(invocation.0.obj().clone()))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| UnwrapInvocationPropositionError::WrongHead(e))?;
    Ok(tail.to_vec())
}

/// Take an proposition, and if it is in the form (expected_head, e1, e2, ..., en) where n = N return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_fixed_length_invocation_proposition<'a,const N: usize>(invocation: &'a PropositionInInference<'a>, expected_head: &Proposition) -> Result<Box<[ExpressionInInference<'a>; N]>,Either<UnwrapInvocationPropositionError,PropositionLengthCheckError>>{
    let vec = proposition_as_slice(invocation)
        .map_err(|e| Either::Left(UnwrapInvocationPropositionError::WrongAtomicity(e)))?;
    let (head, tail) = vec.split_first().ok_or_else(|| Either::Left(UnwrapInvocationPropositionError::NoFirstElement(invocation.0.obj().clone())))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| Either::Left(UnwrapInvocationPropositionError::WrongHead(e)))?;
    Ok(tail.to_vec().try_into().map_err(|_| Either::Right(PropositionLengthCheckError { expected_length: N, proposition: invocation.clone().into_owned() }))?)
}
