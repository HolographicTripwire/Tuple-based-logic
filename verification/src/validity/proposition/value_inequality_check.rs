use std::collections::HashSet;

use tbl_structures::proof::{OwnedPropositionInProofStep, PropositionInProofStep};

pub struct PropositionValueInequalityError {
    pub propositions: Vec<OwnedPropositionInProofStep>,
}
/// Check that the provided [Propositions](PropositionInProofStep) have inequal value, returning an error otherwise
pub fn assert_proposition_value_inequality<'a>(props: &[&'a PropositionInProofStep<'a>]) -> Result<(), PropositionValueInequalityError> {
    let iter = props.iter().map(|o| o.obj);
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionValueInequalityError{
            propositions: props.into_iter().map(|x| (*x).clone().into()).collect()
        }); } }
    Ok(())
}

pub struct FixedLengthPropositionValueInequalityError<const N: usize> {
    pub propositions: [OwnedPropositionInProofStep; N]
}
/// Check that the provided [Propositions](PropositionInProofStep) have inequal length, returning an error otherwise
pub fn assert_fixed_length_proposition_value_inequality<'a,const N: usize>(exprs: &[&'a PropositionInProofStep<'a>; N]) -> Result<(), FixedLengthPropositionValueInequalityError<N>> {
    if N == 0 { panic!("Cannot check length inequality for zero propositions") } 
    let iter = exprs.iter().map(|o| o.obj);
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthPropositionValueInequalityError {
            propositions: exprs.clone().map(|x| (*x).clone().into())
        }); } }
    Ok(())
}
