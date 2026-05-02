use proof_calculus::proofs::inferences::{Inference, InferenceRule};
use trait_aliases::trait_aliases;

use crate::{expressions::types::assigned::compound::CompoundTblExpression, proof_calculus_derived::aliases::propositions::types::TblProposition};

//pub struct TblInference<C: CompoundTblExpression,Rule:TblInferenceRule<C>>(pub Inference<TblProposition<C>,Rule>);
pub type TblInference<C: CompoundTblExpression,Rule: TblInferenceRule<C>> = Inference<TblProposition<C>,Rule>;

trait_aliases!{
    pub trait TblInferenceRule<C: CompoundTblExpression> = InferenceRule<TblProposition<C>>;
}
