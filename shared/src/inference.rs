use crate::proposition::Proposition;


#[derive(Clone)]
pub struct Inference {
    pub inference_type: InferenceRule,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}

#[derive(Clone)]
pub enum InferenceRule {
    // Deduction rules
    ConjunctionIntroduction,
    ImplicationElimination,
    UniversalSubstitution,
    // Verbatim rules
    AtomicityAssertion,
    AtomDifferentiation,
    TupleAppendation,
}
