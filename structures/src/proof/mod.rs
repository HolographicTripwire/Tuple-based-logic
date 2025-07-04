pub mod path;
pub mod error;

use std::collections::HashSet;

use crate::{inference::{Inference, InferenceRule}, propositions::{Proposition}};

/// This struct represents a step within a larger proof
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Proof<Rule: InferenceRule> {
    Atomic(Inference<Rule>), // A single inference step
    Composite(CompositeProof<Rule>) // A composite proof made of further subproofs
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct CompositeProof<Rule: InferenceRule> {
    pub assumptions: Vec<Proposition>,
    pub subproofs: Vec<Proof<Rule>>,
    pub explicit_conclusions: Vec<Proposition>,

}

impl <Rule: InferenceRule> Proof<Rule> {
    /// Get the premises of this [Proof]
    pub fn premises(&self) -> &Vec<Proposition> { match self {
        Proof::Atomic(proof_step) => &proof_step.assumptions,
        Proof::Composite(composite) => &composite.assumptions,
    }}

    /// Get the explicit conclusions of this [Proof]
    pub fn explicit_conclusions(&self) -> &Vec<Proposition> { match self {
        Proof::Atomic(proof_step) => &proof_step.conclusions,
        Proof::Composite(composite) => &composite.explicit_conclusions,
    }}

    /// Get the implicit conclusions of this [Proof]; that is, each [Proposition] which this [Proof] which is proven but not explicitly returned
    pub fn implicit_conclusions(&self) -> HashSet<&Proposition> {
        let explicits = HashSet::from_iter(self.explicit_conclusions());
        self.conclusions().difference(&explicits).cloned().collect()
    }

    fn conclusions(&self) -> HashSet<&Proposition> { match self {
        Proof::Atomic(inference) => HashSet::from_iter(inference.conclusions.iter()),
        Proof::Composite(composite) => {
            composite.subproofs.iter().fold(HashSet::from_iter(&composite.explicit_conclusions), 
            |mut acc,next| { acc.extend(next.conclusions().iter()); acc }
            )
        },
    }}
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
