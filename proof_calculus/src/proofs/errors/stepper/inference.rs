use crate::{proofs::{errors::{ValidatableInferenceRule, stepper::result::ProofValidityStepResultWrapper, validate_located_inference}, inferences::located::InferenceAtPath}, propositions::assigned::Proposition};

pub struct InferenceValidityStepper<'a,P:Proposition,Rule:ValidatableInferenceRule<P>,Path>(
    Option<InferenceAtPath<'a,P,Rule,Path>>
);

impl <'a,P: Proposition, Rule: ValidatableInferenceRule<P>, Path> InferenceValidityStepper<'a,P,Rule,Path> {
    pub fn new(inference: InferenceAtPath<'a,P,Rule,Path>) -> Self { Self(Some(inference)) }

    pub fn step<JoinedPath>(&mut self) -> ProofValidityStepResultWrapper<P,Rule::Err,Path,JoinedPath> {
        if let Some(inference) = self.0.take() {
            match validate_located_inference(inference) {
                Ok(_) => ProofValidityStepResultWrapper::finished_no_err(),
                Err(err) => ProofValidityStepResultWrapper::finished_parent_err(err),
            }
        } else { ProofValidityStepResultWrapper::finished_no_err() }
    }

    pub fn is_finished(&self) -> bool { self.0.is_none() }
}
