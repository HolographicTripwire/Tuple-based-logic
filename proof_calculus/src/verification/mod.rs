use crate::structures::{propositions::Proposition, inferences::{Inference, InferenceRule}};

pub mod inferences;
pub mod abstract_proofs;
pub mod sequential_proofs;

pub trait ValidatableInferenceRule<P:Proposition>: InferenceRule<P> {
    type Err: Clone;

    fn validate(rule: &Inference<P,Self>) -> Result<(),Self::Err>;
}
