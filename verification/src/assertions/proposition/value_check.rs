use std::fmt::Display;

use tbl_structures::{expressions::Proposition, path_composites::{PropositionInProof, OwnedPropositionInProof}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct PropositionValueCheckError<'a> {
    expected_value: Proposition,
    proposition: OwnedPropositionInProof,
    proposition_style: ExpressionStyle<'a>
}
impl <'a> PropositionValueCheckError<'a> {
    pub fn new(expected_value: Proposition, proposition: OwnedPropositionInProof, style: ExpressionStyle<'a>) -> Self
        { Self { expected_value, proposition, proposition_style: style } }
    
}
impl <'a> Display for PropositionValueCheckError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Proposition at {path} has wrong value (expected {value_expected}; found {value_actual})",
            path=self.proposition.0.path(),
            value_expected=self.proposition_style.stringify(&self.expected_value),
            value_actual=self.proposition_style.stringify(self.proposition.0.obj())
        )
    }
}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an value equal to expected_value, returning an error otherwise
pub fn assert_proposition_value<'a>(expr: &PropositionInProof, expected_value: &Proposition, style: ExpressionStyle<'a>) -> Result<(), PropositionValueCheckError<'a>> {
    if expr.0.obj() == expected_value { Ok(()) }
    else { Err(PropositionValueCheckError::new(
        expected_value.clone(),
        expr.into_owned(),
        style
    )) }
}
