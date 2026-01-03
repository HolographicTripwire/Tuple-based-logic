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

#[derive(Clone,PartialEq,Eq)]
pub struct ProofInProof<'a,Rule: InferenceRule>(pub ObjAtPath<'a,Proof<Rule>,ProofInProofPath>);
#[derive(Clone,PartialEq,Eq)]
pub struct OwnedProofInProof<Rule: InferenceRule>(pub OwnedObjAtPath<Proof<Rule>,ProofInProofPath>);

impl <Rule:InferenceRule> HasChildren<AtomicSubproofPath,Proof<Rule>> for Proof<Rule> {
    fn valid_primitive_paths(&self) -> Vec<AtomicSubproofPath> {
        let max = if let Proof::Composite(composite) = self
            { composite.subproofs.len() } else { 0 };
        (0..max).map(|ix| ix.into()).collect()
    }
    
    fn get_child(&self, path: &AtomicSubproofPath) -> Result<&Proof<Rule>,()> {
        if let Proof::Composite(composite) = self
            { composite.get_child(path) }
        else { Err(()) }
    }
    
    fn get_child_owned(&self, path: &AtomicSubproofPath) -> Result<Proof<Rule>,()> where Proof<Rule>: Clone {
        if let Proof::Composite(composite) = self
            { composite.get_child_owned(path) }
        else { Err(()) }
    }
    
    fn to_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicSubproofPath>> where Proof<Rule>: Clone, Self: Sized {
        match self {
            Proof::Atomic(_) => vec![],
            Proof::Composite(composite_proof) => <CompositeProof<Rule> as HasChildren<AtomicSubproofPath,Proof<Rule>>>
                ::to_located_children_owned(composite_proof).into_iter().collect(),
        }
    }
}

impl <Rule:InferenceRule> HasChildren<AtomicSubproofPath,Proof<Rule>> for CompositeProof<Rule> {
    fn valid_primitive_paths(&self) -> Vec<AtomicSubproofPath> 
        { (0..self.subproofs.len()).map(|ix| ix.into()).collect() }
    fn get_child(&self, path: &AtomicSubproofPath) -> Result<&Proof<Rule>,()>
        { self.subproofs.get(path.0).ok_or(()) }
    fn get_child_owned(&self, path: &AtomicSubproofPath) -> Result<Proof<Rule>,()> where Proof<Rule>: Clone
        { self.subproofs.get(path.0).ok_or(()).cloned() }
        
    fn to_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicSubproofPath>> where Proof<Rule>: Clone, Self: Sized {
        self.subproofs.into_iter()
            .enumerate()
            .map(|(id,proof)| OwnedObjAtPath::from_at(proof, AtomicSubproofPath(id)))
    }
}

mod from {
    use crate::proof::{AtomicSubproofPath};
    
    impl <I: Into<usize>> From<I> for AtomicSubproofPath {
        fn from(value: I) -> Self { Self(value.into()) }
    }    
}
