mod utils;
mod proof_step;
mod expression;
mod proposition;

use std::collections::HashSet;

use path_lib::paths::PathSeries;
pub use proof_step::*;
pub use expression::*;
pub use proposition::*;

use genawaiter::{rc::{Gen, r#gen}, sync::GenBoxed, yield_};

use tbl_structures::{expressions::{Proposition, PropositionSet}, inference::{Inference, InferenceRule}, proof::{Proof, ProofInProof, ProofStep, error::OwnedErrorInProof}};

pub fn verify_inference<Err, Rule: VerifiableInferenceRule<Err>>(inference: &Inference<Rule>) -> Result<(),ProofValidityError<Err>> {
    Rule::verify(inference)
        .map_err(|err| ProofValidityError::InvalidInference(err))
}

pub trait VerifiableInferenceRule<Err>: InferenceRule {
    fn verify(rule: &Inference<Self>) -> Result<(),Err>;
}

#[derive(Clone)]
pub enum ProofValidityError<InferenceErr> {
    AssumptionsNotFound(PropositionSet),
    ConclusionsNotFound(PropositionSet),
    InvalidInference(InferenceErr),
}

pub fn verify_proof_validity<'a, E: Clone, Rule: VerifiableInferenceRule<E>>(proof: &'a Proof<Rule>) -> Result<(),OwnedErrorInProof<ProofValidityError<E>>> {
    todo!{}
    //proof_validity_helper(&ProofInProof::from_inner(proof,PathSeries::empty()))
}
pub fn get_proof_validity_errors<'a, E: Clone, Rule: VerifiableInferenceRule<E>>(proof: &'a Proof<Rule>) -> impl Iterator<Item = OwnedErrorInProof<ProofValidityError<E>>> {
    todo!{}
    //proof_validity_helper(&ProofInProof::from_inner(proof,PathSeries::empty()))
}

/// Check if a proof is valid. If not, return the first [ProofValidationError]
fn proof_validity_helper<'a,E: Clone, Rule: VerifiableInferenceRule<E>>(proof: &'a ProofInProof<Rule>) -> GenBoxed<OwnedErrorInProof<ProofValidityError<E>>,_,_> {
    GenBoxed::new(|proof| {
        // Create a list of [Proposition] objects which are considered at this time to be true
        let mut proved = HashSet::<Proposition>::from_iter(proof.obj().get_assumptions_owned());
        
        // Iterate through all steps in the proof
        for (_, subproof) in proof.obj().get_located_immediate_subproofs() // Get steps
            .into_iter()
            .map(|o| ProofInProof::from(o.replace_path(|p| PathSeries::new([p])))) // Convert to [ProofInProof]
            .enumerate() {
            // Throw an error if the assumptions of this step have not yet been proven
            let premises = HashSet::from_iter(subproof.obj().get_assumptions_owned());
            let assumptions_not_found = &proved - &premises;
            if assumptions_not_found.len() != 0 { yield_!(OwnedErrorInProof::from_inner(
                ProofValidityError::AssumptionsNotFound(assumptions_not_found),
                proof.path().clone()
            )) }
            
            // Get the new propositions which have been proved by this step in the proof, assuming that the step is valid
            match subproof.obj() {
                Proof::Atomic(inference) => verify_inference(&inference)
                    .map_err(|e| OwnedErrorInProof::from_inner(e, subproof.path().clone())),
                Proof::Composite(_) => {
                    proof.obj().get_located_immediate_subproofs().into_iter()
                        .map(|subproof| proof_validity_helper(&ProofInProof::from(subproof.replace_path(|p| PathSeries::new([p])))))
                        .collect()
                },
            }?;
            
            // Add the new proved propositions to our set of proved propositions
            let conclusions = subproof.obj().get_explicit_conclusions_owned().into_iter();
            proved.extend(conclusions);
        }

        // Throw an error if the supposed conclusions of this proof have not been derived
        let conclusions_not_found = &HashSet::from_iter(proof.obj().get_explicit_conclusions_owned()) - &proved;
        if conclusions_not_found.len() != 0 { yield_!(OwnedErrorInProof::from_inner(ProofValidityError::ConclusionsNotFound(conclusions_not_found), proof.path().clone())); }
    })
}
