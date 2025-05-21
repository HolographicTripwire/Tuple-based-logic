use shared::{propositions::Proposition,proof::{error::ErrorInProof, Proof, SubProof}};

pub trait ProofGenerator<G: ProofGenerator<G>>: Clone {
    fn generate(&self, conclusions: &[Proposition]) -> Result<SubProofPromise<G>,ProofGenerationError>;
}

#[derive(Clone)]
pub struct ProofPromise<G: ProofGenerator<G>> {
    pub premises: Vec<Proposition>,
    pub subproofs: Vec<SubProofPromise<G>>,
    pub conclusions: Vec<Proposition>
}

impl <G: ProofGenerator<G>> ProofPromise<G> {
    pub fn resolve_once(&self) -> Result<ProofPromise<G>,ErrorInProof<ProofGenerationError>> {
        let mut subproofs = Vec::new();
        for (i, proof) in self.subproofs.iter().enumerate() { match proof.resolve_once() {
            Ok(subproof) => subproofs.push(subproof),
            Err(err) => return Err(ErrorInProof::at_substep(i, err)),
        }}
        Ok(ProofPromise { premises: self.premises.clone(), subproofs, conclusions: self.conclusions.clone() })
    }

    pub fn resolve(&self) -> Result<Proof,ProofGenerationError> {
        let mut subproofs = Vec::new();
        for proof in &self.subproofs { subproofs.push(proof.resolve()?) }
        Ok(Proof::new(self.premises.clone(), subproofs, self.conclusions.clone()))
    }
}

#[derive(Clone)]
pub enum SubProofPromise<G: ProofGenerator<G>> {
    Resolved(SubProof),
    Composite(ProofPromise<G>),
    Generator(G,Vec<Proposition>)
}

impl <G: ProofGenerator<G>> SubProofPromise<G> {
    pub fn resolve_once(&self) -> Result<SubProofPromise<G>,ProofGenerationError> {
        Ok(match self {
            SubProofPromise::Resolved(_) | SubProofPromise::Composite(_) => self.clone(),
            SubProofPromise::Generator(generator, conclusions) => generator.generate(conclusions)?,
        })
    }

    pub fn resolve(&self) -> Result<SubProof,ProofGenerationError> {
        Ok(match self {
            SubProofPromise::Resolved(sub_proof) => sub_proof.clone(),
            SubProofPromise::Composite(proof_promise) => SubProof::Composite(proof_promise.resolve()?),
            SubProofPromise::Generator(generator, conclusions) => generator.generate(conclusions)?.resolve()?,
        })
    }
}

#[derive(Clone)]
pub enum ProofGenerationError {}
