use crate::term::Term;

pub struct Proof {
    pub steps: Vec<ProofStep>
}

pub enum ProofStepType {
    ConjunctionIntroduction,
    ImplicationElimination,
    UniversalSubstitution,
    TupleAppendation,
}

pub struct ProofStep {
    pub step_type: ProofStepType,
    pub assumptions: Vec<Term>,
    pub conclusion: Term
}
