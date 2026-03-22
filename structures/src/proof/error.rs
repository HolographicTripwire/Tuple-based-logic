use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::proof::in_proof::{ProofInProofPath};

pub type ErrorInProof<'a,E> = ObjAtPath<'a,E,ProofInProofPath>;
pub type OwnedErrorInProof<E> = OwnedObjAtPath<E,ProofInProofPath>;
