use tbl_structures::sequential_proofs::inference::{Inference, InferenceRule};

pub struct ExplicitConclusionCountCheckError<Rule: InferenceRule> {
    pub expected_count: usize,
    pub inference: Inference<Rule>
}
impl <Rule: InferenceRule> ExplicitConclusionCountCheckError<Rule> {
    pub fn get_actual_count(&self) -> usize { self.inference.conclusions.len() }
}

/// Check that the provided [Inference](OwnedInferenceInProof) has expected_count explicit conclusions, returning an error otherwise
pub fn assert_explicit_conclusion_count<'a, Rule: InferenceRule>(inference: &Inference<Rule>,expected_count: usize,) -> Result<(), ExplicitConclusionCountCheckError<Rule>> {
    if inference.assumptions.len() == expected_count {
        Ok(())
    } else {
        Err(ExplicitConclusionCountCheckError{
            expected_count: expected_count,
            inference: inference.clone()
        })
    }
}
