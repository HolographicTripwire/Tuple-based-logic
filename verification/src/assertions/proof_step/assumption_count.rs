use std::fmt::Display;

use tbl_structures::{inference::InferenceRule, proof::{InferenceInProof,OwnedInferenceInProof}};

pub struct AssumptionCountCheckError<Rule: InferenceRule> {
    expected_count: usize,
    inference: OwnedInferenceInProof<Rule>
}
impl <Rule: InferenceRule> AssumptionCountCheckError<Rule> {
    pub fn new(expected_count: usize, inference: OwnedInferenceInProof<Rule>) -> Self
        { Self { expected_count, inference } }
    
}
impl <Rule: InferenceRule> Display for AssumptionCountCheckError<Rule> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Proof at step {step} has wrong number of assumptions (expected {expected_count}; found {actual_count}",
            step=self.inference.0.path(),
            expected_count=self.expected_count,
            actual_count=self.inference.0.obj().assumptions.len()
        )
    }
}

/// Check that the provided [Inference](OwnedInferenceInProof) has expected_count assumptions, returning an error otherwise
pub fn assert_assumption_count<'a,Rule: InferenceRule, T: From<AssumptionCountCheckError<Rule>>>(inference: InferenceInProof<Rule>, expected_count: usize) -> Result<(), T> {
    if inference.0.obj().assumptions.len() == expected_count { Ok(()) }
    else { Err(AssumptionCountCheckError::new(expected_count, inference.into_owned()).into()) }
}
