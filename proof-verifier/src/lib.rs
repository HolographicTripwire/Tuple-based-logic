pub mod inference_rules;
pub mod validation_error;

use inference_rules::*;
use shared::{proof::{error::{ErrorInProof, ResultInProof}, Proof}, propositions::PropositionSet};
use validation_error::ProofValidationError;

/// Verify that the provided proof is sound under Tuple-Based logic, given some set of starting assumptions
/// If the proof is not valid, return a [VerificationError] as well as the step that it occurred at
pub fn verify_proof(proof: &Proof, assumptions: &PropositionSet) -> Result<bool,ErrorInProof<ProofValidationError>> {
    validate_proof(proof)?;
    Ok(verify_proof_grounding(proof, assumptions))
}

pub fn verify_proof_grounding(proof: &Proof, assumptions: &PropositionSet) -> bool {
    PropositionSet::from(&proof.premises).subset_of(assumptions)
}

fn validate_proof(proof: &Proof) -> Result<(),ErrorInProof<ProofValidationError>> {
    // Create a list of [Proposition] objects which are considered at this time to be true
    let mut proved = PropositionSet::from(&proof.premises.clone());
    
    // Iterate through all steps in the proof
    for (i, subproof) in proof.subproofs.iter().enumerate() {
        // Throw an error if th assumptions of this step have not yet been proven
        let assumptions_not_found = proved.subtracted(&PropositionSet::from(subproof.premises()));
        if assumptions_not_found.len() != 0 { return Err(ErrorInProof::here(ProofValidationError::AssumptionsNotFound(assumptions_not_found))) }
        
        // Get the new propositions which have been proved by this step in the proof, assuming that the step is valid
        match subproof {
            shared::proof::SubProof::Atomic(proof_step) => ResultInProof::from(verify_rules_in_proof_step(proof_step)),
            shared::proof::SubProof::Composite(proof) => ResultInProof::from(validate_proof(proof)),
        }.resolve(i)?;
        
        // Add the new proved propositions to our set of proved propositions
        proved.extend(subproof.conclusions());
    }

    // Throw an error if the supposed conclusions of this proof have not been derived
    let conclusions_not_found = PropositionSet::from(&proof.conclusions).subtracted(&proved);
    if conclusions_not_found.len() != 0 { return Err(ErrorInProof::here(ProofValidationError::ConclusionsNotFound(conclusions_not_found)))}

    // 
    Ok(())
}
