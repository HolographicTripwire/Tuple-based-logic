use shared::propositions::PropositionSet;

#[derive(Clone)]
pub enum ProofValidationError {
    AssumptionsNotFound(PropositionSet),
    ConclusionsNotFound(PropositionSet),
    InvalidStepSpecification,
}
