use tbl_structures::{expressions::Proposition, path_composites::{PropositionInProof, OwnedPropositionInProof}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct PropositionValueCheckError {
    expected_value: Proposition,
    proposition: OwnedPropositionInProof,
}
impl PropositionValueCheckError {
    pub fn new(expected_value: Proposition, proposition: OwnedPropositionInProof) -> Self
        { Self { expected_value, proposition } }
}

pub fn format_proposition_value_check_error(err: PropositionValueCheckError, style: ExpressionStyle) -> String {
    format!("Proposition at {path} has wrong value (expected {value_expected}; found {value_actual})",
        path=err.proposition.0.path(),
        value_expected=style.stringify(&err.expected_value),
        value_actual=style.stringify(err.proposition.0.obj())
    )
}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an value equal to expected_value, returning an error otherwise
pub fn assert_proposition_value<'a>(prop: &PropositionInProof, expected_value: &Proposition) -> Result<(), PropositionValueCheckError> {
    if prop.0.obj() == expected_value { Ok(()) }
    else { Err(PropositionValueCheckError::new(
        expected_value.clone(),
        prop.clone().into_owned()
    )) }
}
