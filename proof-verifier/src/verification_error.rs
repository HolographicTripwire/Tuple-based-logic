use shared::proposition::PropositionSet;

#[derive(Clone)]
pub enum VerificationError {
    AssumptionsNotFound(PropositionSet),
    ConclusionsNotFound(PropositionSet),
    InvalidStepSpecification,
}
