use tbl_structures::proof::{OwnedPropositionInProofStep, PropositionInProofStep};

pub struct PropositionAtomicityCheckError {
    pub expected_atomicity: bool,
    pub proposition: OwnedPropositionInProofStep
}


/// Check that the provided [Proposition](PropositionInProofStep) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_proposition_atomicity<'a>(prop: &PropositionInProofStep, expected_atomicity: bool) -> Result<(), PropositionAtomicityCheckError> {
    if prop.obj().as_atom().is_ok() == expected_atomicity { Ok(()) }
    else { Err(PropositionAtomicityCheckError{
        expected_atomicity, 
        proposition: prop.clone().into_owned() 
    }) }
}
