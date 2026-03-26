#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct AssumptionIProofStepPath(pub usize);
impl From<usize> for AssumptionIProofStepPath {
    fn from(value: usize) -> Self { Self(value) }
}
#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ExplicitConclusionInProofStepPath(pub usize);

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub enum PropositionInSequentialProofStepPath {
    Assumption(AssumptionIProofStepPath),
    Conclusion(ExplicitConclusionInProofStepPath)
}
impl PropositionInSequentialProofStepPath {
    pub fn new(is_conclusion: bool, proposition_index: usize) -> Self { match is_conclusion {
        true => Self::assumption(proposition_index),
        false => Self::conclusion(proposition_index),
    }}
    pub fn assumption(assumption_index: usize) -> Self { Self::Assumption(AssumptionIProofStepPath(assumption_index)) }
    pub fn conclusion(conclusion_index: usize) -> Self { Self::Conclusion(ExplicitConclusionInProofStepPath(conclusion_index)) }
}
