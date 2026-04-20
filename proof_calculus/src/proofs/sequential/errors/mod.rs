use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::proofs::sequential::subproofs::SequentialProofInProofPath;

pub type ErrorInProof<'a,E> = ObjAtPath<'a,E,SequentialProofInProofPath>;
pub type OwnedErrorInProof<E> = OwnedObjAtPath<E,SequentialProofInProofPath>;
