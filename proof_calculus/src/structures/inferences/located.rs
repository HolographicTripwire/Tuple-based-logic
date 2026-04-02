use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::{inferences::Inference, sequential_proofs::subproofs::SequentialProofInProofPath};

pub type InferenceAtPath<'a, Proposition,Rule,Path> = ObjAtPath<'a,Inference<Proposition,Rule>,Path>;
pub type OwnedInferenceAtPath<Proposition,Rule,Path> = OwnedObjAtPath<Inference<Proposition,Rule>,Path>;

pub type InferenceInProof<'a, Proposition,Rule> = ObjAtPath<'a,Inference<Proposition,Rule>,SequentialProofInProofPath>;
pub type OwnedInferenceInProof<Proposition,Rule> = OwnedObjAtPath<Inference<Proposition,Rule>,SequentialProofInProofPath>;
