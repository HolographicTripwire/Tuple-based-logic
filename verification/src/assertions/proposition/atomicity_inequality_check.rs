use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};



pub struct PropositionAtomicityInequalityError {
    pub prop1: OwnedPropositionInInference,
    pub prop2: OwnedPropositionInInference,
}

pub fn format_proposition_atomicity_inequality_error(err: PropositionAtomicityInequalityError) -> String {
    format!("Atomicity of propositions {prop1} and {prop2} expected to be inequal, but both were {value}",
            prop1 = err.prop1.0.path(),
            prop2 = err.prop2.0.path(),
            value = err.prop1.0.obj().as_atom().is_ok()
        )
}

/// Check that the provided [Propositions](PropositionInInference) have inequal atomicity, returning an error otherwise
pub fn assert_proposition_atomicity_inequality<'a>(prop1: &PropositionInInference, prop2: &PropositionInInference) -> Result<(), PropositionAtomicityInequalityError> {
    let first_atomicity = prop1.0.obj().as_atom().is_ok();
    let second_atomicity = prop2.0.obj().as_atom().is_ok();
    if first_atomicity == second_atomicity { Ok(()) }
    else { Err(PropositionAtomicityInequalityError{
        prop1: prop1.clone().into_owned(), 
        prop2: prop2.clone().into_owned()
    }) }
}
