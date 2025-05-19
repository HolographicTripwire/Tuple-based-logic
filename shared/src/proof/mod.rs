pub mod generation;
pub mod error;

use crate::{inference::Inference, proposition::Proposition};

#[derive(Clone)]
pub struct Proof {
    pub premises: Vec<Proposition>,
    pub subproofs: Vec<SubProof>,
    pub conclusions: Vec<Proposition>
}

impl Proof {
    pub fn subproof_at(&self, mut step: ProofStep) -> Result<&SubProof,()> {
        let Some(incremental_step) = step.pop() else { return Err(()) };
        let Some(subproof) = self.subproofs.get(incremental_step) else { return Err(()) };
        return subproof.subproof_at(step);
    }
}

#[derive(Clone)]
pub enum SubProof {
    Atomic(Inference),
    Composite(Proof)
}

impl SubProof {
    pub fn premises(&self) -> &Vec<Proposition> {
        match self {
            SubProof::Atomic(proof_step) => &proof_step.assumptions,
            SubProof::Composite(proof) => &proof.premises,
        }
    }

    pub fn conclusions(&self) -> &Vec<Proposition> {
        match self {
            SubProof::Atomic(proof_step) => &proof_step.conclusions,
            SubProof::Composite(proof) => &proof.conclusions,
        }
    }

    pub fn subproof_at(&self, step: ProofStep) -> Result<&SubProof,()> {
        if step.0.len() == 0 { Ok(self) }
        else { match self {
            SubProof::Atomic(_) => Err(()),
            SubProof::Composite(proof) => proof.subproof_at(step),
        }}
    }
}

#[derive(Clone)]
/// Identifies a particular step iwthin a [Proof], and can be given to such a [Proof] to retreive the [SubProof] at that step
pub struct ProofStep(pub Vec<usize>);

impl ProofStep {
    /// Pushes a new step to the list of steps.
    /// ProofStep behaves like a queue, so the provided step will go to the back of the queue
    pub fn push(&mut self, step: usize) {self.0.insert(0,step)}
    pub fn pop(&mut self) -> Option<usize> { self.0.pop() }
}