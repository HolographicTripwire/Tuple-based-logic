mod deduction;
mod verbatim;

use deduction::*;
use verbatim::*;

use tbl_structures::{inference::{Inference, InferenceRule}, proof::{error::ErrorInProof, Proof, SubProof}, propositions::{Proposition,tuple_or_error::TupleOrError}};

use crate::ProofValidationError;

pub (self) const TUPLE_OR_ERROR: TupleOrError<ProofValidationError> = TupleOrError{ error: ProofValidationError::InvalidStepSpecification };

/// Check if all deduction rules in the proof are correct
pub fn verify_proof_rules(proof: &Proof) -> Result<(),ErrorInProof<ProofValidationError>> {
    // Iterate through all steps in the proof
    for (i, subproof) in proof.subproofs().iter().enumerate() {
        match subproof { 
            SubProof::Atomic(proof_step) => {
                // Verify that an atomic proof represents a step that correctly applies our production rules
                match verify_rules_in_proof_step(proof_step) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(ErrorInProof::<ProofValidationError>::at_substep( i,err)),
                }},
            SubProof::Composite(proof) => {
                // Verify that a composite proof is valid
                match verify_proof_rules(proof) {
                    Ok(()) => Ok(()),
                    Err(mut located_err) => {
                        located_err.push_step(i);
                        Err(located_err)
                    },
                }},
        }?
    }
    Ok(())
}

pub fn verify_rules_in_proof_step(step: &Inference) -> Result<(),ProofValidationError> {
    let verifier = get_proof_step_verifier_by_type(&step.inference_type);
    verifier(&step.assumptions, &step.conclusions)
}

fn get_proof_step_verifier_by_type(step_type: &InferenceRule) -> impl Fn(&Vec<Proposition>, &Vec<Proposition>) -> Result<(),ProofValidationError> {
    match step_type {
        // Deduction rules
        InferenceRule::ConjunctionIntroduction => verify_conjunction_introduction,
        InferenceRule::ImplicationElimination => verify_implication_elimination,
        InferenceRule::UniversalSubstitution => verify_universal_substitution,
        // Verbatim rules
        InferenceRule::AtomicityAssertion => verify_atomicity_assertion,
        InferenceRule::AtomDifferentiation => verify_atom_differentiation,
        InferenceRule::TupleAppendation => verify_tuple_appendation,
    }
}
