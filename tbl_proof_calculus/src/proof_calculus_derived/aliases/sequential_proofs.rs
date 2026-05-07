use proof_calculus::proofs::sequential::SequentialProof;

use crate::{proof_calculus_derived::aliases::{inferences::TblInferenceRule, propositions::types::TblProposition}, expressions::types::assigned::compound::TblExpressionCompound};

pub struct SequentialTblProof<C: TblExpressionCompound,Rule:TblInferenceRule<C>>(pub SequentialProof<TblProposition<C>,Rule>);
//pub type SequentialTblProof<C: CompoundTblExpression,Rule: TblInferenceRule<C>> = SequentialProof<TblProposition<C>,Rule>;
