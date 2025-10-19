use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, Path};

use crate::{expressions::Proposition, proof::{ProofInProofPath, PropositionInProofStepPath}, DisplayExt};

#[derive(Clone,PartialEq,Eq)]
pub struct PropositionInProofPath {
    pub step_path: ProofInProofPath,
    pub proposition_path: PropositionInProofStepPath,
}
impl PropositionInProofPath {
    pub fn new(step: ProofInProofPath, proposition: PropositionInProofStepPath) -> Self { Self { step_path: step, proposition_path: proposition } }
}
impl Path for PropositionInProofPath {}
impl Display for PropositionInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.step_path.display(),self.proposition_path)
    }
}

pub type PropositionInProof<'a> = ObjAtPath<'a,Proposition,PropositionInProofStepPath>;
pub type OwnedPropositionInProof = OwnedObjAtPath<Proposition,PropositionInProofStepPath>;
