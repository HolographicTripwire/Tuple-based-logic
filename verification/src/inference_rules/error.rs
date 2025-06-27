use tbl_structures::{inference::{path::InferenceSubexpressionPath, Inference, InferenceRule}, propositions::Expression};

use crate::{inference_rules::InferenceVerifier, validation_error::ProofValidationError};

#[derive(Clone)]
pub enum ProofStepSpecificationError {
    WrongAssumptionCount(usize),
    WrongConclusionCount(usize),
    WrongAtomicity(InferenceSubexpressionPath,bool),
    WrongLength(InferenceSubexpressionPath,usize),
    WrongValue(InferenceSubexpressionPath,Expression),
    MismatchedLengths(InferenceSubexpressionPath,InferenceSubexpressionPath),
    MismatchedValues(InferenceSubexpressionPath,InferenceSubexpressionPath)
}

pub fn verify_inference<Rule: VerifiableInferenceRule>(inference: &Inference<Rule>) -> Result<(),ProofValidationError> {
    let verifier = Rule::get_verifier(&inference.inference_type);
    match verifier(&inference) {
        Ok(()) => Ok(()),
        Err(err) => Err(ProofValidationError::InvalidStepSpecification(err)),
    }
}

pub trait VerifiableInferenceRule: InferenceRule {
    fn get_verifier(rule: &Self) -> impl InferenceVerifier<Self>;
}
