use crate::structures::{expressions::TblExpression, proof_calculus_derived::path_composites::OwnedTblPropositionInProof};

pub type TblProposition<C> = TblExpression<C>;

pub type TblPropositionInSequentialProof<C> = OwnedTblPropositionInProof<TblProposition<C>>;
