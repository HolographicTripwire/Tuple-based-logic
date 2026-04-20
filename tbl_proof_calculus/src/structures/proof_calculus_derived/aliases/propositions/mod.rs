use proof_calculus::propositions::Proposition;

use crate::structures::{expressions::{TblExpression, compound::CompoundTblExpression}, proof_calculus_derived::path_composites::OwnedTblPropositionInProof};

pub mod bounds;

pub type TblProposition<C> = TblExpression<C>;

pub type TblPropositionInSequentialProof<C> = OwnedTblPropositionInProof<TblProposition<C>>;

impl <C: CompoundTblExpression> Proposition for TblExpression<C> {}
