mod atomicity_assertion;
mod atom_differentiation;
mod tuple_appendation;

pub use atom_differentiation::*;
pub use atomicity_assertion::*;
pub use tuple_appendation::*;

use itertools::Either;

use tbl_verification::{validity::*};
use tbl_structures::{atoms::BuiltInAtom, path_composites::{ExpressionInInference, OwnedExpressionInInference}};

#[derive(Clone)]
pub enum UnwrapVerbatimExpressionError {
    ExpressionAtomic(OwnedExpressionInInference),
    NoFirstElement(OwnedExpressionInInference),
    WrongHead(OwnedExpressionInInference),
    WrongLength(ExpressionLengthCheckError)
}
impl UnwrapVerbatimExpressionError {
    fn expression(self) -> OwnedExpressionInInference { match self {
        UnwrapVerbatimExpressionError::ExpressionAtomic(expr) => expr,
        UnwrapVerbatimExpressionError::NoFirstElement(expr) => expr,
        UnwrapVerbatimExpressionError::WrongHead(expr) => expr,
        UnwrapVerbatimExpressionError::WrongLength(err) => err.expression,
    }}
}

/// Take an expression, and if it is in the form (Verbatim, e) return e, otherwise return an Error
pub fn unwrap_verbatim_expression<'a>(verbatim: &'a ExpressionInInference<'a>) -> Result<ExpressionInInference<'a>,UnwrapVerbatimExpressionError>{
    let [tail] = *unwrap_fixed_length_invocation_expression(verbatim, &BuiltInAtom::Verbatim.into()).map_err(|e| match e {
        Either::Left(UnwrapInvocationExpressionError::ExpressionAtomic(e)) => UnwrapVerbatimExpressionError::ExpressionAtomic(e),
        Either::Left(UnwrapInvocationExpressionError::NoFirstElement(e)) => UnwrapVerbatimExpressionError::NoFirstElement(e),
        Either::Left(UnwrapInvocationExpressionError::WrongHead(e)) => UnwrapVerbatimExpressionError::WrongHead(e.expression),
        Either::Right(e) => UnwrapVerbatimExpressionError::WrongLength(e)
    })?;
    Ok(tail)
}
