use crate::propositions::Proposition;

pub mod path;

#[derive(Clone,PartialEq,Eq,Debug)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Rule:InferenceRule> {
    pub inference_type: Rule,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}

impl <Rule:InferenceRule> Inference<Rule> {
    pub fn assumptions(&self) -> &Vec<Proposition> { &self.assumptions }
    pub fn conclusions(&self) -> &Vec<Proposition> { &self.conclusions }
}

pub trait InferenceRule: Clone {}
impl <T: Clone> InferenceRule for T {}
