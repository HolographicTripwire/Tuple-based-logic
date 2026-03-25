#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct AssumptionInProofStepPath(pub usize);
pub type AssumptionInSequentialProofStepPath = AssumptionInProofStepPath;
#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ConclusionInSequentialProofStepPath(pub usize);

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub enum PropositionInSequentialProofStepPath {
    Antecedent(AssumptionInProofStepPath),
    Consequent(ConclusionInSequentialProofStepPath)
}
impl PropositionInSequentialProofStepPath {
    pub fn new(is_conclusion: bool, proposition_index: usize) -> Self { match is_conclusion {
        true => Self::antecedent(proposition_index),
        false => Self::consequent(proposition_index),
    }}
    pub fn antecedent(assumption_index: usize) -> Self { Self::Antecedent(AssumptionInProofStepPath(assumption_index)) }
    pub fn consequent(conclusion_index: usize) -> Self { Self::Consequent(ConclusionInSequentialProofStepPath(conclusion_index)) }
}
