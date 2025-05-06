pub mod deduction_rules;

use std::collections::HashSet;

use deduction_rules::*;
use shared::{proof::{Proof, ProofStep, ProofStepType}, proposition::Proposition};

pub fn verify(axioms: &HashSet<Proposition>, proof: &Proof) -> Result<(),(usize,VerificationError)> {
    // Create a list of [Proposition] objects which are considered at this time to be true
    let proved = axioms.clone();
    // Verify each step of the process, throwing an error if any of them fail
    for (i, step) in proof.steps.iter().enumerate() {
        if let Err(err) = verify_proof_step(&proved, step)
            { return Err((i,err)) }
    }
    // If all steps were verified, the proof is valid
    Ok(())
}

fn verify_proof_step(assumptions: &HashSet<Proposition>, step: &ProofStep) -> Result<(),VerificationError> {
    // Check whether the step is a valid instance of the type of step it claims to be
    let step_result = match step.step_type {
        ProofStepType::ConjunctionIntroduction => verify_conjunction_introduction(&step.assumptions, &step.conclusion),
        ProofStepType::ImplicationElimination => verify_implication_elimination(&step.assumptions, &step.conclusion),
        ProofStepType::UniversalInstantiation => verify_universal_instantiation(&step.assumptions, &step.conclusion),
        ProofStepType::TupleAppendation => verify_tuple_appendation(&step.assumptions, &step.conclusion),
    };
    // If it's not a valid instance, throw an Error
    if let Err(err) = step_result { return Err(err) }
    
    // Check that all assumptions of this step are in the provided set of proved propositions
    let mut assumptions_not_found = step.assumptions
        .iter()
        .filter(|assumption| !assumptions.contains(assumption))
        .peekable();
    if assumptions_not_found.peek().is_some() { return Err(VerificationError::AssumptionsNotFound(assumptions_not_found.cloned().collect())) }

    // If the step is valid, and the assumptions are present, then the step has been successfully verified
    Ok(())
}

pub enum VerificationError {
    AssumptionsNotFound(Vec<Proposition>),
    InvalidStepSpecification,
}