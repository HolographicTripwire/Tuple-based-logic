use std::collections::HashSet;

use tbl_structures::sequential_proofs::{OwnedPropositionInProofStep, PropositionInProofStep};

pub struct PropositionLengthInequalityError {
    pub propositions: Vec<OwnedPropositionInProofStep>
}


/// Check that the provided [Propositions](PropositionInProofStep) have inequal length, returning an error otherwise
pub fn assert_proposition_length_inequality<'a>(props: &[&'a PropositionInProofStep<'a>]) -> Result<(), PropositionLengthInequalityError> {
    let iter = props.iter().map(|o| match o.obj.as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionLengthInequalityError {
            propositions: props.into_iter().map(|x| (*x).clone().into()).collect()
        }); } }
    Ok(())
}





pub struct FixedLengthPropositionLengthInequalityError<const N: usize> {
    pub propositions: [OwnedPropositionInProofStep; N]
}
/// Check that the provided [Propositions](PropositionInProofStep) have inequal length, returning an error otherwise
pub fn assert_fixed_length_proposition_length_inequality<'a,const N: usize>(exprs: &[&'a PropositionInProofStep<'a>; N]) -> Result<(), FixedLengthPropositionLengthInequalityError<N>> {
    if N == 0 { panic!("Cannot check length inequality for zero propositions") } 
    let iter = exprs.iter().map(|o| o.obj.len());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthPropositionLengthInequalityError {
            propositions: exprs.clone().map(|x| (*x).clone().into())
        }); } }
    Ok(())
}
