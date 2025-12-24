use std::collections::HashSet;


use tbl_structures::proof::{OwnedPropositionInInference, PropositionInInference};

use crate::assertions::utils::stringify_length;

pub struct PropositionLengthInequalityError {
    pub propositions: Vec<OwnedPropositionInInference>
}

pub fn format_proposition_length_inequality_error(err: PropositionLengthInequalityError) -> String {
    format!("Proposition lengths expected to all be inequal, but weren't; {lengths}",
        lengths = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}


/// Check that the provided [Propositions](PropositionInInference) have inequal length, returning an error otherwise
pub fn assert_proposition_length_inequality<'a>(props: &[PropositionInInference]) -> Result<(), PropositionLengthInequalityError> {
    let iter = props.iter().map(|o| match o.0.obj().as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionLengthInequalityError {
            propositions: props.into_iter().map(|x| x.clone().into_owned()).collect()
        }); } }
    Ok(())
}





pub struct FixedLengthPropositionLengthInequalityError<const N: usize> {
    pub propositions: [OwnedPropositionInInference; N]
}
pub fn format_fixed_length_proposition_length_inequality_error<const N: usize>(err: FixedLengthPropositionLengthInequalityError<N>) -> String {
    format!("Proposition lengths expected to all be equal, but weren't; {atomicities}",
        atomicities = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}
/// Check that the provided [Propositions](PropositionInInference) have inequal length, returning an error otherwise
pub fn assert_fixed_length_proposition_length_inequality<'a,const N: usize>(exprs: &[PropositionInInference; N]) -> Result<(), FixedLengthPropositionLengthInequalityError<N>> {
    if N == 0 { panic!("Cannot check length inequality for zero propositions") } 
    let iter = exprs.iter().map(|o| o.0.obj().len());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthPropositionLengthInequalityError {
            propositions: exprs.clone().map(|x| x.clone().into_owned())
        }); } }
    Ok(())
}
