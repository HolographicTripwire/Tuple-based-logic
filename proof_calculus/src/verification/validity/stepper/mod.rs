use itertools::Either;

use crate::{structures::{propositions::Proposition, sequential_proofs::{at_path_enum::SequentialProofAtPathEnum, subproofs::{SequentialProofAtPath, immediate::ImmediateSequentialProofInProofPath}}}, verification::validity::{ValidatableInferenceRule, stepper::{composite::CompositeProofValidityStepper, inference::InferenceValidityStepper, result::{ProofValidityStepResult, ProofValidityStepResultWrapper}}}};

mod inference;
mod composite;
pub mod result;

pub struct ProofValidityStepper<'a,P:Proposition,Rule:ValidatableInferenceRule<P>,ParentPath: Clone,JoinedPath: Clone + From<(ParentPath,ImmediateSequentialProofInProofPath)> + From<(JoinedPath,ImmediateSequentialProofInProofPath)>>(
    Either<InferenceValidityStepper<'a,P,Rule,ParentPath>,CompositeProofValidityStepper<'a,P,Rule,ParentPath,JoinedPath>>
);
impl <'a,P:Proposition,Rule:ValidatableInferenceRule<P>,ParentPath: Clone,JoinedPath: Clone + From<(ParentPath,ImmediateSequentialProofInProofPath)> + From<(JoinedPath,ImmediateSequentialProofInProofPath)>>
ProofValidityStepper<'a,P,Rule,ParentPath,JoinedPath> {
    pub fn new(proof: SequentialProofAtPath<'a,P,Rule,ParentPath>) -> Self {
        match proof.into() {
            SequentialProofAtPathEnum::Inference(inference) => Self(Either::Left(InferenceValidityStepper::new(inference))),
            SequentialProofAtPathEnum::Composite(composite) => Self(Either::Right(CompositeProofValidityStepper::new(composite)))
        }
    }

    pub fn step(&mut self) -> ProofValidityStepResultWrapper<P,Rule::Err,ParentPath,JoinedPath> {
        match &mut self.0 {
            Either::Left(inference_stepper) => inference_stepper.step(),
            Either::Right(composite_stepper) => composite_stepper.step(),
        }
    }

    pub fn is_finished(&self) -> bool {
        match &self.0 {
            Either::Left(inference_stepper) => inference_stepper.is_finished(),
            Either::Right(composite_stepper) => composite_stepper.is_finished(),
        }
    }
}
impl <'a,P:Proposition,Rule:ValidatableInferenceRule<P>,ParentPath: Clone,JoinedPath: Clone + From<(ParentPath,ImmediateSequentialProofInProofPath)> + From<(JoinedPath,ImmediateSequentialProofInProofPath)>> Iterator for ProofValidityStepper<'a,P,Rule,ParentPath,JoinedPath> {
    type Item = ProofValidityStepResult<P,Rule::Err,ParentPath,JoinedPath>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.step();
        match result.next_result {
            Ok(()) => if result.is_finished { Some(Ok(())) } else { None },
            error => { Some(error) }
        }
    }
}
