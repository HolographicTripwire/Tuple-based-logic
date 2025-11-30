use std::collections::HashSet;

use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};

use crate::assertions::utils::stringify_length;

pub struct PropositionLengthInequalityError {
    propositions: Vec<OwnedPropositionInProof>
}
impl PropositionLengthInequalityError {
    pub fn new(propositions: Vec<OwnedPropositionInProof>) -> Self
        { Self { propositions } }
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


/// Check that the provided [Propositions](OwnedPropositionInProof) have inequal length, returning an error otherwise
pub fn assert_proposition_length_inequality<'a, T: From<PropositionLengthInequalityError>>(props: &[PropositionInProof]) -> Result<(), T> {
    let iter = props.iter().map(|o| match o.0.obj().as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionLengthInequalityError::new(
            props.into_iter().map(|x| x.clone().into_owned()).collect()
        ).into()); } }
    Ok(())
}
