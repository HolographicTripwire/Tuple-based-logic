use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};

use crate::assertions::utils::stringify_atomicity;

pub struct PropositionAtomicityCheckError {
    expected_atomicity: bool,
    proposition: OwnedPropositionInProof
}
impl PropositionAtomicityCheckError {
    pub fn new(expected_atomicity: bool, proposition: OwnedPropositionInProof) -> Self
        { Self { expected_atomicity, proposition } }
    
}

pub fn format_proposition_atomicity_check_error(err: PropositionAtomicityCheckError) -> String {
    format!("Proposition at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
        path=err.proposition.0.path(),
        atomicity_expected=stringify_atomicity(err.expected_atomicity),
        atomicity_actual=stringify_atomicity(err.proposition.0.obj().as_atom().is_ok())
    )
}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_proposition_atomicity<'a,T: From<PropositionAtomicityCheckError>>(prop: PropositionInProof, expected_atomicity: bool) -> Result<(), T> {
    if prop.0.obj().as_atom().is_ok() == expected_atomicity { Ok(()) }
    else { Err(PropositionAtomicityCheckError::new(
        expected_atomicity, 
        prop.into_owned()
    ).into()) }
}
