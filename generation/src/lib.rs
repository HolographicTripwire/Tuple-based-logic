use tbl_structures::{inference::InferenceRule, proof::{error::ErrorInProof, CompositeProof, Proof}, propositions::Proposition};

pub trait ProofGenerator<Rule: InferenceRule, G: ProofGenerator<Rule, G>>: Clone {
    fn generate(&self, conclusions: &[Proposition]) -> Result<ProofPromise<Rule,G>,ProofGenerationError>;
}

#[derive(Clone)]
pub enum ProofPromise<Rule: InferenceRule, G: ProofGenerator<Rule,G>> {
    Resolved(Proof<Rule>),
    Composite(CompositeProofPromise<Rule,G>),
    Generator(G,Vec<Proposition>)
}

impl <Rule: InferenceRule, G: ProofGenerator<Rule,G>> ProofPromise<Rule,G> {
    pub fn resolve_once(&self) -> Result<ProofPromise<Rule,G>,ProofGenerationError> {
        Ok(match self {
            ProofPromise::Resolved(_) | ProofPromise::Composite(_) => self.clone(),
            ProofPromise::Generator(generator, conclusions) => generator.generate(conclusions)?,
        })
    }

    pub fn resolve(&self) -> Result<Proof<Rule>,ProofGenerationError> {
        Ok(match self {
            ProofPromise::Resolved(sub_proof) => sub_proof.clone(),
            ProofPromise::Composite(proof_promise) => Proof::Composite(proof_promise.resolve()?),
            ProofPromise::Generator(generator, conclusions) => generator.generate(conclusions)?.resolve()?,
        })
    }
}

#[derive(Clone)]
pub struct CompositeProofPromise<Rule: InferenceRule, G: ProofGenerator<Rule,G>> {
    pub premises: Vec<Proposition>,
    pub subproofs: Vec<ProofPromise<Rule,G>>,
    pub conclusions: Vec<Proposition>
}

impl <Rule: InferenceRule, G: ProofGenerator<Rule,G>> CompositeProofPromise<Rule,G> {
    pub fn resolve_once(&self) -> Result<CompositeProofPromise<Rule,G>,ErrorInProof<ProofGenerationError>> {
        let mut subproofs = Vec::new();
        for (i, proof) in self.subproofs.iter().enumerate() { match proof.resolve_once() {
            Ok(subproof) => subproofs.push(subproof),
            Err(err) => return Err(ErrorInProof::at_substep(i, err)),
        }}
        Ok(CompositeProofPromise { premises: self.premises.clone(), subproofs, conclusions: self.conclusions.clone() })
    }

    pub fn resolve(&self) -> Result<CompositeProof<Rule>,ProofGenerationError> {
        let mut subproofs = Vec::new();
        for proof in &self.subproofs { subproofs.push(proof.resolve()?) }
        Ok(CompositeProof::new(self.premises.clone(), subproofs, self.conclusions.clone()))
    }
}

#[derive(Clone)]
pub enum ProofGenerationError {}
