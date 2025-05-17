pub mod production_rules;
pub mod verification_error;

use production_rules::*;
use shared::{proof::{error::ErrorInProof, Proof, ProofStep, SubProof}, proposition::{Proposition, PropositionSet}};
use verification_error::ProofVerificationError;

/// Verify that the provided proof is sound under Tuple-Based logic, given some set of starting assumptions
/// If the proof is not valid, return a [VerificationError] as well as the step that it occurred at
pub fn verify_proof(proof: &Proof, assumptions: &PropositionSet) -> Result<PropositionSet,ErrorInProof<ProofVerificationError>> {
    verify_rules_in_proof(proof)?;
    verify_propositions_in_proof(proof)?;
    
    // Create a list of [Proposition] objects which are considered at this time to be true
    let mut proved = assumptions.clone();
    // Iterate through all steps in the proof
    for (i, subproof) in proof.subproofs.iter().enumerate() {
        // Get the new propositions which have been proved by this step in the proof, assuming that the step is valid
        validate_proof(subproof);
        let new_proved = 
        // Add the new proved propositions to our set of proved propositions
        proved.merge(&new_proved);
    }
    
    // Throw an error if the supposed conclusions of this proof have not been derived
    let conclusions_not_found = proof.conclusions.subtracted(&proved);
    if conclusions_not_found.len() != 0 {
        let err = ProofVerificationError::ConclusionsNotFound(conclusions_not_found);
        return Err(ErrorInProof::new(proof.subproofs.len(), err))
    }
    
    // If all steps were verified, the proof is valid
    Ok(proof.conclusions.clone())
}

/// Verify that the provided proof step is valid under Tuple-Based Logic
/// If the proof is not valid, return a [VerificationError]
fn validate_proof_step(step: &ProofStep, assumptions: &PropositionSet) -> Result<(),ProofVerificationError> {
    // Check whether the step is a valid instance of the type of step it claims to be
    let step_result = verify_rules_in_proof_step(step);
    // If it's not a valid instance, throw an Error
    if let Err(err) = step_result { return Err(err) }
    
    // Check that all assumptions of this step are in the provided set of proved propositions
    let assumptions_not_found: Vec<Proposition> = step.assumptions
        .iter()
        .filter(|assumption| !assumptions.contains(assumption))
        .cloned()
        .collect();
    if assumptions_not_found.len() != 0 { return Err(ProofVerificationError::AssumptionsNotFound(PropositionSet::from(&assumptions_not_found))) }

    // If the step is valid, and the assumptions are present, then the step has been successfully verified
    Ok(())
}
