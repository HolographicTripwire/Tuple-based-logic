use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, Path};

use crate::{inference::{Inference}, proof::ProofInProofPath, DisplayExt};

#[derive(Clone,PartialEq,Eq)]
pub struct InferenceInProofPath(pub ProofInProofPath);
impl Path for InferenceInProofPath {}
impl Display for InferenceInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0.display())
    }
}
pub type InferenceInProof<'a,Rule> = ObjAtPath<'a,Inference<Rule>,InferenceInProofPath>;
pub type OwnedInferenceInProof<Rule> = OwnedObjAtPath<Inference<Rule>,InferenceInProofPath>;
