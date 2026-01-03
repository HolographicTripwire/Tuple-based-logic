pub mod assertions;
pub mod errors;
pub mod inference_rules;

use std::collections::HashSet;

use path_lib::{obj_at_path::ObjAtPath, paths::PathSeries};
use tbl_structures::{expressions::{Proposition, PropositionSet}, proof::{Proof, ProofInProof, ProofStep, error::{ErrorInProof, ResultInProof}}};
use errors::ProofValidationError;

use crate::errors::specification_error::{verify_inference, VerifiableInferenceRule};

/// Verify that the provided proof is sound under Tuple-Based logic, given some set of starting assumptions
/// If the proof is not valid, return a [VerificationError] as well as the step that it occurred at
pub fn verify_proof<'a, E: Clone, Rule: VerifiableInferenceRule<E>>(proof: &'a Proof<Rule>, assumptions: &PropositionSet) -> Result<bool,ErrorInProof<ProofValidationError<E>>> {
    validate_proof(ProofInProof(ObjAtPath::from_at(proof,PathSeries::empty())))?;
    Ok(verify_proof_grounding(proof, assumptions))
}

/// Check that all of the premises of a given [Proof] are contained within some [PropositionSet]
/// Used to check the "grounding" of a proof - that is, are all of the proof's premises assumed to be true? If they are, we can trust the proof's conclusions
pub fn verify_proof_grounding<E, Rule: VerifiableInferenceRule<E>>(proof: &Proof<Rule>, assumptions: &PropositionSet) -> bool {
    proof.get_assumptions().into_iter().all(|premise| assumptions.contains(premise))
}

/// Check if a proof is valid. If not, return the first [ProofValidationError]
fn validate_proof<'a,E: Clone, Rule: VerifiableInferenceRule<E>>(proof: ProofInProof<Rule>) -> Result<(),ErrorInProof<ProofValidationError<E>>> {
    // Create a list of [Proposition] objects which are considered at this time to be true
    let mut proved = HashSet::<Proposition>::from_iter(proof.0.obj().get_assumptions_owned());
    
    // Iterate through all steps in the proof
    for (i, subproof) in proof.0.obj().get_located_immediate_subproofs() // Get steps
    .into_iter()
    .map(|o| ProofInProof(o.replace_path(|p| PathSeries::new([p])))) // Convert to [ProofInProof]
    .enumerate() {
        // Throw an error if th assumptions of this step have not yet been proven
        let premises = HashSet::from_iter(subproof.0.obj().get_assumptions_owned());
        let assumptions_not_found = &proved - &premises;
        if assumptions_not_found.len() != 0 { return Err(ErrorInProof::here(ProofValidationError::AssumptionsNotFound(assumptions_not_found))) }
        
        // Get the new propositions which have been proved by this step in the proof, assuming that the step is valid
        match subproof.clone().try_into() {
            Ok(inference) => ResultInProof::from(verify_inference(&inference)),
            Err(proof) => {
                let result_or_error: Result<(),ErrorInProof<ProofValidationError>> = proof.0.obj().get_located_immediate_subproofs().into_iter()
                    .map(|subproof| validate_proof(ProofInProof(subproof.replace_path(|p| PathSeries::new([p])))))
                    .collect();
                ResultInProof::from(result_or_error)
            }
        }.resolve(i)?;
        
        // Add the new proved propositions to our set of proved propositions
        let conclusions = subproof.0.obj().get_explicit_conclusions_owned().into_iter();
        proved.extend(conclusions);
    }

    // Throw an error if the supposed conclusions of this proof have not been derived
    let conclusions_not_found = &HashSet::from_iter(proof.0.obj().get_explicit_conclusions_owned()) - &proved;
    if conclusions_not_found.len() != 0 { return Err(ErrorInProof::here(ProofValidationError::ConclusionsNotFound(conclusions_not_found)))}

    // If no errors were thrown, the proof was valid
    Ok(())
}
