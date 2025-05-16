use crate::proposition::Proposition;

pub struct Proof<'a> {
    pub premises: Vec<Proposition>,
    pub subproofs: Vec<Subproof<'a>>,
    pub conclusions: Vec<Proposition>
}

pub enum Subproof<'a> {
    Atomic(ProofStep),
    Composite(Proof<'a>),
    Generator(&'a dyn Fn(Vec<Proposition>) -> Proof<'a>)
}

pub enum ProofStepType {
    ConjunctionIntroduction,
    ImplicationElimination,
    UniversalSubstitution,
    TupleAppendation,
}

pub struct ProofStep {
    pub step_type: ProofStepType,
    pub assumptions: Vec<Proposition>,
    pub conclusion: Proposition
}
