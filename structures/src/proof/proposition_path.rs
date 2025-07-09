use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::PathPrimitive};

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

pub type PropositionInProof<'a> = ObjAtPath<'a,Proposition,ProofPropositionPath>;
pub type OwnedPropositionInProof = OwnedObjAtPath<Proposition,ProofPropositionPath>;
