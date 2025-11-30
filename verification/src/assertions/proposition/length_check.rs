use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};

use crate::assertions::utils::stringify_length;

pub struct PropositionLengthCheckError {
    expected_length: usize,
    proposition: OwnedPropositionInProof
}
impl PropositionLengthCheckError {
    pub fn new(expected_length: usize, proposition: OwnedPropositionInProof) -> Self
        { Self { expected_length, proposition } }
    
}

pub fn format_proposition_length_check_error(err: PropositionLengthCheckError) -> String {
    let proposition = err.proposition.0.obj();
    format!("Proposition at {path} has wrong length (expected {length_expected}; found {length_actual})",
        path=err.proposition.0.path(),
        length_expected=stringify_length(proposition),
        length_actual=stringify_length(proposition)
    )
}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an length equal to expected_length, returning an error otherwise
pub fn assert_proposition_length<'a,T: From<PropositionLengthCheckError>>(prop: &PropositionInProof, expected_length: usize) -> Result<(), T> {
    match prop.0.obj().as_slice() {
        Ok(tuple) => if tuple.len() == expected_length { Ok(()) }
        else {     Err(PropositionLengthCheckError::new(expected_length, prop.clone().into_owned()).into()) },
        Err(()) => Err(PropositionLengthCheckError::new(expected_length, prop.clone().into_owned()).into())
    }
}
