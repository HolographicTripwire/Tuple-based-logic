use itertools::Either;
use path_lib::obj_at_path::OwnedObjAtPath;

use crate::{structures::{propositions::{LocatedParentOfExplicitConclusions, ParentOfAssumptions, ParentOfExplicitConclusions, Proposition}, propsets::implementations::hash::HashPropSet1O, sequential_proofs::{at_path_enum::SequentialProofAtPathEnum, composite::CompositeSequentialProofAtPath, subproofs::{ParentOfSubproofs, SequentialProofAtPath, immediate::{ImmediateSequentialProofInProofPath, LocatedParentOfImmediateSubproofs}}}}, verification::validity::{ProofValidityError, ValidatableInferenceRule, stepper::{ProofValidityStepResultWrapper, result::ProofValidityStepErr}, validate_inference}};

#[derive(PartialEq,Eq,Debug)]
enum CompositeProofValidityStep {
    CheckAssumptionsFound(usize),
    CheckInternalValidity(usize),
    CheckConclusionsFound,
    Finished
}

pub struct CompositeProofValidityStepper<'a,P:Proposition,Rule:ValidatableInferenceRule<P>,ParentPath: 'a + Clone,JoinedPath: 'a + Clone + From<(ParentPath,ImmediateSequentialProofInProofPath)> + From<(JoinedPath,ImmediateSequentialProofInProofPath)>> {
    proof: CompositeSequentialProofAtPath<'a,P,Rule,ParentPath>,
    step_count: usize,
    
    proved: HashPropSet1O<P>,
    
    current_step: CompositeProofValidityStep,
    inner: Option<Box<CompositeProofValidityStepper<'a,P,Rule,JoinedPath,JoinedPath>>>,
}

impl <'a,P:Proposition,Rule:ValidatableInferenceRule<P>,ParentPath: 'a + Clone,JoinedPath: 'a + Clone + From<(ParentPath,ImmediateSequentialProofInProofPath)> + From<(JoinedPath,ImmediateSequentialProofInProofPath)>>
CompositeProofValidityStepper<'a,P,Rule,ParentPath,JoinedPath> {
    pub fn new(proof: CompositeSequentialProofAtPath<'a,P,Rule,ParentPath>) -> Self {
        let proof_obj = proof.obj;
        Self {
            current_step: CompositeProofValidityStep::CheckAssumptionsFound(0),
            proved: HashPropSet1O::from_iter(proof_obj.get_assumptions_owned()),
            step_count: proof_obj.get_subproof_count(),
            proof,
            inner: None,
        }
    }

    pub fn step(&mut self) -> ProofValidityStepResultWrapper<P,Rule::Err,ParentPath,JoinedPath> {
        match self.current_step {
            CompositeProofValidityStep::CheckAssumptionsFound(step_number) => self.check_assumptions_step(step_number),
            CompositeProofValidityStep::CheckInternalValidity(step_number) => self.internal_validation_step(step_number),
            CompositeProofValidityStep::CheckConclusionsFound => self.check_conclusions_step(),
            CompositeProofValidityStep::Finished => return ProofValidityStepResultWrapper::finished_no_err()
        }
    }

    pub fn is_finished(&self) -> bool { self.current_step == CompositeProofValidityStep::Finished }

    fn next_step(&self) -> CompositeProofValidityStep {
        match self.current_step {
            CompositeProofValidityStep::CheckAssumptionsFound(step_no) => CompositeProofValidityStep::CheckInternalValidity(step_no),
            CompositeProofValidityStep::CheckInternalValidity(step_no) => 
                if step_no >= self.step_count {CompositeProofValidityStep::CheckConclusionsFound}
                else {CompositeProofValidityStep::CheckAssumptionsFound(step_no + 1)},
            CompositeProofValidityStep::CheckConclusionsFound => CompositeProofValidityStep::Finished,
            CompositeProofValidityStep::Finished => CompositeProofValidityStep::Finished,
        }
    }

    fn check_assumptions_step(&mut self, step_number: usize) -> ProofValidityStepResultWrapper<P,Rule::Err,ParentPath,JoinedPath> {
        let subproof: SequentialProofAtPath<'_,P,Rule,JoinedPath> = self.proof.get_located_immediate_subproof(ImmediateSequentialProofInProofPath(step_number))
            .expect("Attempted to call get_subproof when step was not within range");
        let premises = HashPropSet1O::from_iter(subproof.obj.get_assumptions_owned());
        // Determine if an error is present
        let assumptions_not_found = &self.proved - &premises;
        let result = if assumptions_not_found.len() > 0 {
            ProofValidityStepResultWrapper::unfinished_child_err(OwnedObjAtPath{
                obj: ProofValidityError::AssumptionsNotFound(assumptions_not_found),
                path: subproof.path.clone()
            })
        } else { ProofValidityStepResultWrapper::unfinished_no_err() };
        // Move to the next step
        self.current_step = self.next_step();
        // Return the result
        result
    }

    fn internal_validation_step(&mut self, step_number: usize) -> ProofValidityStepResultWrapper<P,Rule::Err,ParentPath,JoinedPath> {
        let next_step = self.next_step();
        // Get internal value
        let internal = {
            if self.inner.is_some() { Either::Right(&mut self.inner)
            } else { match self.proof.get_located_immediate_subproof(ImmediateSequentialProofInProofPath(step_number))
                    .expect("Attempted to call get_subproof when step was not within range")
                    .into() {
                SequentialProofAtPathEnum::Inference(inference) => Either::Left(inference),
                SequentialProofAtPathEnum::Composite(composite) => {
                    self.inner = Some(Box::new(CompositeProofValidityStepper::new(composite)));
                    Either::Right(&mut self.inner)
                },
            }}
        };
        match internal {
            Either::Left(inference) => {
                self.current_step = next_step;
                self.proved.extend(inference.get_explicit_conclusions_owned());
                match validate_inference(inference.obj) {
                    Ok(_) => ProofValidityStepResultWrapper::unfinished_no_err(),
                    Err(err) => ProofValidityStepResultWrapper::unfinished_child_err(OwnedObjAtPath{obj: err, path: inference.path.clone()}),
                }
            }, Either::Right(Some(composite)) => {
                let cloned_value = composite.proof.clone();
                let inner_result = composite.step();
                if inner_result.is_finished {
                    self.current_step = next_step;
                    let conclusions = cloned_value.get_explicit_conclusions_owned();
                    self.proved.extend(conclusions);
                }
                match inner_result.next_result {
                    Ok(()) => ProofValidityStepResultWrapper::unfinished_no_err(),
                    Err(ProofValidityStepErr::InParent(err)) => ProofValidityStepResultWrapper::unfinished_child_err(err),
                    Err(ProofValidityStepErr::InChild(err)) => ProofValidityStepResultWrapper::unfinished_child_err(err),
                }
            }, Either::Right(None) => panic!("ProofStepper inner was supposed to guaranteed as Some(composite), but matched with None")
        }
    }

    fn check_conclusions_step(&mut self) -> ProofValidityStepResultWrapper<P,Rule::Err,ParentPath,JoinedPath> {
        let conclusions = HashPropSet1O::from_iter(self.proof.obj.get_explicit_conclusions_owned());
        let conclusions_not_found = &conclusions - &self.proved;
        let result = if conclusions_not_found.len() > 0 {
            ProofValidityStepResultWrapper::finished_parent_err(OwnedObjAtPath{
                obj: ProofValidityError::ConclusionsNotFound(conclusions_not_found), 
                path: self.proof.path.clone()
            })
        } else { ProofValidityStepResultWrapper::finished_no_err() };
        // Move to the next step
        self.current_step = self.next_step();
        // Return the result
        result
    }
}
