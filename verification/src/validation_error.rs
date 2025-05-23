use tbl_structures::propositions::PropositionSet;

#[derive(Clone)]
pub enum ProofValidationError {
    AssumptionsNotFound(PropositionSet),
    ConclusionsNotFound(PropositionSet),
    InvalidStepSpecification,
}
