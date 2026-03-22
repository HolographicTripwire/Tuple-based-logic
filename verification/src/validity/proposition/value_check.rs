use tbl_structures::{expressions::Proposition, proof::{OwnedPropositionInProofStep, PropositionInProofStep}};

pub struct PropositionValueCheckError {
    pub expected_value: Proposition,
    pub proposition: OwnedPropositionInProofStep,
}
impl PropositionValueCheckError {
    pub fn get_actual_value(&self) -> &Proposition { &self.proposition.obj }
    pub fn into_actual_value(self) -> Proposition { self.proposition.obj }
}
/// Check that the provided [Proposition](PropositionInProofStep) has an value equal to expected_value, returning an error otherwise
pub fn assert_proposition_value<'a>(prop: &PropositionInProofStep, expected_value: &Proposition) -> Result<(), PropositionValueCheckError> {
    if prop.obj == expected_value { Ok(()) }
    else { Err(PropositionValueCheckError{
        expected_value: expected_value.clone(),
        proposition: prop.clone().into()
    }) }
}
