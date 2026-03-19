pub mod immediate;
mod proposition;

use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};

use crate::proof::Proof;

pub use proposition::*;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
/// Identifies a particular step iwthin a [`Proof`], and can be given to such a [`Proof`] to retreive the [`SubProof`] at that step
pub struct AtomicProofInProofPath(pub usize);
impl Display for AtomicProofInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

#[derive(Clone,PartialEq,Eq,Hash,Debug,Default)]
pub struct ProofInProofPath(pub Vec<AtomicProofInProofPath>);
impl Display for ProofInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter()
            .map(|atom| atom.to_string())
            .collect::<Vec<String>>().join(".")
        )
    }
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
    use crate::proof::{AtomicProofInProofPath};
    
    impl <I: Into<usize>> From<I> for AtomicProofInProofPath {
        fn from(value: I) -> Self { Self(value.into()) }
    }
}
