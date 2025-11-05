use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, Path};

use crate::{DisplayExt, inference::{Inference, InferenceRule}, proof::{OwnedProofInProof, Proof, ProofInProof, ProofInProofPath}};

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

impl <'a,Rule: InferenceRule> TryFrom<ProofInProof<'a,Rule>> for InferenceInProof<'a,Rule> {
    type Error = ProofInProof<'a,Rule>;

    fn try_from(value: ProofInProof<'a,Rule>) -> Result<Self, Self::Error> {
        let (obj,path) = value.0.into_obj_and_path();
        if let Proof::Atomic(inference) = obj
            { Ok(InferenceInProof(ObjAtPath::from_at(inference, InferenceInProofPath(path)))) }
        else { Err(ProofInProof(ObjAtPath::from_at(obj, path))) }
    }
}

impl <Rule: InferenceRule> TryFrom<OwnedProofInProof<Rule>> for OwnedInferenceInProof<Rule> {
    type Error = OwnedProofInProof<Rule>;

    fn try_from(value: OwnedProofInProof<Rule>) -> Result<Self, Self::Error> {
        let (obj,path) = value.0.into_obj_and_path();
        if let Proof::Atomic(inference) = obj
            { Ok(OwnedInferenceInProof(OwnedObjAtPath::from_at(inference, InferenceInProofPath(path)))) }
        else { Err(OwnedProofInProof(OwnedObjAtPath::from_at(obj, path))) }
    }
}
