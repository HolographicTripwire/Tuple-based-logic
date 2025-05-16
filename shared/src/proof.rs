use crate::proposition::{Proposition, PropositionSet};

pub struct Proof<G: ProofGenerator<G>> {
    pub premises: Vec<Proposition>,
    pub subproofs: Vec<Subproof<G>>,
    pub conclusions: PropositionSet
}

pub enum Subproof<G: ProofGenerator<G>> {
    Atomic(ProofStep),
    Composite(Proof<G>),
    Generator(G,Vec<Proposition>)
}

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

pub struct ProofStep {
    pub step_type: ProofStepType,
    pub assumptions: Vec<Proposition>,
    pub conclusion: Proposition
}

pub trait ProofGenerator<G: ProofGenerator<G>> {
    fn generate(&self, conclusions: Vec<Proposition>) -> Result<Proof<G>,()>;
}
