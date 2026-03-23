pub mod immediate;
mod proposition;

use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::sequential_proofs::{Proof, immediate::ImmediateProofInProofPath, inference::InferenceRule};

pub use proposition::*;


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



// impl <Rule:InferenceRule> HasChildren<AtomicProofInProofPath,Proof<Rule>> for Proof<Rule> {
//     fn valid_primitive_paths(&self) -> Vec<AtomicProofInProofPath> {
//         let max = if let Proof::Composite(composite) = self
//             { composite.subproofs.len() } else { 0 };
//         (0..max).map(|ix| ix.into()).collect()
//     }
    
//     fn get_child(&self, path: &AtomicProofInProofPath) -> Result<&Proof<Rule>,()> {
//         if let Proof::Composite(composite) = self
//             { composite.get_child(path) }
//         else { Err(()) }
//     }
    
//     fn get_child_owned(&self, path: &AtomicProofInProofPath) -> Result<Proof<Rule>,()> where Proof<Rule>: Clone {
//         if let Proof::Composite(composite) = self
//             { composite.get_child_owned(path) }
//         else { Err(()) }
//     }
    
//     fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicProofInProofPath>> where Proof<Rule>: Clone, Self: Sized {
//         match self {
//             Proof::Inference(_) => vec![],
//             Proof::Composite(composite_proof) => <CompositeProof<Rule> as HasChildren<AtomicProofInProofPath,Proof<Rule>>>
//                 ::into_located_children_owned(composite_proof).into_iter().collect(),
//         }
//     }
// }

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
