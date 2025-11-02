use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, Path, paths::PathPair};

use crate::{expressions::Proposition, proof::{InferenceInProofPath, PropositionInInferencePath}};

#[derive(Clone,PartialEq,Eq)]
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

#[derive(Clone,PartialEq,Eq)]
pub struct PropositionInProof<'a>(pub ObjAtPath<'a,Proposition,PropositionInProofPath>);
#[derive(Clone,PartialEq,Eq)]
pub struct OwnedPropositionInProof(pub OwnedObjAtPath<Proposition,PropositionInProofPath>);

mod from {
    use crate::proof::InferenceInProofPath;

    use super::*;

    impl From<PathPair<InferenceInProofPath,PropositionInInferencePath>> for PropositionInProofPath {
        fn from(pair: PathPair<InferenceInProofPath,PropositionInInferencePath>) -> Self { 
            Self::new(pair.left, pair.right)
        }
    }
}
