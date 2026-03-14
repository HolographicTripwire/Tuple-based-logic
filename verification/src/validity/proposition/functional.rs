use itertools::Either;
use tbl_structures::expressions::Expression;
use tbl_structures::path_composites::ExpressionInInference;
use tbl_structures::proof::{OwnedPropositionInProofStep, PropositionInProofStep};

use crate::validity::{ExpressionValueCheckError, assert_expression_value};
use crate::validity::PropositionLengthCheckError;
use crate::validity::proposition_as_slice;

#[derive(Clone)]
pub enum UnwrapInvocationPropositionError {
    PropositionAtomic(OwnedPropositionInProofStep),
    NoFirstElement(OwnedPropositionInProofStep),
    WrongHead(ExpressionValueCheckError)
}

/// Take an proposition, and if it is in the form (expected_head, e1, e2, ..., en) return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_invocation_proposition<'a>(invocation: &'a PropositionInProofStep<'a>, expected_head: &Expression) -> Result<Vec<ExpressionInInference<'a>>,UnwrapInvocationPropositionError>{
    let vec = proposition_as_slice(invocation)
        .map_err(|_| UnwrapInvocationPropositionError::PropositionAtomic(invocation.clone().into_owned()))?;
    let (head, tail) = vec.split_first().ok_or_else(|| UnwrapInvocationPropositionError::NoFirstElement(invocation.clone().into_owned()))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| UnwrapInvocationPropositionError::WrongHead(e))?;
    Ok(tail.to_vec())
}

/// Take an proposition, and if it is in the form (expected_head, e1, e2, ..., en) where n = N return [e1, e2, ..., en], otherwise return an Error
pub fn unwrap_fixed_length_invocation_proposition<'a,const N: usize>(invocation: &'a PropositionInProofStep<'a>, expected_head: &Expression) -> Result<Box<[ExpressionInInference<'a>; N]>,Either<UnwrapInvocationPropositionError,PropositionLengthCheckError>>{
    let vec = proposition_as_slice(invocation)
        .map_err(|_| Either::Left(UnwrapInvocationPropositionError::PropositionAtomic(invocation.clone().into_owned())))?;
    let (head, tail) = vec.split_first().ok_or_else(|| Either::Left(UnwrapInvocationPropositionError::NoFirstElement(invocation.clone().into_owned())))?;
    assert_expression_value(head, &expected_head)
        .map_err(|e| Either::Left(UnwrapInvocationPropositionError::WrongHead(e)))?;
    Ok(tail.to_vec().try_into().map_err(|_| Either::Right(PropositionLengthCheckError { expected_length: N, proposition: invocation.clone().into_owned() }))?)
}
