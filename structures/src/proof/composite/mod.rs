use path_lib::{HasChildren, obj_at_path::OwnedObjAtPath};
use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{expressions::Proposition, proof::{AtomicProofInProofPath, Proof, ProofInProofPath, ProofStep, get_child_inner, inference::{InferenceRule, PropositionInInferencePath}, valid_primitive_paths_inner}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct CompositeProof<Rule: InferenceRule> {
    pub assumptions: Vec<Proposition>,
    pub subproofs: Vec<Proof<Rule>>,
    pub explicit_conclusions: Vec<Proposition>,
}
impl <Rule: InferenceRule> CompositeProof<Rule> {
    pub fn new(assumptions: Vec<Proposition>, subproofs: Vec<Proof<Rule>>, explicit_conclusions: Vec<Proposition>) -> Self
        { Self { assumptions, subproofs, explicit_conclusions } }
}
impl <Rule: InferenceRule> ProofStep<Rule> for CompositeProof<Rule> {
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>
        { (0..self.assumptions.len()).map(|n| PropositionInInferencePath::assumption(n)) }
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>
        { (0..self.explicit_conclusions.len()).map(|n| PropositionInInferencePath::conclusion(n)) }

    fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition> { &self.assumptions }
    fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition> { &self.explicit_conclusions }
    fn get_immediate_subproofs(&self) -> impl IntoIterator<Item=&Proof<Rule>>
        { <Self as HasChildren<AtomicProofInProofPath, Proof<Rule>>>::get_children(self) }
}
impl <Rule:InferenceRule> HasChildren<PropositionInInferencePath,Proposition> for CompositeProof<Rule> {
    fn valid_primitive_paths(&self) -> Vec<PropositionInInferencePath> { valid_primitive_paths_inner(self) }
    
    fn get_child(&self, path: &PropositionInInferencePath) -> Result<&Proposition,()> { get_child_inner(self,path) }
    fn get_child_owned(&self, path: &PropositionInInferencePath) -> Result<Proposition,()> where Proposition: Clone 
        { get_child_inner(self,path).cloned() }
        
    fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInInferencePath>> where Proposition: Clone, Self: Sized {
        let assumptions = self.assumptions
            .into_iter()
            .enumerate()
            .map(|(id,prop)| OwnedObjAtPath::from_inner(prop,PropositionInInferencePath::assumption(id)));
        let explicit_conclusions = self.explicit_conclusions.into_iter()
            .enumerate()
            .map(|(id, prop)| OwnedObjAtPath::from_inner(prop,PropositionInInferencePath::conclusion(id)));
        return assumptions.chain(explicit_conclusions)
    }
}

generate_obj_at_path_wrappers!{
    (CompositeProof<Rule> where Rule: InferenceRule), ProofInProofPath,
    "CompositeProofInProof", [Clone, PartialEq, Eq, Debug],
    "OwnedCompositeProofInProof", [Clone, PartialEq, Eq, Debug]
}

impl <Rule:InferenceRule> HasChildren<AtomicProofInProofPath,Proof<Rule>> for CompositeProof<Rule> {
    fn valid_primitive_paths(&self) -> Vec<AtomicProofInProofPath> 
        { (0..self.subproofs.len()).map(|ix| ix.into()).collect() }
    fn get_child(&self, path: &AtomicProofInProofPath) -> Result<&Proof<Rule>,()>
        { self.subproofs.get(path.0).ok_or(()) }
    fn get_child_owned(&self, path: &AtomicProofInProofPath) -> Result<Proof<Rule>,()> where Proof<Rule>: Clone
        { self.subproofs.get(path.0).ok_or(()).cloned() }
        
    fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicProofInProofPath>> where Proof<Rule>: Clone, Self: Sized {
        self.subproofs.into_iter()
            .enumerate()
            .map(|(id,proof)| OwnedObjAtPath::from_inner(proof, AtomicProofInProofPath(id)))
    }
}
