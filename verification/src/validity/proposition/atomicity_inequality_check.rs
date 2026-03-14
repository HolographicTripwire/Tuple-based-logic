use tbl_structures::proof::{OwnedPropositionInProofStep, PropositionInProofStep};

pub struct PropositionAtomicityInequalityError {
    pub prop1: OwnedPropositionInProofStep,
    pub prop2: OwnedPropositionInProofStep,
}

/// Check that the provided [Propositions](PropositionInProofStep) have inequal atomicity, returning an error otherwise
pub fn assert_proposition_atomicity_inequality<'a>(prop1: &PropositionInProofStep, prop2: &PropositionInProofStep) -> Result<(), PropositionAtomicityInequalityError> {
    let first_atomicity = prop1.obj().as_atom().is_ok();
    let second_atomicity = prop2.obj().as_atom().is_ok();
    if first_atomicity == second_atomicity { Ok(()) }
    else { Err(PropositionAtomicityInequalityError{
        prop1: prop1.clone().into_owned(), 
        prop2: prop2.clone().into_owned()
    }) }
}
