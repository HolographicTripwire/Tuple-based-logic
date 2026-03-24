
pub struct PropositionValueCheckError {
    pub expected_value: TblProposition,
    pub proposition: OwnedPropositionInProofStep,
}
impl PropositionValueCheckError {
    pub fn get_actual_value(&self) -> &TblProposition { &self.proposition.obj }
    pub fn into_actual_value(self) -> TblProposition { self.proposition.obj }
}
/// Check that the provided [Proposition](PropositionInProofStep) has an value equal to expected_value, returning an error otherwise
pub fn assert_proposition_value<'a>(prop: &PropositionInProofStep, expected_value: &TblProposition) -> Result<(), PropositionValueCheckError> {
    if prop.obj == expected_value { Ok(()) }
    else { Err(PropositionValueCheckError{
        expected_value: expected_value.clone(),
        proposition: prop.clone().into()
    }) }
}
