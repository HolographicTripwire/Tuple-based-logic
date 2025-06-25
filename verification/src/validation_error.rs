use tbl_structures::propositions::PropositionSet;

use crate::inference_rules::error::ProofStepSpecificationError;

#[derive(Clone)]
pub enum ProofValidationError {
    AssumptionsNotFound(PropositionSet),
    ConclusionsNotFound(PropositionSet),
    InvalidStepSpecification(ProofStepSpecificationError),
}
