use tbl_structures::{inference::{Inference, InferenceRule}};

use crate::errors::validation_error::ProofValidationError;

pub fn verify_inference<Err, Rule: VerifiableInferenceRule<Err>>(inference: &Inference<Rule>) -> Result<(),ProofValidationError<Err>> {
    Rule::verify(inference)
        .map_err(|err| ProofValidationError::InvalidInference(err))
}

pub trait VerifiableInferenceRule<Err>: InferenceRule {
    fn verify(rule: &Inference<Self>) -> Result<(),Err>;
}
