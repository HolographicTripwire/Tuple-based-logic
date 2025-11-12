use std::fmt::Display;

use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};

use crate::assertions::stringify_length;

pub struct PropositionLengthCheckError {
    expected_length: usize,
    proposition: OwnedPropositionInProof
}
impl PropositionLengthCheckError {
    pub fn new(expected_length: usize, proposition: OwnedPropositionInProof) -> Self
        { Self { expected_length, proposition } }
    
}
impl Display for PropositionLengthCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let proposition = self.proposition.0.obj();
        write!(f,"Proposition at {path} has wrong length (expected {length_expected}; found {length_actual})",
            path=self.proposition.0.path(),
            length_expected=stringify_length(proposition),
            length_actual=stringify_length(proposition)
        )
    }
}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an length equal to expected_length, returning an error otherwise
pub fn assert_proposition_length<'a,T: From<PropositionLengthCheckError>>(expr: &PropositionInProof, expected_length: usize) -> Result<(), T> {
    match expr.0.obj().as_slice() {
        Ok(tuple) => if tuple.len() == expected_length { Ok(()) }
        else {     Err(PropositionLengthCheckError::new(expected_length, expr).into()) },
        Err(()) => Err(PropositionLengthCheckError::new(expected_length, expr).into())
    }
}
