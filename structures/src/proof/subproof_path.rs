use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::{PathPrimitive, PathSeries}, HasChildren};

use crate::{inference::InferenceRule, proof::{CompositeProof, Proof}, DisplayExt};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
/// Identifies a particular step iwthin a [`Proof`], and can be given to such a [`Proof`] to retreive the [`SubProof`] at that step
pub struct AtomicSubproofPath(usize);
impl PathPrimitive for AtomicSubproofPath {}
impl Display for AtomicSubproofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

pub type ProofInProofPath = PathSeries<AtomicSubproofPath>;
impl DisplayExt for ProofInProofPath {
    fn display(&self) -> String {
        self.paths().iter()
            .map(|atom| atom.to_string())
            .collect::<Vec<String>>().join(".")
    }
}

pub type ProofInProof<'a,Rule> = ObjAtPath<'a,Proof<Rule>,ProofInProofPath>;
pub type OwnedProofInProof<Rule> = OwnedObjAtPath<Proof<Rule>,ProofInProofPath>;

impl <'a,Rule:'a + InferenceRule> HasChildren<'a,AtomicSubproofPath,Proof<Rule>> for Proof<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = AtomicSubproofPath> {
        let max = if let Proof::Composite(composite) = self
            { composite.subproofs.len() } else { 0 };
        (0..max).map(|ix| ix.into())
    }
    
    fn get_child(&'a self, path: &AtomicSubproofPath) -> Result<&'a Proof<Rule>,()> {
        if let Proof::Composite(composite) = self
            { composite.get_child(path) }
        else { Err(()) }
    }
    
    fn get_child_owned(&self, path: &AtomicSubproofPath) -> Result<Proof<Rule>,()> where Proof<Rule>: Clone {
        if let Proof::Composite(composite) = self
            { composite.get_child_owned(path) }
        else { Err(()) }
    }
}

impl <'a,Rule:'a + InferenceRule> HasChildren<'a,AtomicSubproofPath,Proof<Rule>> for CompositeProof<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = AtomicSubproofPath> 
        { (0..self.subproofs.len()).map(|ix| ix.into()) }
    fn get_child(&'a self, path: &AtomicSubproofPath) -> Result<&'a Proof<Rule>,()>
        { self.subproofs.get(path.0).ok_or(()) }
    fn get_child_owned(&self, path: &AtomicSubproofPath) -> Result<Proof<Rule>,()> where Proof<Rule>: Clone
        { self.subproofs.get(path.0).ok_or(()).cloned() }
}

mod from {
    use crate::proof::{AtomicSubproofPath};
    
    impl <I: Into<usize>> From<I> for AtomicSubproofPath {
        fn from(value: I) -> Self { Self(value.into()) }
    }    
}
