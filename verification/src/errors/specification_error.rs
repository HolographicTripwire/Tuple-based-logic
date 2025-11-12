use tbl_structures::{inference::InferenceRule, proof::InferenceInProof};

use crate::errors::validation_error::ProofValidationError;

pub fn verify_inference<InnerErr, Rule: VerifiableInferenceRule<InnerErr>, StepErr: From<InnerErr>>(inference: &InferenceInProof<Rule>) -> Result<(),ProofValidationError<InnerErr>> {
    match Rule::verify(inference) {
        Ok(()) => Ok(()),
        Err(err) => Err(ProofValidationError::InvalidInference(err)),
    }
}

pub trait VerifiableInferenceRule<Err>: InferenceRule {
    fn verify(rule: &InferenceInProof<Self>) -> Result<(),Err>;
}
