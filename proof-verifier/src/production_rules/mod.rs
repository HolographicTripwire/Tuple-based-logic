mod deduction;
mod verbatim;

use deduction::*;
use verbatim::*;

use shared::{proof::ProofStepType, proposition::Proposition, term::Term};


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

pub (self) struct TupleOrError;

impl TupleOrError {
    pub fn prop_as_tuple(proposition: &Proposition) -> Result<&Vec<Term>,VerificationError> {
        proposition.0.as_tuple().or(Err(VerificationError::InvalidStepSpecification))
    }

    pub fn term_as_tuple(term: &Term) -> Result<&Vec<Term>,VerificationError> {
        term.as_tuple().or(Err(VerificationError::InvalidStepSpecification))
    }

    pub fn prop_as_slice(proposition: &Proposition) -> Result<&[Term],VerificationError> {
        proposition.0.as_slice().or(Err(VerificationError::InvalidStepSpecification))
    }

    pub fn term_as_slice(term: &Term) -> Result<&[Term],VerificationError> {
        term.as_slice().or(Err(VerificationError::InvalidStepSpecification))
    }
}