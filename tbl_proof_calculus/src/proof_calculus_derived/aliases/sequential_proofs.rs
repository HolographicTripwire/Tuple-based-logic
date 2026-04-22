use proof_calculus::proofs::sequential::SequentialProof;

use crate::{proof_calculus_derived::aliases::{inferences::TblInferenceRule, propositions::TblProposition}, expressions::assigned::compound::CompoundTblExpression};

pub struct SequentialTblProof<C: CompoundTblExpression,Rule:TblInferenceRule<C>>(pub SequentialProof<TblProposition<C>,Rule>);
//pub type SequentialTblProof<C: CompoundTblExpression,Rule: TblInferenceRule<C>> = SequentialProof<TblProposition<C>,Rule>;
