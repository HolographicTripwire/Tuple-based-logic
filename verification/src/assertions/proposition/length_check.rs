use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};

use crate::assertions::utils::stringify_length;

pub struct PropositionLengthCheckError {
    pub expected_length: usize,
    pub proposition: OwnedPropositionInInference
}
impl PropositionLengthCheckError {
    pub fn get_actual_length(&self) -> Option<usize> { self.proposition.0.obj().len() }
}

pub fn format_proposition_length_check_error(err: PropositionLengthCheckError) -> String {
    let proposition = err.proposition.0.obj();
    format!("Proposition at {path} has wrong length (expected {length_expected}; found {length_actual})",
        path=err.proposition.0.path(),
        length_expected=stringify_length(proposition),
        length_actual=stringify_length(proposition)
    )
}

/// Check that the provided [Proposition](PropositionInInference) has an length equal to expected_length, returning an error otherwise
pub fn assert_proposition_length<'a>(expr: &PropositionInInference, expected_length: usize) -> Result<(), PropositionLengthCheckError> {
    match expr.0.obj().as_slice() {
        Ok(tuple) => if tuple.len() == expected_length { Ok(()) }
        else { Err(PropositionLengthCheckError {
            expected_length,
            proposition: expr.clone().into_owned()
        }) },
        Err(()) => Err(PropositionLengthCheckError {
            expected_length, 
            proposition: expr.clone().into_owned()
        })
    }
}
