pub mod production_rules;

use production_rules::*;
use shared::{proof::{Proof, ProofGenerator, ProofStep, Subproof}, proposition::{Proposition, PropositionSet}};

/// Verify that the provided proof is sound under Tuple-Based logic, given some set of starting assumptions
/// If the proof is not valid, return a [VerificationError] as well as the step that it occurred at
pub fn verify_proof<G: ProofGenerator<G>>(proof: &Proof<G>, assumptions: &PropositionSet) -> Result<PropositionSet,LocatedVerificationError> {
    // Create a list of [Proposition] objects which are considered at this time to be true
    let mut proved = assumptions.clone();
    // Verify each step of the proof, throwing an error if any of them fail
    for (i, step) in proof.subproofs.iter().enumerate() {
        let new_proved = match step {
            // Verify that an atomic proof is a correct instance of the 
            Subproof::Atomic(proof_step) => {
                match verify_proof_step(proof_step, &proved) {
                    Ok(prop_set) => Ok(prop_set),
                    Err(err) => Err(LocatedVerificationError::new( i,err)),
                }},
            Subproof::Composite(proof) => {
                match verify_proof(proof, &proved) {
                    Ok(prop_set) => Ok(prop_set),
                    Err(located_err) => Err(located_err.add_step(i)),
                }},
            Subproof::Generator(generator,propositions) => {
                match generator.generate(propositions) {
                    Ok(proof) => { match verify_proof(&proof, &proved) {
                        Ok(prop_set) => Ok(prop_set),
                        Err(located_err) => Err(located_err.add_step(i)),
                    }},
                    Err(_) => Err(LocatedVerificationError::new(i, VerificationError::FailedToGenerateProof)),
                }
            },
        }?;
        proved.merge(&new_proved);
    }
    
    let conclusions_not_found = proof.conclusions.subtracted(&proved);
    if conclusions_not_found.len() != 0 {
        let err = VerificationError::ConclusionsNotFound(conclusions_not_found);
        return Err(LocatedVerificationError::new(proof.subproofs.len(), err))
    }
    
    // If all steps were verified, the proof is valid
    Ok(proof.conclusions.clone())
}

/// Verify that the provided proof step is sound under Tuple-Based Logic, given some set of existing assumptions
/// If the proof is not valid, return a [VerificationError]
fn verify_proof_step(step: &ProofStep, assumptions: &PropositionSet) -> Result<PropositionSet,VerificationError> {
    // Check whether the step is a valid instance of the type of step it claims to be
    let step_result = verify_proof_step_by_type(&step.step_type, &step.assumptions, &step.conclusion);
    // If it's not a valid instance, throw an Error
    if let Err(err) = step_result { return Err(err) }
    
    // Check that all assumptions of this step are in the provided set of proved propositions
    let assumptions_not_found: Vec<Proposition> = step.assumptions
        .iter()
        .filter(|assumption| !assumptions.contains(assumption))
        .cloned()
        .collect();
    if assumptions_not_found.len() != 0 { return Err(VerificationError::AssumptionsNotFound(PropositionSet::from(&assumptions_not_found))) }

    // If the step is valid, and the assumptions are present, then the step has been successfully verified
    Ok(PropositionSet::from(&step.conclusion))
}

#[derive(Clone)]
pub enum VerificationError {
    AssumptionsNotFound(PropositionSet),
    ConclusionsNotFound(PropositionSet),
    InvalidStepSpecification,
    FailedToGenerateProof,
}

struct LocatedVerificationError(Vec<usize>,VerificationError);

impl LocatedVerificationError {
    pub fn new(step: usize, err: VerificationError) -> Self {
        Self(vec![step],err)
    }

    pub fn add_step(&self, step: usize) -> Self {
        let mut steps = self.0.clone();
        steps.push(step);
        Self(steps,self.1.clone())
    }

    pub fn location(&self) -> Vec<usize> {
        self.0.iter().copied().rev().collect()
    }

    pub fn err(&self) -> VerificationError { self.1.clone() }
}
