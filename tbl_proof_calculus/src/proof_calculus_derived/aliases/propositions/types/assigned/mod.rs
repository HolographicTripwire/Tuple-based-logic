use proof_calculus::propositions::types::assigned::Proposition;

use crate::{
    expressions::types::assigned::{TblExpression, compound::TblExpressionCompound},
    proof_calculus_derived::path_composites::OwnedTblPropositionInProof,
};

pub mod binding;

pub type TblProposition<C> = TblExpression<C>;
pub type TblPropositionInSequentialProof<C> = OwnedTblPropositionInProof<TblProposition<C>>;

impl<C: TblExpressionCompound> Proposition for TblExpression<C> {}
