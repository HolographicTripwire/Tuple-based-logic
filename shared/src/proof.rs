use crate::proposition::Proposition;

pub struct Proof {
    pub steps: Vec<ProofStep>
}

pub enum ProofStepType {
    ConjunctionIntroduction,
    ImplicationElimination,
    UniversalInstantiation,
    TupleAppendation,
}

pub struct ProofStep {
    pub step_type: ProofStepType,
    pub assumptions: Vec<Proposition>,
    pub conclusion: Proposition
}
