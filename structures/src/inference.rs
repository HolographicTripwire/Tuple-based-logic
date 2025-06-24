use crate::propositions::Proposition;

#[derive(Clone)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Rules: InferenceRules> {
    pub inference_type: Rules,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}

pub trait InferenceRules: Clone {}

