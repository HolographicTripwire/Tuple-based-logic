use crate::{proofs::{errors::{ValidatableInferenceRule, get_proof_validity_errors, grounding::{get_proof_grounding_errors, verify_proof_grounding}, stepper::result::ProofValidityStepErr, verify_proof_validity}, sequential::{SequentialProof, subproofs::SequentialProofInProofPath}}, propositions::assigned::{Proposition, collections::sets::PropSet1O}};

pub enum ProofSoundnessError<P: Proposition, Err: Clone>{
    LacksGrounding(P),
    Invalid(ProofValidityStepErr<P,Err,(),SequentialProofInProofPath>)
}

/// Verify that the provided proof is sound under Tuple-Based logic, given some set of starting assumptions
/// If the proof is not valid, return a [VerificationError] as well as the step that it occurred at
pub fn verify_proof_soundness<'a, P: Proposition, PS: PropSet1O<P>, Rule: ValidatableInferenceRule<P>>(proof: &'a SequentialProof<P, Rule>, assumptions: &PS) -> Result<(),ProofSoundnessError<P,Rule::Err>> {
    verify_proof_grounding(proof,assumptions).map_err(|e| ProofSoundnessError::LacksGrounding(e.clone()))?;
    verify_proof_validity(proof).map_err(|e| ProofSoundnessError::Invalid(e))?;
    Ok(())
}

pub fn get_proof_soundness_errors<'a, P: Proposition, PS: PropSet1O<P>, Rule: ValidatableInferenceRule<P>>(proof: &'a SequentialProof<P,Rule>, assumptions: &PS) -> impl Iterator<Item = ProofSoundnessError<P,Rule::Err>> {
    get_proof_grounding_errors(proof, assumptions).map(|e| ProofSoundnessError::LacksGrounding(e.clone()))
        .chain(get_proof_validity_errors(proof).filter_map(|e| match e {
            Ok(_) => None,
            Err(e) => Some(ProofSoundnessError::Invalid(e)),
        }))
}
