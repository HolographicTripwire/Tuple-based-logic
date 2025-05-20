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
    pub fn subproof_at(&self, mut step: ProofStep) -> Result<&SubProof,()> {
        let Some(incremental_step) = step.pop() else { return Err(()) };
        let Some(subproof) = self.subproofs.get(incremental_step) else { return Err(()) };
        subproof.subproof_at(step)
    }

    pub fn implicit_conclusions(&self) -> PropositionSet {
        let mut result = PropositionSet::new(&[]);
        for subproof in &self.subproofs { result.merge(&subproof.implicit_conclusions()); }
        result
    }

    // Getters and setters
    pub fn premises(&self) -> &Vec<Proposition> { &self.premises }
    pub fn subproofs(&self) -> &Vec<SubProof> { &self.subproofs }
    pub fn conclusions(&self) -> &Vec<Proposition> { &self.conclusions }
}

#[derive(Clone)]
pub enum SubProof {
    Atomic(Inference),
    Composite(Proof)
}

impl SubProof {
    pub fn premises(&self) -> &Vec<Proposition> { match self {
            SubProof::Atomic(proof_step) => &proof_step.assumptions,
            SubProof::Composite(proof) => &proof.premises,
    }}

    pub fn conclusions(&self) -> &Vec<Proposition> { match self {
        SubProof::Atomic(proof_step) => &proof_step.conclusions,
        SubProof::Composite(proof) => &proof.conclusions,
    }}

    pub fn implicit_conclusions(&self) -> PropositionSet { match self {
        SubProof::Atomic(inference) => PropositionSet::from(&inference.conclusions),
        SubProof::Composite(proof) => proof.implicit_conclusions(),
    }}

    pub fn subproof_at(&self, step: ProofStep) -> Result<&SubProof,()> {
        if step.0.is_empty() { Ok(self) }
        else { match self {
            SubProof::Atomic(_) => Err(()),
            SubProof::Composite(proof) => proof.subproof_at(step),
        }}
    }
}
