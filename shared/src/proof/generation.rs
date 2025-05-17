use crate::proposition::{Proposition, PropositionSet};

use super::{error::ErrorInProof, Proof, SubProof};

pub trait ProofGenerator<G: ProofGenerator<G>>: Clone {
    fn generate(&self, conclusions: &Vec<Proposition>) -> Result<SubProofPromise<G>,()>;
}

#[derive(Clone)]
pub struct ProofPromise<G: ProofGenerator<G>> {
    pub premises: Vec<Proposition>,
    pub subproofs: Vec<SubProofPromise<G>>,
    pub conclusions: PropositionSet
}

impl <G: ProofGenerator<G>> ProofPromise<G> {
    pub fn resolve_once(&self) -> Result<ProofPromise<G>,ErrorInProof<()>> {
        let mut subproofs = Vec::new();
        for (i, proof) in self.subproofs.iter().enumerate() { match proof.resolve_once() {
            Ok(subproof) => subproofs.push(subproof),
            Err(err) => return Err(ErrorInProof::new(i, err)),
        }}
        Ok(ProofPromise { premises: self.premises.clone(), subproofs, conclusions: self.conclusions.clone() })
    }

    pub fn resolve(&self) -> Result<Proof,()> {
        let mut subproofs = Vec::new();
        for proof in &self.subproofs { subproofs.push(proof.resolve()?) }
        Ok(Proof { premises: self.premises.clone(), subproofs, conclusions: self.conclusions.clone() })
    }
}

#[derive(Clone)]
pub enum SubProofPromise<G: ProofGenerator<G>> {
    Resolved(SubProof),
    Composite(ProofPromise<G>),
    Generator(G,Vec<Proposition>)
}

impl <G: ProofGenerator<G>> SubProofPromise<G> {
    pub fn resolve_once(&self) -> Result<SubProofPromise<G>,()> {
        Ok(match self {
            SubProofPromise::Resolved(_) => self.clone(),
            SubProofPromise::Composite(_) => self.clone(),
            SubProofPromise::Generator(generator, conclusions) => generator.generate(conclusions)?,
        })
    }

    pub fn resolve(&self) -> Result<SubProof,()> {
        Ok(match self {
            SubProofPromise::Resolved(sub_proof) => sub_proof.clone(),
            SubProofPromise::Composite(proof_promise) => SubProof::Composite(proof_promise.resolve()?),
            SubProofPromise::Generator(generator, conclusions) => generator.generate(conclusions)?.resolve()?,
        })
    }
}
