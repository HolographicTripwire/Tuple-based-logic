mod subproof_path;
mod proposition_in_inference_path;
mod inference_path;
pub mod error;

use std::collections::HashSet;

use path_lib::{HasChildren, obj_at_path::{ObjAtPath, OwnedObjAtPath}};

pub use subproof_path::*;
pub use proposition_in_inference_path::*;
pub use inference_path::*;

use crate::{inference::{Inference, InferenceRule}, expressions::Proposition};

pub trait ProofStep<'a, Rule:'a + InferenceRule> : HasChildren<'a,PropositionInInferencePath,Proposition> + HasChildren<'a,AtomicSubproofPath,Proof<Rule>> {
    // To be implemented by implemntors of this trait
    /// Get the [ProofPropositionPaths](ProofPropositionPath) of all assumptions within this [ProofStep]
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>;
    /// Get the [ProofPropositionPaths](ProofPropositionPath) of all explicit conclusions within this [ProofStep]
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>;
    /// Get the [AtomicSubproofPaths](AtomicSubproofPath) of all immediate subproofs within this [ProofStep]
    fn immediate_subproof_paths(&'a self) -> impl IntoIterator<Item=AtomicSubproofPath> { <Self as HasChildren<AtomicSubproofPath,Proof<Rule>>>::valid_primitive_paths(self) }
    
    /// Get references to all assumptions within this [ProofStep]
    fn get_assumptions(&'a self) -> impl IntoIterator<Item = &'a Proposition>
        { self.assumption_paths().into_iter().map(|p| self.get_child(&p).unwrap()) }
    /// Get all assumptions within this [ProofStep]
    fn get_assumptions_owned(&'a self) -> impl IntoIterator<Item = Proposition>
        { self.assumption_paths().into_iter().map(|p| self.get_child(&p).unwrap().to_owned()) }
    /// Get references to all assumptions within this [ProofStep], located by their [ProofPropositionPath]
    fn get_located_assumptions(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Proposition,PropositionInInferencePath>>
        { self.assumption_paths().into_iter().map(|p| self.get_located_child(p).unwrap()) }
    /// Get all assumptions within this [ProofStep], located by their [ProofPropositionPath]
    fn get_located_assumptions_owned(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInInferencePath>>
        { self.assumption_paths().into_iter().map(|p| self.get_located_child_owned(p).unwrap()) }
    
    /// Get all explicit conclusions within this [ProofStep]
    fn get_explicit_conclusions(&'a self) -> impl IntoIterator<Item = &'a Proposition>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_child(&p).unwrap()) }
    /// Get owned versions of all explicit conclusions within this [ProofStep]
    fn get_explicit_conclusions_owned(&'a self) -> impl IntoIterator<Item = Proposition>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_child(&p).unwrap().to_owned()) }
    /// Get all explicit conclusions within this [ProofStep], located by their [ProofPropositionPath]
    fn get_located_explicit_conclusions(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Proposition,PropositionInInferencePath>>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_located_child(p).unwrap()) }
    /// Get owned versions of all explicit conclusions within this [ProofStep], located by their [ProofPropositionPath]
    fn get_located_explicit_conclusions_owned(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInInferencePath>>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_located_child_owned(p).unwrap()) }
    
    /// Get all immediate subproofs within this [ProofStep]
    fn get_immediate_subproofs(&'a self) -> impl IntoIterator<Item = &'a Proof<Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_child(&p).unwrap()) }
    /// Get owned versions of all immediate subproofs within this [ProofStep]
    fn get_immediate_subproofs_owned(&'a self) -> impl IntoIterator<Item = Proof<Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_child(&p).unwrap().to_owned()) }
    /// Get all immediate subproofs within this [ProofStep], located by their [AtomicSubproofPath]
    fn get_located_immediate_subproofs(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Proof<Rule>,AtomicSubproofPath>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_located_child(p).unwrap()) }
    /// Get owned versions of all immediate subproofs within this [ProofStep], located by their [AtomicSubproofPath]
    fn get_located_immediate_subproofs_owned(&'a self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicSubproofPath>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_located_child_owned(p).unwrap()) }

    /// Get all conclusions of this [ProofStep]
    fn get_conclusions(&'a self) -> HashSet<&'a Proposition> {
        self.get_immediate_subproofs().into_iter().fold(
            // Combine
            HashSet::from_iter(self.get_explicit_conclusions()), // The explcit conclusions of this ProofStep
            |mut acc,next| { acc.extend(next.get_conclusions().iter()); acc } // And the conclusions of this expression's children Subproofs
        )
    }
    /// Get owned versions of all conclusions of this [ProofStep]
    fn get_conclusions_owned(&'a self) -> HashSet<Proposition> {
        self.get_immediate_subproofs().into_iter().fold(
            // Combine
            HashSet::from_iter(self.get_explicit_conclusions_owned()), // The explcit conclusions of this ProofStep
            |mut acc,next| { acc.extend(next.get_conclusions_owned().into_iter()); acc } // And the conclusions of this expression's children Subproofs
        )
    }
    
    /// Get all implicit conclusions of this [ProofStep]
    fn get_implicit_conclusions(&'a self) -> HashSet<&'a Proposition> {
        let explicits = HashSet::from_iter(self.get_explicit_conclusions());
        self.get_conclusions().difference(&explicits).cloned().collect()
    }
    /// Get owned versions of all implicit conclusions of this [ProofStep]
    fn get_implicit_conclusions_owned(&'a self) -> HashSet<Proposition> {
        let explicits = HashSet::from_iter(self.get_explicit_conclusions_owned());
        self.get_conclusions_owned().difference(&explicits).cloned().collect()
    }
}
pub (crate) fn valid_primitive_paths_inner<'a,Rule: 'a + InferenceRule, Step: ProofStep<'a,Rule>>(step: &'a Step) -> impl IntoIterator<Item = PropositionInInferencePath> {
    let assumptions = (0..step.assumption_paths().into_iter().count()).map(|ix| PropositionInInferencePath::assumption(ix));
    let conclusions = (0..step.get_conclusions().len()).map(|ix| PropositionInInferencePath::conclusion(ix));
    assumptions.chain(conclusions)
}
pub (crate) fn get_child_inner<'a,Rule: 'a + InferenceRule, Step: ProofStep<'a,Rule>>(step: &'a Step, path: &PropositionInInferencePath) -> Result<&'a Proposition,()> {
    let n = path.proposition_index;
    if path.is_conclusion { step.get_assumptions().into_iter().nth(n) } 
    else { step.get_explicit_conclusions().into_iter().nth(n) }
    .ok_or(())
}

/// This struct represents a step within a larger proof
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Proof<Rule: InferenceRule> {
    Atomic(Inference<Rule>), // A single inference step
    Composite(CompositeProof<Rule>) // A composite proof made of further subproofs
}

impl <'a,Rule: 'a + InferenceRule> ProofStep<'a,Rule> for Proof<Rule> {
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath> { match self {
        Proof::Atomic(inference) => inference.assumption_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.assumption_paths().into_iter().collect(),
    }}
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath> {match self {
        Proof::Atomic(inference) => inference.explicit_conclusion_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.explicit_conclusion_paths().into_iter().collect(),
    }}
    
    fn get_immediate_subproofs(&'a self) -> impl IntoIterator<Item=&'a Proof<Rule>>
        { <Self as HasChildren<'_, AtomicSubproofPath, Proof<Rule>>>::get_children(self) }
}

impl <'a, Rule:'a + InferenceRule> HasChildren<'a,PropositionInInferencePath,Proposition> for Proof<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath> { valid_primitive_paths_inner(self) }
    
    fn get_child(&'a self, path: &PropositionInInferencePath) -> Result<&'a Proposition,()> { get_child_inner(self,path) }
    fn get_child_owned(&self, path: &PropositionInInferencePath) -> Result<Proposition,()> where Proposition: Clone
        { get_child_inner(self, path).cloned() }
}

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
impl <'a,Rule: 'a + InferenceRule> ProofStep<'a,Rule> for CompositeProof<Rule> {
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>
        { (0..self.assumptions.len()).map(|n| PropositionInInferencePath::assumption(n)) }
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath>
        { (0..self.explicit_conclusions.len()).map(|n| PropositionInInferencePath::conclusion(n)) }

    fn get_assumptions(&'a self) -> impl IntoIterator<Item = &'a Proposition> { &self.assumptions }
    fn get_explicit_conclusions(&'a self) -> impl IntoIterator<Item = &'a Proposition> { &self.explicit_conclusions }
    fn get_immediate_subproofs(&'a self) -> impl IntoIterator<Item=&'a Proof<Rule>>
        { <Self as HasChildren<'_, AtomicSubproofPath, Proof<Rule>>>::get_children(self) }
}
impl <'a, Rule:'a + InferenceRule> HasChildren<'a,PropositionInInferencePath,Proposition> for CompositeProof<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = PropositionInInferencePath> { valid_primitive_paths_inner(self) }
    
    fn get_child(&'a self, path: &PropositionInInferencePath) -> Result<&'a Proposition,()> { get_child_inner(self,path) }
    fn get_child_owned(&self, path: &PropositionInInferencePath) -> Result<Proposition,()> where Proposition: Clone 
        { get_child_inner(self,path).cloned() }
}

#[cfg(test)]
mod tests {
    use crate::proof::subproof_path::ProofInProofPath;    

    #[test]
    fn test_getters() {
        let step = ProofInProofPath::empty();
        assert_eq!(step.paths(), &vec![])
    }
}
