pub mod path;
pub mod error;

use std::collections::HashSet;

use path_lib::HasChildren;

use crate::{inference::{Inference, InferenceRule}, proof::path::{AtomicSubproofPath}, propositions::Proposition};

pub trait ProofStep<'a, Rule:'a + InferenceRule>: HasChildren<'a,AtomicSubproofPath,Proof<Rule>> {
    fn assumptions(&self) -> &Vec<Proposition>;
    fn explicit_conclusions(&self) -> &Vec<Proposition>;
    
    fn subproofs(&'a self) -> impl IntoIterator<Item=&'a Proof<Rule>> { self.children() }
    fn conclusions(&'a self) -> HashSet<&'a Proposition> {
        self.subproofs().into_iter().fold(HashSet::from_iter(self.explicit_conclusions()), 
            |mut acc,next| { acc.extend(next.conclusions().iter()); acc }
        )
    }
    fn implicit_conclusions(&'a self) -> HashSet<&'a Proposition> {
        let explicits = HashSet::from_iter(self.explicit_conclusions());
        self.conclusions().difference(&explicits).cloned().collect()
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
}

#[cfg(test)]
mod tests {
    use crate::proof::path::SubproofPath;    

    #[test]
    fn test_getters() {
        let step = SubproofPath::empty();
        assert_eq!(step.paths(), &vec![])
    }
}
