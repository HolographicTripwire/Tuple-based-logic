use std::fmt::Display;

use tbl_structures::{expressions::Proposition, path_composites::{PropositionInProof, OwnedPropositionInProof}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};


pub struct PropositionValueEqualityError<'a> {
    propositions: Vec<OwnedPropositionInProof>,
    proposition_style: ExpressionStyle<'a>
}
impl <'a> PropositionValueEqualityError<'a> {
    pub fn new(propositions: Vec<OwnedPropositionInProof>, style: ExpressionStyle<'a>) -> Self
        { Self { propositions, proposition_style: style } }
}
impl <'a> Display for PropositionValueEqualityError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Proposition values expected to all be equal, but weren't; {atomicities}",
            atomicities = self.propositions.iter().map(|o|
                o.0.path().to_string()
                + " -> " +
                &self.proposition_style.stringify(o.0.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
}

/// Check that the provided [Propositions](OwnedPropositionInProof) have equal value, returning an error otherwise
pub fn assert_proposition_value_equality<'a, T: From<PropositionValueEqualityError<'a>>>(exprs: &[PropositionInProof], style: ExpressionStyle<'a>) -> Result<Proposition, T> {
    let mut iter = exprs.iter().map(|o| o.0.obj() );
    let first_value = iter.next().expect("Cannot check value equality for zero propositions");
    for nth_value in iter {
        if nth_value != first_value { return Err(PropositionValueEqualityError::new(
            exprs.into_iter().map(|x| x.into_owned()).collect(),
            style
        ).into()) }
    }
    Ok(first_value)
}
