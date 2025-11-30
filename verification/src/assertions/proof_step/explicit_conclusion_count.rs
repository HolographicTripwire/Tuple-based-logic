use tbl_structures::{inference::InferenceRule, proof::{InferenceInProof, OwnedInferenceInProof}};

pub struct ExplicitConclusionCountCheckError<Rule: InferenceRule> {
    expected_count: usize,
    inference: OwnedInferenceInProof<Rule>,
}
impl<Rule: InferenceRule> ExplicitConclusionCountCheckError<Rule> {
    pub fn new(expected_count: usize, inference: OwnedInferenceInProof<Rule>) -> Self {
        Self {
            expected_count,
            inference,
        }
    }
}

pub fn format_explicit_conclusion_count_check_error<Rule: InferenceRule>(err: ExplicitConclusionCountCheckError<Rule>) -> String {
    format!("Proof at step {step} has wrong number of explicit conclusions (expected {expected_count}; found {actual_count}",
        step = err.inference.0.path(),
        expected_count = err.expected_count,
        actual_count = err.inference.0.obj().assumptions.len()
    )
}

/// Check that the provided [Inference](OwnedInferenceInProof) has expected_count explicit conclusions, returning an error otherwise
pub fn assert_explicit_conclusion_count<'a, Rule: InferenceRule, T: From<ExplicitConclusionCountCheckError<Rule>>>
(inference: InferenceInProof<Rule>,expected_count: usize,) -> Result<(), T> {
    if inference.0.obj().assumptions.len() == expected_count {
        Ok(())
    } else {
        Err(ExplicitConclusionCountCheckError::new(expected_count, inference.into_owned()).into())
    }
}
