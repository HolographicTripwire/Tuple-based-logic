use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::structures::{propositions::paths::PropositionInSequentialProofStepPath, sequential_proofs::subproofs::ProofInProofPath};

use crate::structures::expressions::compound::CompoundTblExpression;

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct PropositionInProofPath {
    pub step_path: ProofInProofPath,
    pub proposition_path: PropositionInSequentialProofStepPath,
}
impl PropositionInProofPath {
    pub fn new(step: ProofInProofPath, proposition: PropositionInSequentialProofStepPath) -> Self { Self { step_path: step, proposition_path: proposition } }
}
impl Display for PropositionInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.step_path,self.proposition_path)
    }
}

pub type TblPropositionInProof<'a,C: CompoundTblExpression> = ObjAtPath<'a,C,PropositionInProofPath>;
pub type OwnedTblPropositionInProof<C: CompoundTblExpression> = OwnedObjAtPath<C,PropositionInProofPath>;

mod from {
    
    use super::*;

    impl From<(ProofInProofPath,PropositionInSequentialProofStepPath)> for PropositionInProofPath {
        fn from(pair: (ProofInProofPath,PropositionInSequentialProofStepPath)) -> Self { 
            Self::new(pair.0, pair.1)
        }
    }
}
