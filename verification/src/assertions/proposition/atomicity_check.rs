use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};

use crate::assertions::utils::stringify_atomicity;

pub struct PropositionAtomicityCheckError {
    pub expected_atomicity: bool,
    pub proposition: OwnedPropositionInInference
}

pub fn format_proposition_atomicity_check_error(err: PropositionAtomicityCheckError) -> String {
    format!("Proposition at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
        path=err.proposition.0.path(),
        atomicity_expected=stringify_atomicity(err.expected_atomicity),
        atomicity_actual=stringify_atomicity(err.proposition.0.obj().as_atom().is_ok())
    )
}

/// Check that the provided [Proposition](PropositionInInference) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_proposition_atomicity<'a>(prop: &PropositionInInference, expected_atomicity: bool) -> Result<(), PropositionAtomicityCheckError> {
    if prop.0.obj().as_atom().is_ok() == expected_atomicity { Ok(()) }
    else { Err(PropositionAtomicityCheckError{
        expected_atomicity, 
        proposition: prop.clone().into_owned() 
    }) }
}
