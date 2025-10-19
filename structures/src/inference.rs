use path_lib::HasChildren;

use crate::{expressions::Proposition, proof::{get_child_inner, valid_primitive_paths_inner, AtomicSubproofPath, Proof, PropositionInProofStepPath, ProofStep}};

#[derive(Clone,PartialEq,Eq,Debug)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Rule:InferenceRule> {
    pub inference_type: Rule,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}

impl <'a, Rule:'a + InferenceRule> ProofStep<'a,Rule> for Inference<Rule> {
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInProofStepPath>
        { (0..self.assumptions.len()).map(|n| PropositionInProofStepPath::assumption(n)) }
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInProofStepPath>
        { (0..self.conclusions.len()).map(|n| PropositionInProofStepPath::conclusion(n)) }

    // Faster versions of default members
    fn get_assumptions(&'a self) -> impl IntoIterator<Item = &'a Proposition> { &self.assumptions }
    fn get_explicit_conclusions(&'a self) -> impl IntoIterator<Item = &'a Proposition> { &self.conclusions }
}
impl <'a, Rule:'a + InferenceRule> HasChildren<'a,PropositionInProofStepPath,Proposition> for Inference<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = PropositionInProofStepPath> { valid_primitive_paths_inner(self) }
    fn get_child(&'a self, path: &PropositionInProofStepPath) -> Result<&'a Proposition,()> { get_child_inner(self,path) }
    
    fn get_child_owned(&self, path: &PropositionInProofStepPath) -> Result<Proposition,()> where Proposition: Clone
        { get_child_inner(self,path).cloned() }
}
impl <'a, Rule: 'a + InferenceRule> HasChildren<'a,AtomicSubproofPath,Proof<Rule>> for Inference<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = AtomicSubproofPath> { [] }
    fn get_child(&'a self, _: &AtomicSubproofPath) -> Result<&'a Proof<Rule>,()> { Err(()) }
    fn get_child_owned(&self, _: &AtomicSubproofPath) -> Result<Proof<Rule>,()> { Err(()) }
}

pub trait InferenceRule: 'static + Clone + PartialEq {}
