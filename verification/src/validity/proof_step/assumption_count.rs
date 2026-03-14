use tbl_structures::proof::inference::{Inference, InferenceRule};


pub struct AssumptionCountCheckError<Rule: InferenceRule> {
    pub expected_count: usize,
    pub inference: Inference<Rule>
}
impl <Rule: InferenceRule> AssumptionCountCheckError<Rule> {
    pub fn get_actual_count(&self) -> usize { self.inference.assumptions.len() }
}

/// Check that the provided [Inference](OwnedInferenceInProof) has expected_count assumptions, returning an error otherwise
pub fn assert_assumption_count<'a,Rule: InferenceRule>(inference: &Inference<Rule>, expected_count: usize) -> Result<(), AssumptionCountCheckError<Rule>> {
    if inference.assumptions.len() == expected_count { Ok(()) }
    else { Err(AssumptionCountCheckError{
        expected_count, 
        inference: inference.clone()
    }) }
}
