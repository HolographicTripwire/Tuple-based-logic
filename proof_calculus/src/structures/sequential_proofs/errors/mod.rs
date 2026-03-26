use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::sequential_proofs::subproofs::ProofInProofPath;

pub type ErrorInProof<'a,E> = ObjAtPath<'a,E,ProofInProofPath>;
pub type OwnedErrorInProof<E> = OwnedObjAtPath<E,ProofInProofPath>;
