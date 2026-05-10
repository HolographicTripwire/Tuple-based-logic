use proof_calculus::proofs::sequential::SequentialProof;

use crate::{
    expressions::types::assigned::compound::TblExpressionCompound,
    proof_calculus_derived::aliases::{
        inferences::TblInferenceRule, propositions::types::assigned::TblProposition,
    },
};

pub struct SequentialTblProof<C: TblExpressionCompound, Rule: TblInferenceRule<C>>(
    pub SequentialProof<TblProposition<C>, Rule>,
);
//pub type SequentialTblProof<C: CompoundTblExpression,Rule: TblInferenceRule<C>> = SequentialProof<TblProposition<C>,Rule>;
