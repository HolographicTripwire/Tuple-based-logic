mod deduction;
mod verbatim;

use deduction::*;
use verbatim::*;

use shared::{proof::ProofStepType, proposition::Proposition};


use crate::VerificationError;

pub fn verify_proof_step_by_type(step_type: &ProofStepType, assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    let verifier = get_proof_step_verifier_by_type(step_type);
    verifier(assumptions, conclusion)
}

fn get_proof_step_verifier_by_type(step_type: &ProofStepType) -> impl Fn(&Vec<Proposition>, &Proposition) -> Result<(),VerificationError> {
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
