use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::PathPrimitive, Path};

use crate::{expressions::Proposition, proof::SubproofPath, DisplayExt};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ProofStepPropositionPath {
    pub is_conclusion: bool,
    pub proposition_index: usize
}
impl ProofStepPropositionPath {
    pub fn new(is_conclusion: bool, proposition_index: usize) -> Self { Self { is_conclusion, proposition_index } }
    pub fn assumption(assumption_index: usize) -> Self { Self::new(false, assumption_index) }
    pub fn conclusion(conclusion_index: usize) -> Self { Self::new(true, conclusion_index) }
}
impl PathPrimitive for ProofStepPropositionPath {}

impl Display for ProofStepPropositionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_conclusion { write!(f,"C{}",self.proposition_index) }
        else { write!(f,"A{}",self.proposition_index) }
    }
}

#[derive(Clone)]
pub struct ProofPropositionPath {
    pub step_path: SubproofPath,
    pub proposition_path: ProofStepPropositionPath,
}
impl ProofPropositionPath {
    pub fn new(step: SubproofPath, proposition: ProofStepPropositionPath) -> Self { Self { step_path: step, proposition_path: proposition } }
}
impl Path for ProofPropositionPath {
    
}
impl Display for ProofPropositionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.step_path.display(),self.proposition_path)
    }
}

pub type PropositionInProof<'a> = ObjAtPath<'a,Proposition,ProofStepPropositionPath>;
pub type OwnedPropositionInProof = OwnedObjAtPath<Proposition,ProofStepPropositionPath>;
