use tbl_structures::inference::{Inference, InferenceRule};

pub struct AssumptionCountCheckError<Rule: InferenceRule> {
    pub expected_count: usize,
    pub inference: Inference<Rule>
}
impl <Rule: InferenceRule> AssumptionCountCheckError<Rule> {
    pub fn get_actual_count(&self) -> usize { self.inference.assumptions.len() }
}

pub fn format_assumption_count_check_error<Rule: InferenceRule>(err: AssumptionCountCheckError<Rule>) -> String {
    format!("Inference has wrong number of assumptions (expected {expected_count}; found {actual_count}",
        expected_count=err.expected_count,
        actual_count=err.get_actual_count()
    )
}

/// Check that the provided [Inference](OwnedInferenceInProof) has expected_count assumptions, returning an error otherwise
pub fn assert_assumption_count<'a,Rule: InferenceRule>(inference: &Inference<Rule>, expected_count: usize) -> Result<(), AssumptionCountCheckError<Rule>> {
    if inference.assumptions.len() == expected_count { Ok(()) }
    else { Err(AssumptionCountCheckError{
        expected_count, 
        inference: inference.clone()
    }) }
}
