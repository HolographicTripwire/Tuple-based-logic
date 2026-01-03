use path_lib::{HasChildren, obj_at_path::OwnedObjAtPath};

use crate::{expressions::Proposition, proof::{AtomicSubproofPath, Proof, ProofStep, PropositionInInferencePath, get_child_inner, valid_primitive_paths_inner}};

#[derive(Clone,PartialEq,Eq,Debug)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Rule:InferenceRule> {
    pub inference_type: Rule,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}

impl <Rule:InferenceRule> ProofStep<Rule> for Inference<Rule> {
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>
        { (0..self.assumptions.len()).map(|n| PropositionInInferencePath::assumption(n)) }
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>
        { (0..self.conclusions.len()).map(|n| PropositionInInferencePath::conclusion(n)) }

    // Faster versions of default members
    fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition> { &self.assumptions }
    fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition> { &self.conclusions }
}
impl <Rule:InferenceRule> HasChildren<PropositionInInferencePath,Proposition> for Inference<Rule> {
    fn valid_primitive_paths(&self) -> Vec<PropositionInInferencePath> { valid_primitive_paths_inner(self) }
    fn get_child(&self, path: &PropositionInInferencePath) -> Result<&Proposition,()> { get_child_inner(self,path) }
    
    fn get_child_owned(&self, path: &PropositionInInferencePath) -> Result<Proposition,()> where Proposition: Clone
        { get_child_inner(self,path).cloned() }
        
    fn to_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInInferencePath>> where Proposition: Clone, Self: Sized {
        let assumptions = self.assumptions.into_iter()
            .enumerate()
            .map(|(id,conclusion)| OwnedObjAtPath::from_at(conclusion, PropositionInInferencePath::assumption(id)));
        let conclusions = self.conclusions.into_iter()
            .enumerate()
            .map(|(id,conclusion)| OwnedObjAtPath::from_at(conclusion, PropositionInInferencePath::conclusion(id)));
        assumptions.chain(conclusions)
    }
}
impl <Rule: InferenceRule> HasChildren<AtomicSubproofPath,Proof<Rule>> for Inference<Rule> {
    fn valid_primitive_paths(&self) -> Vec<AtomicSubproofPath> { vec![] }
    fn get_child(&self, _: &AtomicSubproofPath) -> Result<&Proof<Rule>,()> { Err(()) }
    fn get_child_owned(&self, _: &AtomicSubproofPath) -> Result<Proof<Rule>,()> { Err(()) }
    
    fn to_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicSubproofPath>> where Proof<Rule>: Clone, Self: Sized
        { vec![] }
}

pub trait InferenceRule: 'static + Clone + PartialEq {}
