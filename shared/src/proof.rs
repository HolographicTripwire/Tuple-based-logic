use crate::proposition::Proposition;

pub struct Proof<G: ProofGenerator<G>> {
    pub premises: Vec<Proposition>,
    pub subproofs: Vec<Subproof<G>>,
    pub conclusions: Vec<Proposition>
}

pub enum Subproof<G: ProofGenerator<G>> {
    Atomic(ProofStep),
    Composite(Proof<G>),
    Generator(G)
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
    fn generate(conclusions: Vec<Proposition>) -> Result<Proof<G>,()>;
}
