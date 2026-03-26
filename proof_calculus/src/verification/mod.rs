use crate::structures::{Proposition, inference::{Inference, InferenceRule}};

pub trait VerifiableInferenceRule<P:Proposition>: InferenceRule<P> {
    type Err: Clone;

    fn verify(rule: &Inference<P,Self>) -> Result<(),Self::Err>;
}
