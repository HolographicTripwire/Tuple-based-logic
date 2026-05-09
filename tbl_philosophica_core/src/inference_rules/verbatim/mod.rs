mod atom_differentiation;
mod tuple_appendation;
mod unitarity_assertion;

pub use atom_differentiation::*;
use tbl_proof_calculus::{
    proofs::assertions::{
        ExpressionLengthCheckError, UnwrapInvocationExpressionError,
        unwrap_fixed_length_invocation_expression,
    },
    structures::{
        expressions::{compound::CompoundTblExpression, located::OwnedTblExpressionAtPath},
        proof_calculus_derived::path_composites::{
            TblExpressionInInference, TblExpressionInInferencePath,
        },
    },
};
pub use tuple_appendation::*;
pub use unitarity_assertion::*;

use itertools::Either;

use crate::structures::atoms::PhilosophicaInferenceAtoms;

#[derive(Clone)]
pub enum UnwrapVerbatimExpressionError<C: CompoundTblExpression, Path> {
    ExpressionUnitary(OwnedTblExpressionAtPath<C, Path>),
    NoFirstElement(OwnedTblExpressionAtPath<C, Path>),
    WrongHead(OwnedTblExpressionAtPath<C, Path>),
    WrongLength(ExpressionLengthCheckError<C, Path>),
}
impl<C: CompoundTblExpression, Path> UnwrapVerbatimExpressionError<C, Path> {
    fn expression(self) -> OwnedTblExpressionAtPath<C, Path> {
        match self {
            UnwrapVerbatimExpressionError::ExpressionUnitary(expr) => expr,
            UnwrapVerbatimExpressionError::NoFirstElement(expr) => expr,
            UnwrapVerbatimExpressionError::WrongHead(expr) => expr,
            UnwrapVerbatimExpressionError::WrongLength(err) => err.expression,
        }
    }
}

/// Take an expression, and if it is in the form (Verbatim, e) return e, otherwise return an Error
#[inline]
pub fn unwrap_verbatim_expression<'a, C: CompoundTblExpression>(
    verbatim: &'a TblExpressionInInference<'a, C>,
) -> Result<
    TblExpressionInInference<'a, C>,
    UnwrapVerbatimExpressionError<C, TblExpressionInInferencePath>,
> {
    let [tail] = *unwrap_fixed_length_invocation_expression(
        verbatim,
        &PhilosophicaInferenceAtoms::Verbatim.into(),
    )
    .map_err(|e| match e {
        Either::Left(UnwrapInvocationExpressionError::ExpressionUnitary(e)) => {
            UnwrapVerbatimExpressionError::ExpressionUnitary(e)
        }
        Either::Left(UnwrapInvocationExpressionError::NoFirstElement(e)) => {
            UnwrapVerbatimExpressionError::NoFirstElement(e)
        }
        Either::Left(UnwrapInvocationExpressionError::WrongHead(e)) => {
            UnwrapVerbatimExpressionError::WrongHead(e.expression)
        }
        Either::Right(e) => UnwrapVerbatimExpressionError::WrongLength(e),
    })?;
    Ok(tail)
}
