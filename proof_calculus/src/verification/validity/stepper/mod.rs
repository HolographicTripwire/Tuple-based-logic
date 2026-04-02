use itertools::Either;

use crate::{structures::{propositions::Proposition, sequential_proofs::{at_path_enum::SequentialProofAtPathEnum, subproofs::{SequentialProofAtPath, immediate::ImmediateSequentialProofInProofPath}}}, verification::validity::{ValidatableInferenceRule, error::OwnedProofValidityErrorAtPath, stepper::{composite::{CompositeProofValidityStepper}, inference::InferenceValidityStepper}}};

mod inference;
mod composite;

pub struct ProofValidityStepResultWrapper<P:Proposition,IE:Clone,ParentPath,JoinedPath> {
    is_finished: bool,
    next_result: ProofValidityStepResult<P,IE,ParentPath,JoinedPath>
}
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum ProofValidityStepResult<P:Proposition,IE:Clone,ParentPath,JoinedPath> {
    Ok,
    ErrInParent(OwnedProofValidityErrorAtPath<P,IE,ParentPath>),
    ErrInChild(OwnedProofValidityErrorAtPath<P,IE,JoinedPath>),
}
impl <P:Proposition,IE:Clone,ParentPath,JoinedPath> ProofValidityStepResult<P,IE,ParentPath,JoinedPath> {
    pub fn replace_path<NewParentPath,NewJoinedPath>(self, parent_replace: impl Fn(ParentPath) -> NewParentPath, joined_replace: impl Fn(JoinedPath) -> NewJoinedPath) -> ProofValidityStepResult<P,IE,NewParentPath,NewJoinedPath> {
        match self {
            ProofValidityStepResult::Ok => ProofValidityStepResult::Ok,
            ProofValidityStepResult::ErrInParent(parent) => ProofValidityStepResult::ErrInParent(parent.replace_path(parent_replace)),
            ProofValidityStepResult::ErrInChild(child) => ProofValidityStepResult::ErrInChild(child.replace_path(joined_replace)),
        }
    }
}
impl <P:Proposition,IE:Clone,ParentPath,JoinedPath> ProofValidityStepResultWrapper<P,IE,ParentPath,JoinedPath> {
    fn unfinished_no_err() -> Self { Self {
        is_finished: false,
        next_result: ProofValidityStepResult::Ok
    }}
    fn finished_no_err() -> Self { Self {
        is_finished: true,
        next_result: ProofValidityStepResult::Ok
    }}
    fn unfinished_parent_err(err: OwnedProofValidityErrorAtPath<P,IE,ParentPath>) -> Self { Self {
        is_finished: false,
        next_result: ProofValidityStepResult::ErrInParent(err)
    }}
    fn finished_parent_err(err: OwnedProofValidityErrorAtPath<P,IE,ParentPath>) -> Self { Self {
        is_finished: true,
        next_result: ProofValidityStepResult::ErrInParent(err)
    }}
    fn unfinished_child_err(err: OwnedProofValidityErrorAtPath<P,IE,JoinedPath>) -> Self { Self {
        is_finished: false,
        next_result: ProofValidityStepResult::ErrInChild(err)
    }}
    fn finished_child_err(err: OwnedProofValidityErrorAtPath<P,IE,JoinedPath>) -> Self { Self {
        is_finished: true,
        next_result: ProofValidityStepResult::ErrInChild(err)
    }}
}

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
            ProofValidityStepResult::Ok => if result.is_finished { Some(ProofValidityStepResult::Ok) } else { None },
            error => { Some(error) }
        }
    }
}
