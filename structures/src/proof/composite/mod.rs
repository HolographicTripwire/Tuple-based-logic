use std::collections::HashSet;

use path_lib::{HasChildren, obj_at_path::OwnedObjAtPath};
use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{expressions::{Proposition, PropositionSet}, proof::{AtomicProofInProofPath, ImmediateProofInProof, OwnedImmediateProofInProof, Proof, ProofInProofPath, ProofStep, get_child_inner, inference::{InferenceRule, PropositionInInferencePath}, valid_primitive_paths_inner}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct CompositeProof<Rule: InferenceRule> {
    pub assumptions: Vec<Proposition>,
    pub subproofs: Vec<Proof<Rule>>,
    pub explicit_conclusions: Vec<Proposition>,
}
impl <Rule: InferenceRule> CompositeProof<Rule> {
    pub fn new(assumptions: Vec<Proposition>, subproofs: Vec<Proof<Rule>>, explicit_conclusions: Vec<Proposition>) -> Self
        { Self { assumptions, subproofs, explicit_conclusions } }
    /// Get the [AtomicSubproofPaths](AtomicSubproofPath) of all immediate subproofs within this [ProofStep]
    pub fn immediate_subproof_paths<'a>(&'a self) -> impl IntoIterator<Item=AtomicProofInProofPath> { <Self as HasChildren<AtomicProofInProofPath,Proof<Rule>>>::valid_primitive_paths(self) }
    
    /// Get all immediate subproofs within this [ProofStep]
    pub fn get_immediate_subproofs(&self) -> impl IntoIterator<Item = &Proof<Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_child(&p).unwrap()) }
    /// Get owned versions of all immediate subproofs within this [ProofStep]
    pub fn get_immediate_subproofs_owned(&self) -> impl IntoIterator<Item = Proof<Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_child(&p).unwrap().to_owned()) }
    /// Get all immediate subproofs within this [ProofStep], located by their [AtomicSubproofPath]
    pub fn get_located_immediate_subproofs<'a>(&'a self) -> impl IntoIterator<Item = ImmediateProofInProof<'a,Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_located_child(p).unwrap().into()) }
    /// Get owned versions of all immediate subproofs within this [ProofStep], located by their [AtomicSubproofPath]
    pub fn get_located_immediate_subproofs_owned(&self) -> impl IntoIterator<Item = OwnedImmediateProofInProof<Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_located_child_owned(p).unwrap().into()) }

    /// Get all conclusions of this [ProofStep]
    pub fn get_conclusions(&self) -> HashSet<&Proposition> {
        self.get_immediate_subproofs().into_iter().fold(
            // Combine
            HashSet::from_iter(self.get_explicit_conclusions()), // The explcit conclusions of this ProofStep
            |mut acc,next| {
                match next {
                    Proof::Inference(inference) => acc.extend(inference.get_explicit_conclusions()),
                    Proof::Composite(composite) => acc.extend(composite.get_conclusions().into_iter()),
                } acc
            } // And the conclusions of this expression's children Subproofs
        )
    }
    /// Get owned versions of all conclusions of this [ProofStep]
    pub fn get_conclusions_owned(&self) -> PropositionSet {
        self.get_immediate_subproofs().into_iter().fold(
            // Combine
            PropositionSet::from_iter(self.get_explicit_conclusions_owned()), // The explcit conclusions of this ProofStep
            |mut acc,next| {
                match next {
                    Proof::Inference(inference) => acc.extend(inference.get_explicit_conclusions_owned()),
                    Proof::Composite(composite) => acc.extend(composite.get_conclusions_owned().into_iter()),
                }; acc
            } // And the conclusions of this expression's children Subproofs
        )
    }
    
    /// Get all implicit conclusions of this [ProofStep]
    pub fn get_implicit_conclusions(&self) -> HashSet<&Proposition> {
        let explicits = HashSet::from_iter(self.get_explicit_conclusions());
        self.get_conclusions().difference(&explicits).cloned().collect()
    }
    /// Get owned versions of all implicit conclusions of this [ProofStep]
    pub fn get_implicit_conclusions_owned(&self) -> PropositionSet {
        let explicits = PropositionSet::from_iter(self.get_explicit_conclusions_owned());
        self.get_conclusions_owned().difference(&explicits).cloned().collect()
    }
}
impl <Rule: InferenceRule> ProofStep<Rule> for CompositeProof<Rule> {
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>
        { (0..self.assumptions.len()).map(|n| PropositionInInferencePath::assumption(n)) }
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>
        { (0..self.explicit_conclusions.len()).map(|n| PropositionInInferencePath::conclusion(n)) }

    fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition> { &self.assumptions }
    fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition> { &self.explicit_conclusions }
}
impl <Rule:InferenceRule> HasChildren<PropositionInInferencePath,Proposition> for CompositeProof<Rule> {
    fn valid_primitive_paths(&self) -> Vec<PropositionInInferencePath> { valid_primitive_paths_inner(self, self.explicit_conclusions.len()) }
    
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
