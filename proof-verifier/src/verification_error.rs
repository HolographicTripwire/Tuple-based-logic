use shared::proposition::PropositionSet;

#[derive(Clone)]
pub enum ProofVerificationError {
    AssumptionsNotFound(PropositionSet),
    ConclusionsNotFound(PropositionSet),
    InvalidStepSpecification,
}
