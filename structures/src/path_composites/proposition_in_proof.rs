use std::fmt::Display;

use path_lib::{Path, paths::PathPair};
use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{expressions::Proposition, proof::{InferenceInProofPath, PropositionInInferencePath}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct PropositionInProofPath {
    pub step_path: InferenceInProofPath,
    pub proposition_path: PropositionInInferencePath,
}
impl PropositionInProofPath {
    pub fn new(step: InferenceInProofPath, proposition: PropositionInInferencePath) -> Self { Self { step_path: step, proposition_path: proposition } }
}
impl Path for PropositionInProofPath {}
impl Display for PropositionInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.step_path,self.proposition_path)
    }
}

generate_obj_at_path_wrappers!{
    (Proposition), PropositionInProofPath,
    "ExpressionInExpression", [Clone, PartialEq, Eq, Debug],
    "OwnedExpressionInExpression", [Clone, PartialEq, Eq, Debug]
}

mod from {
    use crate::proof::InferenceInProofPath;

    use super::*;

    impl From<PathPair<InferenceInProofPath,PropositionInInferencePath>> for PropositionInProofPath {
        fn from(pair: PathPair<InferenceInProofPath,PropositionInInferencePath>) -> Self { 
            Self::new(pair.left, pair.right)
        }
    }
}
