use std::fmt::Display;

use path_lib::Path;
use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{DisplayExt, inference::{Inference, InferenceRule}, proof::{OwnedProofInProof, Proof, ProofInProof, ProofInProofPath}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct InferenceInProofPath(pub ProofInProofPath);
impl Path for InferenceInProofPath {}
impl Display for InferenceInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0.display())
    }
}

generate_obj_at_path_wrappers!{
    (Inference<Rule> where Rule: InferenceRule),InferenceInProofPath,
    "InferenceInProof", [Clone, PartialEq, Eq, Debug],
    "OwnedInferenceInProof", [Clone, PartialEq, Eq, Debug]
}

impl <'a,Rule: InferenceRule> TryFrom<ProofInProof<'a,Rule>> for InferenceInProof<'a,Rule> {
    type Error = ProofInProof<'a,Rule>;

    fn try_from(value: ProofInProof<'a,Rule>) -> Result<Self, Self::Error> {
        let (obj,path) = value.into_obj_and_path();
        if let Proof::Atomic(inference) = obj
            { Ok(InferenceInProof::from_inner(&inference, InferenceInProofPath(path))) }
        else { Err(ProofInProof::from_inner(&obj, path)) }
    }
}

impl <Rule: InferenceRule> TryFrom<OwnedProofInProof<Rule>> for OwnedInferenceInProof<Rule> {
    type Error = OwnedProofInProof<Rule>;

    fn try_from(value: OwnedProofInProof<Rule>) -> Result<Self, Self::Error> {
        let (obj,path) = value.into_obj_and_path();
        if let Proof::Atomic(inference) = obj
            { Ok(OwnedInferenceInProof::from_inner(inference, InferenceInProofPath(path))) }
        else { Err(OwnedProofInProof::from_inner(obj, path)) }
    }
}
