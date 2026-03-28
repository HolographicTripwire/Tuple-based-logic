use proof_calculus::structures::sequential_proofs::SequentialProof;

use crate::structures::{proof_calculus_derived::aliases::{inferences::TblInferenceRule, propositions::TblProposition}, expressions::compound::CompoundTblExpression};

pub struct SequentialTblProof<C: CompoundTblExpression,Rule:TblInferenceRule<C>>(pub SequentialProof<TblProposition<C>,Rule>);
//pub type SequentialTblProof<C: CompoundTblExpression,Rule: TblInferenceRule<C>> = SequentialProof<TblProposition<C>,Rule>;
