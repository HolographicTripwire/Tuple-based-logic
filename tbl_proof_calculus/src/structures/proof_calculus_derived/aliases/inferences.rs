use proof_calculus::structures::inferences::{Inference, InferenceRule};
use trait_aliases::trait_aliases;

use crate::structures::{proof_calculus_derived::aliases::propositions::TblProposition, expressions::{compound::CompoundTblExpression}};

//pub struct TblInference<C: CompoundTblExpression,Rule:TblInferenceRule<C>>(pub Inference<TblProposition<C>,Rule>);
pub type TblInference<C: CompoundTblExpression,Rule: TblInferenceRule<C>> = Inference<TblProposition<C>,Rule>;

trait_aliases!{
    pub trait TblInferenceRule<C: CompoundTblExpression> = InferenceRule<TblProposition<C>>;
}
