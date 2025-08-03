use tbl_structures::expressions::PropositionSet;

use super::ProofStepSpecificationError;

#[derive(Clone)]
pub enum ProofValidationError {
    AssumptionsNotFound(PropositionSet),
    ConclusionsNotFound(PropositionSet),
    InvalidStepSpecification(ProofStepSpecificationError),
}
