use std::{collections::HashSet, fmt::Display};

use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};

pub struct PropositionValueInequalityError {
    propositions: Vec<OwnedPropositionInProof>
}
impl PropositionValueInequalityError {
    pub fn new(propositions: Vec<OwnedPropositionInProof>) -> Self
        { Self { propositions } }
}
impl Display for PropositionValueInequalityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Proposition values expected to all be inequal, but weren't; {values}",
            values = self.propositions.iter().map(|o|
                o.0.path().to_string()
                + " -> " +
                &o.0.obj()
            ).collect::<Vec<_>>().join(", ")
        )
    }
}

/// Check that the provided [Propositions](OwnedPropositionInProof) have inequal value, returning an error otherwise
pub fn assert_proposition_value_inequality<'a, T: From<PropositionValueInequalityError>>(exprs: &[PropositionInProof]) -> Result<(), T> {
    let iter = exprs.iter().map(|o| match o.0.obj().as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionValueInequalityError::new(
            exprs.into_iter().map(|x| x.into_owned()).collect()
        ).into()); } }
    Ok(())
}
