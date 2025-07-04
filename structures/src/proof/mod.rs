pub mod path;
pub mod error;

use std::collections::HashSet;

use path::AtomicSubproofPath;

use crate::{inference::{Inference, InferenceRule}, propositions::{Proposition}};

/// This struct represents a step within a larger proof
#[derive(Clone)]
pub enum Proof<Rules: InferenceRule> {
    Atomic(Inference<Rules>), // A single inference step
    Composite(Vec<Proposition>,Vec<Proof<Rules>>,Vec<Proposition>) // A composite proof made of further subproofs
}

impl <Rules: InferenceRule> Proof<Rules> {
    /// Get the premises of this [Proof]
    pub fn premises(&self) -> &Vec<Proposition> { match self {
        Proof::Atomic(proof_step) => &proof_step.assumptions,
        Proof::Composite(premises, _, _) => premises,
    }}

    /// Get the explicit conclusions of this [Proof]
    pub fn explicit_conclusions(&self) -> &Vec<Proposition> { match self {
        Proof::Atomic(proof_step) => &proof_step.conclusions,
        Proof::Composite(_,_,explicit_conclusions) => explicit_conclusions,
    }}

    /// Get the implicit conclusions of this [Proof]; that is, each [Proposition] which this [Proof] which is proven but not explicitly returned
    pub fn implicit_conclusions(&self) -> HashSet<&Proposition> {
        let explicits = HashSet::from_iter(self.explicit_conclusions());
        self.conclusions().difference(&explicits).cloned().collect()
    }

    fn conclusions(&self) -> HashSet<&Proposition> { match self {
        Proof::Atomic(inference) => HashSet::from_iter(inference.conclusions.iter()),
        Proof::Composite(_, subproofs, explicit_conclusions) => {
            subproofs.iter().fold(HashSet::from_iter(explicit_conclusions), 
            |mut acc,next| { acc.extend(next.conclusions().iter()); acc }
            )
        },
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getters() {
        let step = AtomicSubproofPath::here();
        assert_eq!(step.0, vec![])
    }
}
