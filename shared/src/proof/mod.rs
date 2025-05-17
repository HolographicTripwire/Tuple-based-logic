pub mod generation;

use crate::proposition::{Proposition, PropositionSet};

#[derive(Clone)]
pub struct Proof {
    pub premises: Vec<Proposition>,
    pub subproofs: Vec<SubProof>,
    pub conclusions: PropositionSet
}

#[derive(Clone)]
pub enum SubProof {
    Atomic(ProofStep),
    Composite(Proof)
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
    pub conclusion: Proposition
}
