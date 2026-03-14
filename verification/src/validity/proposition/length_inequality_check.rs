use std::collections::HashSet;

use tbl_structures::proof::inference::{OwnedPropositionInInference, PropositionInInference};

pub struct PropositionLengthInequalityError {
    pub propositions: Vec<OwnedPropositionInInference>
}


/// Check that the provided [Propositions](PropositionInInference) have inequal length, returning an error otherwise
pub fn assert_proposition_length_inequality<'a>(props: &[&'a PropositionInInference<'a>]) -> Result<(), PropositionLengthInequalityError> {
    let iter = props.iter().map(|o| match o.obj().as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionLengthInequalityError {
            propositions: props.into_iter().map(|x| (*x).clone().into_owned()).collect()
        }); } }
    Ok(())
}





pub struct FixedLengthPropositionLengthInequalityError<const N: usize> {
    pub propositions: [OwnedPropositionInInference; N]
}
/// Check that the provided [Propositions](PropositionInInference) have inequal length, returning an error otherwise
pub fn assert_fixed_length_proposition_length_inequality<'a,const N: usize>(exprs: &[&'a PropositionInInference<'a>; N]) -> Result<(), FixedLengthPropositionLengthInequalityError<N>> {
    if N == 0 { panic!("Cannot check length inequality for zero propositions") } 
    let iter = exprs.iter().map(|o| o.obj().len());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthPropositionLengthInequalityError {
            propositions: exprs.clone().map(|x| (*x).clone().into_owned())
        }); } }
    Ok(())
}
