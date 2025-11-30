use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};

use crate::assertions::utils::stringify_length;


pub struct PropositionLengthEqualityError {
    propositions: Vec<OwnedPropositionInProof>
}
impl PropositionLengthEqualityError {
    pub fn new(propositions: Vec<OwnedPropositionInProof>) -> Self
        { Self { propositions } }
}

pub fn format_proposition_length_equality_error(err: PropositionLengthEqualityError) -> String {
    format!("Proposition lengths expected to all be equal, but weren't; {atomicities}",
        atomicities = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}

/// Check that the provided [Propositions](OwnedPropositionInProof) have equal length, returning an error otherwise
pub fn assert_proposition_length_equality<'a, T: From<PropositionLengthEqualityError>>(props: &[PropositionInProof]) -> Result<Option<usize>, T> {
    let mut iter = props.iter().map(|o| match o.0.obj().as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let first_length = iter.next().expect("Cannot check length equality for zero propositions");
    for nth_length in iter {
        if nth_length != first_length { return Err(PropositionLengthEqualityError::new(
            props.into_iter().map(|x| x.clone().into_owned()).collect()
        ).into()) }
    }
    Ok(first_length)
}
