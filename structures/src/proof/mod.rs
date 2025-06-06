pub mod step;
pub mod error;

use step::ProofStep;

use crate::{inference::Inference, propositions::{Proposition, PropositionSet}};

#[derive(Clone)]
pub struct Proof {
    premises: Vec<Proposition>,
    subproofs: Vec<SubProof>,
    conclusions: Vec<Proposition>
}

impl Proof {
    /// Create a new [Proof] with the given premises, subproofs, and conclusions
    pub fn new(premises: Vec<Proposition>, subproofs: Vec<SubProof>, conclusions: Vec<Proposition>) -> Self
        { Self { premises, subproofs, conclusions } }

    // Getters and setters
    /// Get the premises of this [Proof]
    pub fn premises(&self) -> &Vec<Proposition> { &self.premises }
    /// Get the subproofs within this [Proof]
    pub fn subproofs(&self) -> &Vec<SubProof> { &self.subproofs }
    /// Get the conclusions of this [Proof]
    pub fn conclusions(&self) -> &Vec<Proposition> { &self.conclusions }

    /// Get the implicit conclusions of this [Proof]; that is, each [Proposition] which this [Proof] which is proven but not explicitly returned
    pub fn implicit_conclusions(&self) -> PropositionSet {
        let mut result = PropositionSet::new(&[]);
        for subproof in &self.subproofs { result.merge(&subproof.implicit_conclusions()); }
        result
    }

    /// Get the [SubProof] within this [Proof] at a particular proof step, if there is one
    pub fn subproof_at(&self, mut step: ProofStep) -> Result<&SubProof,()> {
        let Some(incremental_step) = step.pop() else { return Err(()) };
        let Some(subproof) = self.subproofs.get(incremental_step) else { return Err(()) };
        subproof.subproof_at(step)
    }
}

/// This struct represents a step within a larger proof
#[derive(Clone)]
pub enum SubProof {
    Atomic(Inference), // A single inference step
    Composite(Proof)   // A composite proof made of further subproofs
}

impl SubProof {
    /// Get the premises of this [SubProof]
    pub fn premises(&self) -> &Vec<Proposition> { match self {
            SubProof::Atomic(proof_step) => &proof_step.assumptions,
            SubProof::Composite(proof) => &proof.premises,
    }}

    /// Get the conclusions of this [SubProof]
    pub fn conclusions(&self) -> &Vec<Proposition> { match self {
        SubProof::Atomic(proof_step) => &proof_step.conclusions,
        SubProof::Composite(proof) => &proof.conclusions,
    }}

    /// Get the implicit conclusions of this [SubProof]; that is, each [Proposition] which this [SubProof] which is proven but not explicitly returned
    pub fn implicit_conclusions(&self) -> PropositionSet { match self {
        SubProof::Atomic(inference) => PropositionSet::from(&inference.conclusions),
        SubProof::Composite(proof) => proof.implicit_conclusions(),
    }}

    /// Get the [SubProof] within this [SubProof] at a particular proof step, if there is one
    pub fn subproof_at(&self, step: ProofStep) -> Result<&SubProof,()> {
        if step.0.is_empty() { Ok(self) }
        else { match self {
            SubProof::Atomic(_) => Err(()),
            SubProof::Composite(proof) => proof.subproof_at(step),
        }}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getters() {
        let step = ProofStep::here();
        assert_eq!(step.0, vec![])
    }
}
