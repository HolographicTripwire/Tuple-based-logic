pub mod production_rules;

use std::collections::HashSet;

use production_rules::*;
use shared::{proof::{Proof, ProofStep}, proposition::Proposition};

/// Verify that the provided proof is sound under Tuple-Based logic, given some set of starting assumptions
/// If the proof is not valid, return a [VerificationError] as well as the step that it occurred at
pub fn verify_proof(proof: &Proof, assumptions: &HashSet<Proposition>) -> Result<(),(usize,VerificationError)> {
    // Create a list of [Proposition] objects which are considered at this time to be true
    let proved = assumptions.clone();
    // Verify each step of the process, throwing an error if any of them fail
    for (i, step) in proof.steps.iter().enumerate() {
        if let Err(err) = verify_proof_step(step, &proved)
            { return Err((i,err)) }
    }
    // If all steps were verified, the proof is valid
    Ok(())
}

/// Verify that the provided proof step is sound under Tuple-Based Logic, given some set of existing assumptions
/// If the proof is not valid, return a [VerificationError]
fn verify_proof_step(step: &ProofStep, assumptions: &HashSet<Proposition>) -> Result<(),VerificationError> {
    // Check whether the step is a valid instance of the type of step it claims to be
    let step_result = verify_proof_step_by_type(&step.step_type, &step.assumptions, &step.conclusion);
    // If it's not a valid instance, throw an Error
    if let Err(err) = step_result { return Err(err) }
    
    // Check that all assumptions of this step are in the provided set of proved propositions
    let mut assumptions_not_found = step.assumptions
        .iter()
        .filter(|assumption| !assumptions.contains(assumption))
        .peekable();
    if assumptions_not_found.peek().is_some() { return Err(VerificationError::AssumptionsNotFound(assumptions_not_found.cloned().collect())) }

    // If the step is valid, and the assumptions are present, then the step has been successfully verified
    Ok(())
}

pub enum VerificationError {
    AssumptionsNotFound(Vec<Proposition>),
    InvalidStepSpecification,
}