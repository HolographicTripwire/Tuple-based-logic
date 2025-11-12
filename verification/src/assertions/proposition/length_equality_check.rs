use std::fmt::Display;

use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};

use crate::assertions::stringify_length;


pub struct PropositionLengthEqualityError {
    propositions: Vec<OwnedPropositionInProof>
}
impl PropositionLengthEqualityError {
    pub fn new(propositions: Vec<OwnedPropositionInProof>) -> Self
        { Self { propositions } }
}
impl Display for PropositionLengthEqualityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Proposition lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = self.propositions.iter().map(|o|
                o.0.path().to_string()
                + " -> " +
                &stringify_length(o.0.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
}

/// Check that the provided [Propositions](OwnedPropositionInProof) have equal length, returning an error otherwise
pub fn assert_proposition_length_equality<'a, T: From<PropositionLengthEqualityError>>(exprs: &[PropositionInProof]) -> Result<Option<usize>, T> {
    let mut iter = exprs.iter().map(|o| match o.0.obj().as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let first_length = iter.next().expect("Cannot check length equality for zero propositions");
    for nth_length in iter {
        if nth_length != first_length { return Err(PropositionLengthEqualityError::new(
            exprs.into_iter().map(|x| x.into_owned()).collect()
        ).into()) }
    }
    Ok(first_length)
}
