use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, Path};

use crate::{DisplayExt, inference::{Inference, InferenceRule}, proof::{ProofInProofPath}};

#[derive(Clone,PartialEq,Eq)]
pub struct InferenceInProofPath(pub ProofInProofPath);
impl Path for InferenceInProofPath {}
impl Display for InferenceInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0.display())
    }
}

#[derive(Clone,PartialEq,Eq)]
pub struct InferenceInProof<'a,Rule: InferenceRule>(pub ObjAtPath<'a,Inference<Rule>,InferenceInProofPath>);
#[derive(Clone,PartialEq,Eq)]
pub struct OwnedInferenceInProof<Rule: InferenceRule>(pub OwnedObjAtPath<Inference<Rule>,InferenceInProofPath>);
