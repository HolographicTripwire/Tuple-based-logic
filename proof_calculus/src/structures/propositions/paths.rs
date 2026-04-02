use std::fmt::Display;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct AssumptionInSequentialProofStepPath(pub usize);
impl From<usize> for AssumptionInSequentialProofStepPath {
    fn from(value: usize) -> Self { Self(value) }
}
impl Display for AssumptionInSequentialProofStepPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "A{}", self.0) }
}

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ExplicitConclusionInSequentialProofStepPath(pub usize);
impl From<usize> for ExplicitConclusionInSequentialProofStepPath {
    fn from(value: usize) -> Self { Self(value) }
}
impl Display for ExplicitConclusionInSequentialProofStepPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "C{}", self.0) }
}

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub enum PropositionInSequentialProofStepPath {
    Assumption(AssumptionInSequentialProofStepPath),
    Conclusion(ExplicitConclusionInSequentialProofStepPath)
}
impl PropositionInSequentialProofStepPath {
    pub fn new(is_conclusion: bool, proposition_index: usize) -> Self { match is_conclusion {
        true => Self::assumption(proposition_index),
        false => Self::conclusion(proposition_index),
    }}
    pub fn assumption(assumption_index: usize) -> Self { Self::Assumption(AssumptionInSequentialProofStepPath(assumption_index)) }
    pub fn conclusion(conclusion_index: usize) -> Self { Self::Conclusion(ExplicitConclusionInSequentialProofStepPath(conclusion_index)) }
}
impl Display for PropositionInSequentialProofStepPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self {
            PropositionInSequentialProofStepPath::Assumption(assumption_path) => write!(f, "{}", assumption_path),
            PropositionInSequentialProofStepPath::Conclusion(conclusion_path) => write!(f, "{}", conclusion_path),
    }}
}
