use std::marker::PhantomData;

use itertools::Either;
use tbl_structures::{expressions::PropositionSet, proof::{AtomicProofInProofPath, ProofStep, SplitProofInProof, composite::CompositeProofInProof, error::OwnedErrorInProof, inference::InferenceInProof}};

use crate::validity::{ProofValidityError, VerifiableInferenceRule, verify_inference};

enum ProofValidityStep {
    CheckAssumptionsFound(usize),
    CheckInternalValidity(usize),
    CheckConclusionsFound,
    Finished
}

struct ProofValidityStepResult<E:Clone> {
    is_finished: bool,
    next_result: Result<(),OwnedErrorInProof<ProofValidityError<E>>>
}
impl <E:Clone> ProofValidityStepResult<E> {
    fn unfinished_no_err() -> Self { Self {
        is_finished: false,
        next_result: Ok(())
    }}
    fn finished_no_err() -> Self { Self {
        is_finished: true,
        next_result: Ok(())
    }}
    fn unfinished_err(err: OwnedErrorInProof<ProofValidityError<E>>) -> Self { Self {
        is_finished: false,
        next_result: Err(err)
    }}
    fn finished_err(err: OwnedErrorInProof<ProofValidityError<E>>) -> Self { Self {
        is_finished: true,
        next_result: Err(err)
    }}
}

struct ProofValidityStepper<'a,E:Clone,Rule:VerifiableInferenceRule<E>> {
    proof: CompositeProofInProof<'a,Rule>,
    step_count: usize,
    
    proved: PropositionSet,
    
    current_step: ProofValidityStep,
    inner: Option<Box<ProofValidityStepper<'a,E,Rule>>>,
    
    phantom: PhantomData<E>
}

impl <'a,E:Clone,Rule:VerifiableInferenceRule<E>> ProofValidityStepper<'a,E,Rule> {
    fn new(proof: CompositeProofInProof<'a,Rule>) -> Self {
        let proof_obj = proof.obj();
        Self {
            current_step: ProofValidityStep::CheckAssumptionsFound(0),
            proved: PropositionSet::from_iter(proof_obj.get_assumptions_owned()),
            step_count: proof_obj.get_immediate_subproofs().into_iter().count(),
            proof,
            inner: None,
            phantom: PhantomData
        }
    }


    pub fn step(&'a mut self) -> ProofValidityStepResult<E> {
        match self.current_step {
            ProofValidityStep::CheckAssumptionsFound(step_number) => self.check_assumptions_step(step_number),
            ProofValidityStep::CheckInternalValidity(step_number) => self.internal_validation_step(step_number),
            ProofValidityStep::CheckConclusionsFound => self.check_conclusions_step(),
            ProofValidityStep::Finished => return ProofValidityStepResult::finished_no_err()
        }
    }

    fn next_step(&self) -> ProofValidityStep {
        match self.current_step {
            ProofValidityStep::CheckAssumptionsFound(step_no) => ProofValidityStep::CheckInternalValidity(step_no),
            ProofValidityStep::CheckInternalValidity(step_no) => 
                if step_no >= self.step_count {ProofValidityStep::CheckConclusionsFound}
                else {ProofValidityStep::CheckAssumptionsFound(step_no + 1)},
            ProofValidityStep::CheckConclusionsFound => ProofValidityStep::Finished,
            ProofValidityStep::Finished => ProofValidityStep::Finished,
        }
    }

    fn check_assumptions_step(&mut self, step_number: usize) -> ProofValidityStepResult<E> {
        let subproof = self.proof.get_located_immediate_subproof(AtomicProofInProofPath(step_number))
            .expect("Attempted to call get_subproof when step was not within range");
        let premises = PropositionSet::from_iter(subproof.obj().get_assumptions_owned());
        // Determine if an error is present
        let assumptions_not_found = &self.proved - &premises;
        let result = if assumptions_not_found.len() > 0 {
            ProofValidityStepResult::unfinished_err(OwnedErrorInProof::from_inner(
                ProofValidityError::AssumptionsNotFound(assumptions_not_found),
                subproof.path().clone()
            ))
        } else { ProofValidityStepResult::unfinished_no_err() };
        // Move to the next step
        self.current_step = self.next_step();
        // Return the result
        result
    }

    fn internal_validation_step(&'a mut self, step_number: usize) -> ProofValidityStepResult<E> {
        let next_step = self.next_step();
        // Get internal value
        let internal = {
            if self.inner.is_some() { Either::Right(&mut self.inner)
            } else { match self.proof.get_located_immediate_subproof(AtomicProofInProofPath(step_number))
                    .expect("Attempted to call get_subproof when step was not within range")
                    .into() {
                SplitProofInProof::Inference(inference) => Either::Left(inference),
                SplitProofInProof::Composite(composite) => {
                    self.inner = Some(Box::new(ProofValidityStepper::new(composite)));
                    Either::Right(&mut self.inner)
                },
            }}
        };
        let mut result = match internal {
            Either::Left(inference) => match verify_inference(inference.obj()) {
                Ok(_) => ProofValidityStepResult::finished_no_err(),
                Err(err) => ProofValidityStepResult::finished_err(OwnedErrorInProof::from_inner(err, inference.path().clone())),
            },
            Either::Right(inner) => {
                let composite = match inner {
                    Some(composite) => composite,
                    None => panic!("ProofStepper inner was supposed to guaranteed as Some(composite), but matched with None"),
                };
                composite.step()
            },
        };
        if result.is_finished {
            self.current_step = next_step;
            result.is_finished = false;
            let new_props = match internal {
                Either::Left(x) => x.get_explicit_conclusions_owned(),
                Either::Right(Some(x)) => x.proof.get_,
                Either::Right(None) => panic!("ProofStepper inner was supposed to guaranteed as Some(composite), but matched with None")
            };
            self.proved = self.proved.union(internal);
        }
        result
    }

    fn check_conclusions_step(&mut self) -> ProofValidityStepResult<E> {
        let conclusions = PropositionSet::from_iter(self.proof.obj().get_explicit_conclusions_owned());
        let conclusions_not_found = &conclusions - &self.proved;
        let result = if conclusions_not_found.len() > 0 {
            ProofValidityStepResult::finished_err(OwnedErrorInProof::from_inner(
                ProofValidityError::ConclusionsNotFound(conclusions_not_found), 
                self.proof.path().clone()
            ))
        } else { ProofValidityStepResult::finished_no_err() };
        // Move to the next step
        self.current_step = self.next_step();
        // Return the result
        result
    }
}
