mod atomicity_assertion;
mod atom_differentiation;
mod tuple_appendation;

pub use atom_differentiation::*;
pub use atomicity_assertion::*;
use tbl_proof_calculus::{structures::{expressions::{compound::CompoundTblExpression, located::OwnedTblExpressionAtPath}, proof_calculus_derived::path_composites::{TblExpressionInInference, TblExpressionInInferencePath}}, proofs::assertions::{ExpressionLengthCheckError, UnwrapInvocationExpressionError, unwrap_fixed_length_invocation_expression}};
pub use tuple_appendation::*;

use itertools::Either;

use crate::structures::atoms::PhilosophicaInferenceAtoms;


#[derive(Clone)]
pub enum UnwrapVerbatimExpressionError<C:CompoundTblExpression,Path> {
    ExpressionAtomic(OwnedTblExpressionAtPath<C,Path>),
    NoFirstElement(OwnedTblExpressionAtPath<C,Path>),
    WrongHead(OwnedTblExpressionAtPath<C,Path>),
    WrongLength(ExpressionLengthCheckError<C,Path>)
}
impl <C: CompoundTblExpression,Path> UnwrapVerbatimExpressionError<C,Path> {
    fn expression(self) -> OwnedTblExpressionAtPath<C,Path> { match self {
        UnwrapVerbatimExpressionError::ExpressionAtomic(expr) => expr,
        UnwrapVerbatimExpressionError::NoFirstElement(expr) => expr,
        UnwrapVerbatimExpressionError::WrongHead(expr) => expr,
        UnwrapVerbatimExpressionError::WrongLength(err) => err.expression,
    }}
}

/// Take an expression, and if it is in the form (Verbatim, e) return e, otherwise return an Error
#[inline]
pub fn unwrap_verbatim_expression<'a,C:CompoundTblExpression>(verbatim: &'a TblExpressionInInference<'a,C>) -> Result<TblExpressionInInference<'a,C>,UnwrapVerbatimExpressionError<C,TblExpressionInInferencePath>>{
    let [tail] = *unwrap_fixed_length_invocation_expression(verbatim, &PhilosophicaInferenceAtoms::Verbatim.into()).map_err(|e| match e {
        Either::Left(UnwrapInvocationExpressionError::ExpressionAtomic(e)) => UnwrapVerbatimExpressionError::ExpressionAtomic(e),
        Either::Left(UnwrapInvocationExpressionError::NoFirstElement(e)) => UnwrapVerbatimExpressionError::NoFirstElement(e),
        Either::Left(UnwrapInvocationExpressionError::WrongHead(e)) => UnwrapVerbatimExpressionError::WrongHead(e.expression),
        Either::Right(e) => UnwrapVerbatimExpressionError::WrongLength(e)
    })?;
    Ok(tail)
}
