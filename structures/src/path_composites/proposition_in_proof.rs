use std::fmt::Display;

use path_lib::{Path, paths::PathPair};
use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{DisplayExt, expressions::Proposition, proof::{ProofInProofPath, PropositionInProofStepPath}};

#[derive(Clone,PartialEq,Eq,Debug)]
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

generate_obj_at_path_wrappers!{
    (Proposition), PropositionInProofPath,
    "ExpressionInExpression", [Clone, PartialEq, Eq, Debug],
    "OwnedExpressionInExpression", [Clone, PartialEq, Eq, Debug]
}

mod from {
    use crate::proof::ProofInProofPath;

    use super::*;

    impl From<PathPair<ProofInProofPath,PropositionInProofStepPath>> for PropositionInProofPath {
        fn from(pair: PathPair<ProofInProofPath,PropositionInProofStepPath>) -> Self { 
            Self::new(pair.left, pair.right)
        }
    }
}
