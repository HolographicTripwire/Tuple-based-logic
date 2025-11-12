use tbl_structures::expressions::PropositionSet;

#[derive(Clone)]
pub enum ProofValidationError<InferenceErr> {
    AssumptionsNotFound(PropositionSet),
    ConclusionsNotFound(PropositionSet),
    InvalidInference(InferenceErr),
}
