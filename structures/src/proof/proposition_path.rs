use path_lib::{obj_at_path::ObjAtPath, paths::PathPrimitive};

use crate::expressions::Proposition;

#[derive(Clone)]
pub struct ProofPropositionPath {
    pub is_conclusion: bool,
    pub proposition_index: usize
}
impl ProofPropositionPath {
    pub fn new(is_conclusion: bool, proposition_index: usize) -> Self { Self { is_conclusion, proposition_index } }
    pub fn assumption(assumption_index: usize) -> Self { Self::new(false, assumption_index) }
    pub fn conclusion(conclusion_index: usize) -> Self { Self::new(true, conclusion_index) }
}
impl PathPrimitive for ProofPropositionPath {}

pub type PropositionInInference<'a> = ObjAtPath<'a,Proposition,ProofPropositionPath>;

mod into {
    use path_lib::paths::PathSeries;

    use super::*;

    impl Into<PathSeries<ProofPropositionPath>> for ProofPropositionPath {
        fn into(self) -> PathSeries<ProofPropositionPath> { PathSeries::new([self]) }
    }
}
