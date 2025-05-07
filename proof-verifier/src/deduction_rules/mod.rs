mod conjunction_introduction;
mod implication_elimination;
mod universal_instantiation;
mod tuple_appendation;

use conjunction_introduction::verify_conjunction_introduction;
use implication_elimination::verify_implication_elimination;
use shared::{proof::ProofStepType, proposition::Proposition};
use universal_instantiation::verify_universal_instantiation;
use tuple_appendation::verify_tuple_appendation;

use crate::VerificationError;

pub fn verify_proof_step_by_type(step_type: &ProofStepType, assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    let verifier = get_proof_step_verifier_by_type(step_type);
    verifier(assumptions, conclusion)
}

fn get_proof_step_verifier_by_type(step_type: &ProofStepType) -> impl Fn(&Vec<Proposition>, &Proposition) -> Result<(),VerificationError> {
    match step_type {
        ProofStepType::ConjunctionIntroduction => verify_conjunction_introduction,
        ProofStepType::ImplicationElimination => verify_implication_elimination,
        ProofStepType::UniversalInstantiation => verify_universal_instantiation,
        ProofStepType::TupleAppendation => verify_tuple_appendation,
    }
}
