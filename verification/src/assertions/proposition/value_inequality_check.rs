use std::collections::HashSet;

use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct PropositionValueInequalityError {
    propositions: Vec<OwnedPropositionInProof>,
}
impl PropositionValueInequalityError {
    pub fn new(propositions: Vec<OwnedPropositionInProof>) -> Self
        { Self { propositions } }
}


pub fn format_proposition_value_inequality_error(err: PropositionValueInequalityError, style: ExpressionStyle) -> String {
    format!("Proposition values expected to all be inequal, but weren't; {values}",
        values = err.propositions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}


/// Check that the provided [Propositions](OwnedPropositionInProof) have inequal value, returning an error otherwise
pub fn assert_proposition_value_inequality<'a, T: From<PropositionValueInequalityError>>(props: &[PropositionInProof]) -> Result<(), T> {
    let iter = props.iter().map(|o| match o.0.obj().as_slice() {
        Ok(propositions) => Some(propositions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(PropositionValueInequalityError::new(
            props.into_iter().map(|x| x.clone().into_owned()).collect()
        ).into()); } }
    Ok(())
}
