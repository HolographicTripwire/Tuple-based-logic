use crate::{proofs::{inferences::{Inference, InferenceRule}}, propositions::Proposition};

pub struct AssumptionCountCheckError<P: Proposition, Rule: InferenceRule<P>> {
    pub expected_count: usize,
    pub inference: Inference<P,Rule>
}
impl <P: Proposition, Rule: InferenceRule<P>> AssumptionCountCheckError<P,Rule> {
    pub fn get_actual_count(&self) -> usize { self.inference.assumptions.len() }
}

/// Check that the provided [Inference](OwnedInferenceInProof) has expected_count assumptions, returning an error otherwise
pub fn assert_assumption_count<'a,P: Proposition, Rule: InferenceRule<P>>(inference: &Inference<P,Rule>, expected_count: usize) -> Result<(), AssumptionCountCheckError<P,Rule>> {
    if inference.assumptions.len() == expected_count { Ok(()) }
    else { Err(AssumptionCountCheckError{
        expected_count, 
        inference: inference.clone()
    }) }
}
