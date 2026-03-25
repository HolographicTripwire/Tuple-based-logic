use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::sequential_proofs::subproof::immediate::ImmediateProofInProofPath;

mod immediate;

#[derive(Clone,PartialEq,Eq,Hash,Debug,Default)]
pub struct ProofInProofPath(pub Vec<ImmediateProofInProofPath>);
impl Display for ProofInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter()
            .map(|atom| atom.to_string())
            .collect::<Vec<String>>().join(".")
        )
    }
}

generate_parent_of_children_trait!{
    (Proof<Rule> where Rule: InferenceRule), ProofInProofPath,
    "subproof", "subproofs", "Subproofs"
}

pub type ProofInProof<'a,Rule> = ObjAtPath<'a,Proof<Rule>,ProofInProofPath>;
pub type OwnedProofInProof<Rule> = OwnedObjAtPath<Proof<Rule>,ProofInProofPath>;

mod from {
    use super::*;
    
    impl From<usize> for ProofInProofPath {
        fn from(value: usize) -> Self { value.into() }
    }
    impl From<ImmediateProofInProofPath> for ProofInProofPath {
        fn from(value: ImmediateProofInProofPath) -> Self { vec![value].into() }
    }
    impl From<(ImmediateProofInProofPath,ImmediateProofInProofPath)> for ProofInProofPath {
        fn from(value: (ImmediateProofInProofPath,ImmediateProofInProofPath)) -> Self { vec![value.0,value.1].into() }
    }
    impl From<Vec<ImmediateProofInProofPath>> for ProofInProofPath {
        fn from(value: Vec<ImmediateProofInProofPath>) -> Self { Self(value) }
    }

    impl From<(ProofInProofPath,ImmediateProofInProofPath)> for ProofInProofPath {
        fn from(mut value: (ProofInProofPath,ImmediateProofInProofPath)) -> Self {
            value.0.0.push(value.1);
            value.0
        }
    }
    impl From<(ImmediateProofInProofPath,ProofInProofPath)> for ProofInProofPath {
        fn from(mut value: (ImmediateProofInProofPath,ProofInProofPath)) -> Self {
            value.1.0.push(value.0);
            value.1
        }
    }
    impl From<(ProofInProofPath,ProofInProofPath)> for ProofInProofPath {
        fn from(mut value: (ProofInProofPath,ProofInProofPath)) -> Self {
            value.0.0.append(&mut value.1.0);
            value.0
        }
    }
}
