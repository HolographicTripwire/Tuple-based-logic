mod deduction;
mod verbatim;
mod tuple_or_error;

use deduction::*;
use verbatim::*;

use shared::{proof::{error::ErrorInProof, Proof, ProofStep, ProofStepType, SubProof}, proposition::Proposition};

use crate::ProofValidationError;

/// Check if all deduction rules in the proof are correct
pub fn verify_proof_rules(proof: &Proof) -> Result<(),ErrorInProof<ProofValidationError>> {
    // Iterate through all steps in the proof
    for (i, subproof) in proof.subproofs.iter().enumerate() {
        match subproof { 
            SubProof::Atomic(proof_step) => {
                // Verify that an atomic proof represents a step that correctly applies our production rules
                match verify_rules_in_proof_step(proof_step) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(ErrorInProof::<ProofValidationError>::new( i,err)),
                }},
            SubProof::Composite(proof) => {
                // Verify that a composite proof is valid
                match verify_proof_rules(proof) {
                    Ok(()) => Ok(()),
                    Err(located_err) => Err(located_err.add_step(i)),
                }},
        }?
    }
    Ok(())
}

pub fn verify_rules_in_proof_step(step: &ProofStep) -> Result<(),ProofValidationError> {
    let verifier = get_proof_step_verifier_by_type(&step.step_type);
    verifier(&step.assumptions, &step.conclusions)
}

fn get_proof_step_verifier_by_type(step_type: &ProofStepType) -> impl Fn(&Vec<Proposition>, &Vec<Proposition>) -> Result<(),ProofValidationError> {
    match step_type {
        // Deduction rules
        ProofStepType::ConjunctionIntroduction => verify_conjunction_introduction,
        ProofStepType::ImplicationElimination => verify_implication_elimination,
        ProofStepType::UniversalSubstitution => verify_universal_substitution,
        // Verbatim rules
        ProofStepType::AtomicityAssertion => verify_atomicity_assertion,
        ProofStepType::AtomDifferentiation => verify_atom_differentiation,
        ProofStepType::TupleAppendation => verify_tuple_appendation,
    }
}
