pub mod inference_rules;
pub mod validation_error;

use std::collections::HashSet;

use inference_rules::*;
use tbl_structures::{proof::{error::{ErrorInProof, ResultInProof}, Proof, SubProof}, propositions::{Proposition, PropositionSet}};
use validation_error::ProofValidationError;

/// Verify that the provided proof is sound under Tuple-Based logic, given some set of starting assumptions
/// If the proof is not valid, return a [VerificationError] as well as the step that it occurred at
pub fn verify_proof(proof: &Proof, assumptions: &PropositionSet) -> Result<bool,ErrorInProof<ProofValidationError>> {
    validate_proof(proof)?;
    Ok(verify_proof_grounding(proof, assumptions))
}

/// Check that all of the premises of a given [Proof] are contained within some [PropositionSet]
/// Used to check the "grounding" of a proof - that is, are all of the proof's premises assumed to be true? If they are, we can trust the proof's conclusions
pub fn verify_proof_grounding(proof: &Proof, assumptions: &PropositionSet) -> bool {
    proof.premises().iter().all(|premise| assumptions.contains(premise))
}

/// 
fn validate_proof(proof: &Proof) -> Result<(),ErrorInProof<ProofValidationError>> {
    // Create a list of [Proposition] objects which are considered at this time to be true
    let mut proved = HashSet::<&Proposition>::from_iter(proof.premises());
    
    // Iterate through all steps in the proof
    for (i, subproof) in proof.subproofs().iter().enumerate() {
        // Throw an error if th assumptions of this step have not yet been proven
        let premises = HashSet::<&Proposition>::from_iter(subproof.premises());
        let assumptions_not_found = &proved - &premises;
        if assumptions_not_found.len() != 0 { return Err(ErrorInProof::here(ProofValidationError::AssumptionsNotFound(assumptions_not_found.into_iter().cloned().collect()))) }
        
        // Get the new propositions which have been proved by this step in the proof, assuming that the step is valid
        match subproof {
            SubProof::Atomic(proof_step) => ResultInProof::from(verify_rules_in_proof_step(proof_step)),
            SubProof::Composite(proof) => ResultInProof::from(validate_proof(proof)),
        }.resolve(i)?;
        
        // Add the new proved propositions to our set of proved propositions
        proved.extend(subproof.conclusions());
    }

    // Throw an error if the supposed conclusions of this proof have not been derived
    let conclusions_not_found = &HashSet::from_iter(proof.conclusions()) - &proved;
    if conclusions_not_found.len() != 0 { return Err(ErrorInProof::here(ProofValidationError::ConclusionsNotFound(conclusions_not_found.into_iter().cloned().collect())))}

    // 
    Ok(())
}
