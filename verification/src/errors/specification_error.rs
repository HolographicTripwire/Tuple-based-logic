use tbl_structures::{inference::{Inference, InferenceRule}};

use crate::errors::validation_error::ProofValidationError;

pub fn verify_inference<InnerErr, Rule: VerifiableInferenceRule<InnerErr>, StepErr: From<InnerErr>>(inference: &Inference<Rule>) -> Result<(),ProofValidationError<InnerErr>> {
    Rule::verify(inference)
        .map_err(|err| ProofValidationError::InvalidInference(err))
}

pub trait VerifiableInferenceRule<Err>: InferenceRule {
    fn verify(rule: &Inference<Self>) -> Result<(),Err>;
}
