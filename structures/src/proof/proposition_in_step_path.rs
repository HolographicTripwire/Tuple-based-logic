use std::fmt::Display;

use path_lib::paths::PathPrimitive;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct PropositionInProofStepPath {
    pub is_conclusion: bool,
    pub proposition_index: usize
}
impl PropositionInProofStepPath {
    pub fn new(is_conclusion: bool, proposition_index: usize) -> Self { Self { is_conclusion, proposition_index } }
    pub fn assumption(assumption_index: usize) -> Self { Self::new(false, assumption_index) }
    pub fn conclusion(conclusion_index: usize) -> Self { Self::new(true, conclusion_index) }
}
impl PathPrimitive for PropositionInProofStepPath {}

impl Display for PropositionInProofStepPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_conclusion { write!(f,"C{}",self.proposition_index) }
        else { write!(f,"A{}",self.proposition_index) }
    }
}
