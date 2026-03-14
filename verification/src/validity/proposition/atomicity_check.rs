use tbl_structures::proof::inference::{OwnedPropositionInInference, PropositionInInference};

pub struct PropositionAtomicityCheckError {
    pub expected_atomicity: bool,
    pub proposition: OwnedPropositionInInference
}


/// Check that the provided [Proposition](PropositionInInference) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_proposition_atomicity<'a>(prop: &PropositionInInference, expected_atomicity: bool) -> Result<(), PropositionAtomicityCheckError> {
    if prop.obj().as_atom().is_ok() == expected_atomicity { Ok(()) }
    else { Err(PropositionAtomicityCheckError{
        expected_atomicity, 
        proposition: prop.clone().into_owned() 
    }) }
}
