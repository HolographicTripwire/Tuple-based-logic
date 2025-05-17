pub mod generation;
pub mod error;

use crate::proposition::{Proposition};

#[derive(Clone)]
pub struct Proof {
    pub premises: Vec<Proposition>,
    pub subproofs: Vec<SubProof>,
    pub conclusions: Vec<Proposition>
}

#[derive(Clone)]
pub enum SubProof {
    Atomic(ProofStep),
    Composite(Proof)
}

impl SubProof {
    pub fn premises(&self) -> &Vec<Proposition> {
        match self {
            SubProof::Atomic(proof_step) => &proof_step.assumptions,
            SubProof::Composite(proof) => &proof.premises,
        }
    }

    pub fn conclusions(&self) -> &Vec<Proposition> {
        match self {
            SubProof::Atomic(proof_step) => &proof_step.conclusion,
            SubProof::Composite(proof) => &proof.conclusions,
        }
    }
}

#[derive(Clone)]
pub enum ProofStepType {
    // Deduction rules
    ConjunctionIntroduction,
    ImplicationElimination,
    UniversalSubstitution,
    // Verbatim rules
    AtomicityAssertion,
    AtomDifferentiation,
    TupleAppendation,
}

#[derive(Clone)]
pub struct ProofStep {
    pub step_type: ProofStepType,
    pub assumptions: Vec<Proposition>,
    pub conclusion: Vec<Proposition>
}
