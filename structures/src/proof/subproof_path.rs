use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::{PathPrimitive, PathSeries}, HasChildren};

use crate::{inference::InferenceRule, proof::{CompositeProof, Proof}};

#[derive(Clone,PartialEq,Eq,Debug)]
/// Identifies a particular step iwthin a [`Proof`], and can be given to such a [`Proof`] to retreive the [`SubProof`] at that step
pub struct AtomicSubproofPath(usize);
impl PathPrimitive for AtomicSubproofPath {}
impl Display for AtomicSubproofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct SubproofPath(pub PathSeries<AtomicSubproofPath>);
impl SubproofPath {
    pub fn empty() -> Self { Self(PathSeries::empty()) }
    pub fn new<I: Into<AtomicSubproofPath>>(series: impl IntoIterator<Item=I>) -> Self { Self(PathSeries::new(series)) }
}
impl Display for SubproofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.paths().iter()
            .map(|atom| atom.to_string())
            .collect::<Vec<String>>()
            .join(".")
        )
    }
}

pub type SubproofInProof<'a,Rule> = ObjAtPath<'a,Proof<Rule>,SubproofPath>;
pub type OwnedSubproofInProof<Rule> = OwnedObjAtPath<Proof<Rule>,SubproofPath>;

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
}

impl <'a,Rule:'a + InferenceRule> HasChildren<'a,AtomicSubproofPath,Proof<Rule>> for CompositeProof<Rule> {
    fn valid_primitive_paths(&self) -> impl IntoIterator<Item = AtomicSubproofPath> 
        { (0..self.subproofs.len()).map(|ix| ix.into()) }
    fn get_child(&'a self, path: &AtomicSubproofPath) -> Result<&'a Proof<Rule>,()>
        { self.subproofs.get(path.0).ok_or(()) }
}

mod from {
    use crate::proof::{AtomicSubproofPath};
    
    impl <I: Into<usize>> From<I> for AtomicSubproofPath {
        fn from(value: I) -> Self { Self(value.into()) }
    }    
}
