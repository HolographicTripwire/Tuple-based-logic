use path_lib::HasChildren;

use crate::{expressions::Proposition, proof::{ProofPropositionPath, ProofStep}};

#[derive(Clone,PartialEq,Eq,Debug)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Rule:InferenceRule> {
    pub inference_type: Rule,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}

impl <'a, Rule:'a + InferenceRule> ProofStep<'a,Rule> for Inference<Rule> {
    fn assumptions(&self) -> &Vec<Proposition> { &self.assumptions }
    fn explicit_conclusions(&self) -> &Vec<Proposition> { &self.conclusions }
    fn subproofs(&'a self) -> impl IntoIterator<Item=&'a crate::proof::Proof<Rule>> { [] }
}
impl <'a, Rule:'a + InferenceRule> HasChildren<'a,ProofPropositionPath,Proposition> for Inference<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath> { self._valid_primitive_paths() }
    fn get_child(&'a self, path: &ProofPropositionPath) -> Result<&'a Proposition,()> { self._get_child(path) }
}

pub trait InferenceRule: Clone {}
impl <T: Clone> InferenceRule for T {}
