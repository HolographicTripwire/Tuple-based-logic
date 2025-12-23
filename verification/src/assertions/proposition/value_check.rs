use tbl_structures::{expressions::Proposition, proof::{OwnedPropositionInInference, PropositionInInference}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct PropositionValueCheckError {
    pub expected_value: Proposition,
    pub proposition: OwnedPropositionInInference,
}
impl PropositionValueCheckError {
    pub fn get_actual_value(&self) -> &Proposition { self.proposition.0.obj() }
}

pub fn format_proposition_value_check_error(err: PropositionValueCheckError, style: ExpressionStyle) -> String {
    format!("Proposition at {path} has wrong value (expected {value_expected}; found {value_actual})",
        path=err.proposition.0.path(),
        value_expected=style.stringify(&err.expected_value),
        value_actual=style.stringify(err.proposition.0.obj())
    )
}

/// Check that the provided [Proposition](PropositionInInference) has an value equal to expected_value, returning an error otherwise
pub fn assert_proposition_value<'a>(prop: &PropositionInInference, expected_value: &Proposition) -> Result<(), PropositionValueCheckError> {
    if prop.0.obj() == expected_value { Ok(()) }
    else { Err(PropositionValueCheckError{
        expected_value: expected_value.clone(),
        proposition: prop.clone().into_owned()
    }) }
}
