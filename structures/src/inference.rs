use crate::propositions::Proposition;

#[derive(Clone)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
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
