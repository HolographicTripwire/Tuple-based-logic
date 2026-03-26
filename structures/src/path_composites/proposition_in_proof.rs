use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::TblProposition, sequential_proofs::{ProofInProofPath, PropositionInProofStepPath}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct PropositionInProofPath {
    pub step_path: ProofInProofPath,
    pub proposition_path: PropositionInProofStepPath,
}
impl PropositionInProofPath {
    pub fn new(step: ProofInProofPath, proposition: PropositionInProofStepPath) -> Self { Self { step_path: step, proposition_path: proposition } }
}
impl Display for PropositionInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.step_path,self.proposition_path)
    }
}

pub type PropositionInProof<'a> = ObjAtPath<'a,TblProposition,PropositionInProofPath>;
pub type OwnedPropositionInProof = OwnedObjAtPath<TblProposition,PropositionInProofPath>;

mod from {
    use crate::sequential_proofs::ProofInProofPath;

    use super::*;

    impl From<(ProofInProofPath,PropositionInProofStepPath)> for PropositionInProofPath {
        fn from(pair: (ProofInProofPath,PropositionInProofStepPath)) -> Self { 
            Self::new(pair.0, pair.1)
        }
    }
}
