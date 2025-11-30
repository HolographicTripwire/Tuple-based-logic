use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};

use crate::assertions::utils::stringify_atomicity;

pub struct PropositionAtomicityEqualityError {
    propositions: Vec<OwnedPropositionInProof>
}
impl PropositionAtomicityEqualityError {
    pub fn new(propositions: Vec<OwnedPropositionInProof>) -> Self
        { Self { propositions } }
}

pub fn format_proposition_atomicity_equality_error(err: PropositionAtomicityEqualityError) -> String {
    format!("Proposition atomicities expected to all be equal, but weren't; {atomicities}",
        atomicities = itertools::join(err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            stringify_atomicity(o.0.obj().as_atom().is_ok())
        ),", ")
    )
}

/// Check that the provided [Propositions](OwnedPropositionInProof) have equal atomicity, returning an error otherwise
pub fn assert_proposition_atomicity_equality<'a, T: From<PropositionAtomicityEqualityError>>(props: &[PropositionInProof]) -> Result<(), T> {
    let mut iter = props.iter().map(|o| o.0.obj().as_atom().is_ok());
    let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero propositions");
    for nth_atomicity in iter {
        if nth_atomicity != first_atomicity { return Err(PropositionAtomicityEqualityError::new(
            props.into_iter().map(|x| x.clone().into_owned()).collect()
        ).into()) }
    }
    Ok(())
}
