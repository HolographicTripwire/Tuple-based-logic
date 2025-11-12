use std::fmt::Display;

use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};

use crate::assertions::utils::stringify_atomicity;

pub struct PropositionAtomicityEqualityError {
    propositions: Vec<OwnedPropositionInProof>
}
impl PropositionAtomicityEqualityError {
    pub fn new(propositions: Vec<OwnedPropositionInProof>) -> Self
        { Self { propositions } }
}
impl Display for PropositionAtomicityEqualityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Proposition atomicities expected to all be equal, but weren't; {atomicities}",
            atomicities = itertools::join(self.propositions.iter().map(|o|
                o.0.path().to_string()
                + " -> " +
                stringify_atomicity(o.0.obj().as_atom().is_ok())
            ),", ")
        )
    }
}

/// Check that the provided [Propositions](OwnedPropositionInProof) have equal atomicity, returning an error otherwise
pub fn assert_proposition_atomicity_equality<'a, T: From<PropositionAtomicityEqualityError>>(exprs: &[PropositionInProof]) -> Result<(), T> {
    let mut iter = exprs.iter().map(|o| o.0.obj().as_atom().is_ok());
    let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero propositions");
    for nth_atomicity in iter {
        if nth_atomicity != first_atomicity { return Err(PropositionAtomicityEqualityError::new(
            exprs.into_iter().map(|x| x.into_owned()).collect()
        ).into()) }
    }
    Ok(())
}
