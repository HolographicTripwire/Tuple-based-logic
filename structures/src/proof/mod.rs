mod subproof_path;
mod proposition_path;
mod subexpression_path;
pub mod error;

use std::collections::HashSet;

use path_lib::{obj_at_path::ObjAtPath, HasChildren, HasDescendants};

pub use subproof_path::*;
pub use proposition_path::*;
pub use subexpression_path::*;

use crate::{inference::{Inference, InferenceRule}, expressions::Proposition};

pub trait ProofStep<'a, Rule:'a + InferenceRule> : HasChildren<'a,ProofPropositionPath,Proposition> + HasChildren<'a,AtomicSubproofPath,Proof<Rule>> {
    fn assumption_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath>;
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath>;
    fn immediate_subproof_paths(&'a self) -> impl IntoIterator<Item=AtomicSubproofPath> { <Self as HasChildren<'a,AtomicSubproofPath,Proof<Rule>>>::valid_primitive_paths(self) }
    
    fn get_assumptions(&'a self) -> impl IntoIterator<Item = &'a Proposition>
        { self.assumption_paths().into_iter().map(|p| self.get_child(&p).unwrap()) }
    fn get_located_assumptions(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Proposition,ProofPropositionPath>>
        { self.assumption_paths().into_iter().map(|p| self.get_located_child(p).unwrap()) }
    fn get_explicit_conclusions(&'a self) -> impl IntoIterator<Item = &'a Proposition>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_child(&p).unwrap()) }
    fn get_located_explicit_conclusions(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Proposition,ProofPropositionPath>>
        { self.explicit_conclusion_paths().into_iter().map(|p| self.get_located_child(p).unwrap()) }
    fn get_immediate_subproofs(&'a self) -> impl IntoIterator<Item = &'a Proof<Rule>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_descendant(&p).unwrap()) }
    fn get_located_immediate_subproofs(&'a self) -> impl IntoIterator<Item = ObjAtPath<'a,Proof<Rule>,AtomicSubproofPath>>
        { self.immediate_subproof_paths().into_iter().map(|p| self.get_located_child(p).unwrap()) }

    fn get_conclusions(&'a self) -> HashSet<&'a Proposition> {
        self.get_immediate_subproofs().into_iter().fold(HashSet::from_iter(self.get_explicit_conclusions()), 
            |mut acc,next| { acc.extend(next.get_conclusions().iter()); acc }
        )
    }
    fn get_implicit_conclusions(&'a self) -> HashSet<&'a Proposition> {
        let explicits = HashSet::from_iter(self.get_explicit_conclusions());
        self.get_conclusions().difference(&explicits).cloned().collect()
    }
}
pub (crate) fn valid_primitive_paths_inner<'a,Rule: 'a + InferenceRule, Step: ProofStep<'a,Rule>>(step: &'a Step) -> impl IntoIterator<Item = ProofPropositionPath> {
    let assumptions = (0..step.assumption_paths().into_iter().count()).map(|ix| ProofPropositionPath::assumption(ix));
    let conclusions = (0..step.get_conclusions().len()).map(|ix| ProofPropositionPath::conclusion(ix));
    assumptions.chain(conclusions)
}
pub (crate) fn get_child_inner<'a,Rule: 'a + InferenceRule, Step: ProofStep<'a,Rule>>(step: &'a Step, path: &ProofPropositionPath) -> Result<&'a Proposition,()> {
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
    fn assumption_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath> { match self {
        Proof::Atomic(inference) => inference.assumption_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.assumption_paths().into_iter().collect(),
    }}
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath> {match self {
        Proof::Atomic(inference) => inference.explicit_conclusion_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.explicit_conclusion_paths().into_iter().collect(),
    }}
    
    fn get_immediate_subproofs(&'a self) -> impl IntoIterator<Item=&'a Proof<Rule>>
        { <Self as HasChildren<'_, AtomicSubproofPath, Proof<Rule>>>::get_children(self) }
}

impl <'a, Rule:'a + InferenceRule> HasChildren<'a,ProofPropositionPath,Proposition> for Proof<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath> { valid_primitive_paths_inner(self) }
    fn get_child(&'a self, path: &ProofPropositionPath) -> Result<&'a Proposition,()> { get_child_inner(self,path) }
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
    fn assumption_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath>
        { (0..self.assumptions.len()).map(|n| ProofPropositionPath::assumption(n)) }
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath>
        { (0..self.explicit_conclusions.len()).map(|n| ProofPropositionPath::conclusion(n)) }

    fn get_assumptions(&'a self) -> impl IntoIterator<Item = &'a Proposition> { &self.assumptions }
    fn get_explicit_conclusions(&'a self) -> impl IntoIterator<Item = &'a Proposition> { &self.explicit_conclusions }
    fn get_immediate_subproofs(&'a self) -> impl IntoIterator<Item=&'a Proof<Rule>>
        { <Self as HasChildren<'_, AtomicSubproofPath, Proof<Rule>>>::get_children(self) }
}

impl <'a, Rule:'a + InferenceRule> HasChildren<'a,ProofPropositionPath,Proposition> for CompositeProof<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath> { valid_primitive_paths_inner(self) }
    fn get_child(&'a self, path: &ProofPropositionPath) -> Result<&'a Proposition,()> { get_child_inner(self,path) }
}

#[cfg(test)]
mod tests {
    use crate::proof::subproof_path::SubproofPath;    

    #[test]
    fn test_getters() {
        let step = SubproofPath::empty();
        assert_eq!(step.paths(), &vec![])
    }
}
