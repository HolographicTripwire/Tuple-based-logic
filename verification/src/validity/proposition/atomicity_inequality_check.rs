use tbl_structures::proof::inference::{OwnedPropositionInInference, PropositionInInference};

pub struct PropositionAtomicityInequalityError {
    pub prop1: OwnedPropositionInInference,
    pub prop2: OwnedPropositionInInference,
}

/// Check that the provided [Propositions](PropositionInInference) have inequal atomicity, returning an error otherwise
pub fn assert_proposition_atomicity_inequality<'a>(prop1: &PropositionInInference, prop2: &PropositionInInference) -> Result<(), PropositionAtomicityInequalityError> {
    let first_atomicity = prop1.obj().as_atom().is_ok();
    let second_atomicity = prop2.obj().as_atom().is_ok();
    if first_atomicity == second_atomicity { Ok(()) }
    else { Err(PropositionAtomicityInequalityError{
        prop1: prop1.clone().into_owned(), 
        prop2: prop2.clone().into_owned()
    }) }
}
