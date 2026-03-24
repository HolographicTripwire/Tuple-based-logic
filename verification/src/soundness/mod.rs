
use crate::{grounding::{get_proof_grounding_errors, verify_proof_grounding}, validity::{ProofValidityError, VerifiableInferenceRule, get_proof_validity_errors, verify_proof_validity}};

enum ProofSoundnessError<E: Clone>{
    LacksGrounding(TblProposition),
    InvalidInference(OwnedErrorInProof<ProofValidityError<E>>)
}

/// Verify that the provided proof is sound under Tuple-Based logic, given some set of starting assumptions
/// If the proof is not valid, return a [VerificationError] as well as the step that it occurred at
pub fn verify_proof_soundness<'a, E: Clone, Rule: VerifiableInferenceRule<E>>(proof: &'a Proof<Rule>, assumptions: &TblPropSet) -> Result<(),ProofSoundnessError<E>> {
    verify_proof_grounding(proof,assumptions).map_err(|e| ProofSoundnessError::LacksGrounding(e.clone()))?;
    verify_proof_validity(proof).map_err(|e| ProofSoundnessError::InvalidInference(e))?;
    Ok(())
}

pub fn get_proof_soundness_errors<'a, E: Clone, Rule: VerifiableInferenceRule<E>>(proof: &'a Proof<Rule>, assumptions: &TblPropSet) -> impl Iterator<Item = ProofSoundnessError<E>> {
    get_proof_grounding_errors(proof, assumptions).map(|e| ProofSoundnessError::LacksGrounding(e.clone()))
        .chain(get_proof_validity_errors(proof).map(|e| ProofSoundnessError::InvalidInference(e)))
}
