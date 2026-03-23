use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::sequential_proofs::in_proof::{ProofInProofPath};

pub type ErrorInProof<'a,E> = ObjAtPath<'a,E,ProofInProofPath>;
pub type OwnedErrorInProof<E> = OwnedObjAtPath<E,ProofInProofPath>;
