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

impl <Rule: InferenceRule> Proof<Rule> {
    /// Get the premises of this [Proof]
    pub fn assumptions(&self) -> &Vec<Proposition> { match self {
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
        Proof::Composite(composite) => { composite.conclusions() },
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

        pub fn assumptions(&self) -> &Vec<Proposition> { &self.assumptions }
        pub fn subproofs(&self) -> &Vec<Proof<Rule>> { &self.subproofs }
        pub fn explicit_conclusions(&self) -> &Vec<Proposition> { &self.explicit_conclusions }
        pub fn implicit_conclusions(&self) -> HashSet<&Proposition> {
            let explicits = HashSet::from_iter(self.explicit_conclusions());
            self.conclusions().difference(&explicits).cloned().collect()
        }
        pub fn conclusions(&self) -> HashSet<&Proposition> { 
            self.subproofs.iter().fold(HashSet::from_iter(&self.explicit_conclusions), 
            |mut acc,next| { acc.extend(next.conclusions().iter()); acc }
            )
        }
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
