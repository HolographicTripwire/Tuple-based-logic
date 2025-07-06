mod subproof_path;
mod proposition_path;
mod subexpression_path;
pub mod error;

use std::collections::HashSet;

use path_lib::HasChildren;

pub use subproof_path::*;
pub use proposition_path::*;
pub use subexpression_path::*;

use crate::{inference::{Inference, InferenceRule}, expressions::Proposition};

pub trait ProofStep<'a, Rule:'a + InferenceRule> : HasChildren<'a,ProofPropositionPath,Proposition> {
    fn assumptions(&self) -> &Vec<Proposition>;
    fn explicit_conclusions(&self) -> &Vec<Proposition>;
    fn subproofs(&'a self) -> impl IntoIterator<Item=&'a Proof<Rule>>;
    
    fn conclusions(&'a self) -> HashSet<&'a Proposition> {
        self.subproofs().into_iter().fold(HashSet::from_iter(self.explicit_conclusions()), 
            |mut acc,next| { acc.extend(next.conclusions().iter()); acc }
        )
    }
    fn implicit_conclusions(&'a self) -> HashSet<&'a Proposition> {
        let explicits = HashSet::from_iter(self.explicit_conclusions());
        self.conclusions().difference(&explicits).cloned().collect()
    }

    fn _valid_primitive_paths(&'a self) -> impl IntoIterator<Item = ProofPropositionPath> {
        let assumptions = (0..self.assumptions().len()).map(|ix| ProofPropositionPath::assumption(ix));
        let conclusions = (0..self.conclusions().len()).map(|ix| ProofPropositionPath::conclusion(ix));
        assumptions.chain(conclusions)
    }

    fn _get_child(&'a self, path: &ProofPropositionPath) -> Result<&'a Proposition,()> {
        let propositions = if path.is_conclusion { self.assumptions() } else { self.explicit_conclusions() };
        propositions.get(path.proposition_index).ok_or(())
    }
}

/// This struct represents a step within a larger proof
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Proof<Rule: InferenceRule> {
    Atomic(Inference<Rule>), // A single inference step
    Composite(CompositeProof<Rule>) // A composite proof made of further subproofs
}

impl <'a,Rule: 'a + InferenceRule> ProofStep<'a,Rule> for Proof<Rule> {
    fn assumptions(&self) -> &Vec<Proposition> { match self {
        Proof::Atomic(inference) => inference.assumptions(),
        Proof::Composite(composite) => composite.assumptions(),
    }}
    fn explicit_conclusions(&self) -> &Vec<Proposition> { match self {
        Proof::Atomic(inference) => inference.explicit_conclusions(),
        Proof::Composite(composite) => composite.explicit_conclusions(),
    }}
    
    fn subproofs(&'a self) -> impl IntoIterator<Item=&'a Proof<Rule>>
        { <Self as HasChildren<'_, AtomicSubproofPath, Proof<Rule>>>::get_children(self) }
}

impl <'a, Rule:'a + InferenceRule> HasChildren<'a,ProofPropositionPath,Proposition> for Proof<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath> { self._valid_primitive_paths() }
    fn get_child(&'a self, path: &ProofPropositionPath) -> Result<&'a Proposition,()> { self._get_child(path) }
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
    fn assumptions(&self) -> &Vec<Proposition> { &self.assumptions }
    fn explicit_conclusions(&self) -> &Vec<Proposition> { &self.explicit_conclusions }
    fn subproofs(&'a self) -> impl IntoIterator<Item=&'a Proof<Rule>>
        { <Self as HasChildren<'_, AtomicSubproofPath, Proof<Rule>>>::get_children(self) }
}

impl <'a, Rule:'a + InferenceRule> HasChildren<'a,ProofPropositionPath,Proposition> for CompositeProof<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = ProofPropositionPath> { self._valid_primitive_paths() }
    fn get_child(&'a self, path: &ProofPropositionPath) -> Result<&'a Proposition,()> { self._get_child(path) }
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
