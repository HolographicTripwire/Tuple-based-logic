mod deduction;
mod verbatim;
mod tuple_or_error;

use deduction::*;
use verbatim::*;

use shared::{proof::{ProofStep, ProofStepType}, proposition::Proposition, term::Term};


use crate::ProofVerificationError;

pub fn verify_rules_in_proof_step(step: &ProofStep) -> Result<(),ProofVerificationError> {
    let verifier = get_proof_step_verifier_by_type(&step.step_type);
    verifier(&step.assumptions, &step.conclusion)
}

fn get_proof_step_verifier_by_type(step_type: &ProofStepType) -> impl Fn(&Vec<Proposition>, &Proposition) -> Result<(),ProofVerificationError> {
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
