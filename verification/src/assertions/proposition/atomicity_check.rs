use std::fmt::Display;

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
impl Display for PropositionAtomicityCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Proposition at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
            path=self.proposition.0.path(),
            atomicity_expected=stringify_atomicity(self.expected_atomicity),
            atomicity_actual=stringify_atomicity(self.proposition.0.obj().as_atom().is_ok())
        )
    }
}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_proposition_atomicity<'a,T: From<PropositionAtomicityCheckError>>(expr: &PropositionInProof, expected_atomicity: bool) -> Result<(), T> {
    if expr.0.obj().as_atom().is_ok() == expected_atomicity { Ok(()) }
    else { Err(PropositionAtomicityCheckError::new(
        expected_atomicity, 
        expr.into_owned() 
    ).into()) }
}
