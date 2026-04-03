use itertools::Either;

use crate::{structures::{propositions::Proposition, sequential_proofs::{at_path_enum::SequentialProofAtPathEnum, subproofs::{SequentialProofAtPath, immediate::ImmediateSequentialProofInProofPath}}}, verification::validity::{ValidatableInferenceRule, error::OwnedProofValidityErrorAtPath, stepper::{composite::{CompositeProofValidityStepper}, inference::InferenceValidityStepper}}};

mod inference;
mod composite;

pub struct ProofValidityStepResultWrapper<P:Proposition,IE:Clone,ParentPath,JoinedPath> {
    is_finished: bool,
    next_result: ProofValidityStepResult<P,IE,ParentPath,JoinedPath>
}

pub type ProofValidityStepResult<P,IE,ParentPath,JoinedPath> = Result<(),ProofValidityStepErr<P,IE,ParentPath,JoinedPath>>;
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum ProofValidityStepErr<P:Proposition,IE:Clone,ParentPath,JoinedPath> {
    InParent(OwnedProofValidityErrorAtPath<P,IE,ParentPath>),
    InChild(OwnedProofValidityErrorAtPath<P,IE,JoinedPath>),
}
impl <P:Proposition,IE:Clone,ParentPath,JoinedPath> ProofValidityStepErr<P,IE,ParentPath,JoinedPath> {
    pub fn replace_path<NewParentPath,NewJoinedPath>(self, parent_replace: impl Fn(ParentPath) -> NewParentPath, joined_replace: impl Fn(JoinedPath) -> NewJoinedPath) -> ProofValidityStepErr<P,IE,NewParentPath,NewJoinedPath> {
        match self {
            ProofValidityStepErr::InParent(parent) => ProofValidityStepErr::InParent(parent.replace_path(parent_replace)),
            ProofValidityStepErr::InChild(child) => ProofValidityStepErr::InChild(child.replace_path(joined_replace)),
        }
    }
}
impl <P:Proposition,IE:Clone,ParentPath,JoinedPath> ProofValidityStepResultWrapper<P,IE,ParentPath,JoinedPath> {
    fn unfinished_no_err() -> Self { Self {
        is_finished: false,
        next_result: Ok(())
    }}
    fn finished_no_err() -> Self { Self {
        is_finished: true,
        next_result: Ok(())
    }}
    fn _unfinished_parent_err(err: OwnedProofValidityErrorAtPath<P,IE,ParentPath>) -> Self { Self {
        is_finished: false,
        next_result: Err(ProofValidityStepErr::InParent(err))
    }}
    fn finished_parent_err(err: OwnedProofValidityErrorAtPath<P,IE,ParentPath>) -> Self { Self {
        is_finished: true,
        next_result: Err(ProofValidityStepErr::InParent(err))
    }}
    fn unfinished_child_err(err: OwnedProofValidityErrorAtPath<P,IE,JoinedPath>) -> Self { Self {
        is_finished: false,
        next_result: Err(ProofValidityStepErr::InChild(err))
    }}
    fn _finished_child_err(err: OwnedProofValidityErrorAtPath<P,IE,JoinedPath>) -> Self { Self {
        is_finished: true,
        next_result: Err(ProofValidityStepErr::InChild(err))
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
            Ok(()) => if result.is_finished { Some(Ok(())) } else { None },
            error => { Some(error) }
        }
    }
}
