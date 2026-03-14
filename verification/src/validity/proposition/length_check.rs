use tbl_structures::proof::{OwnedPropositionInProofStep, PropositionInProofStep};

pub struct PropositionLengthCheckError {
    pub expected_length: usize,
    pub proposition: OwnedPropositionInProofStep
}
impl PropositionLengthCheckError {
    pub fn get_actual_length(&self) -> Option<usize> { self.proposition.obj().len() }
}

/// Check that the provided [Proposition](PropositionInProofStep) has an length equal to expected_length, returning an error otherwise
pub fn assert_proposition_length<'a>(expr: &PropositionInProofStep, expected_length: usize) -> Result<(), PropositionLengthCheckError> {
    match expr.obj().as_slice() {
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
