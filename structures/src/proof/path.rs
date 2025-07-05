use path_lib::{paths::{PathPrimitive, PathSeries}, ObjAtPath, HasChildren};

use crate::{inference::InferenceRule, proof::{CompositeProof, Proof}};

#[derive(Clone,PartialEq,Eq,Debug)]
/// Identifies a particular step iwthin a [`Proof`], and can be given to such a [`Proof`] to retreive the [`SubProof`] at that step
pub struct AtomicSubproofPath(usize);
impl PathPrimitive for AtomicSubproofPath {}
impl From<usize> for AtomicSubproofPath {
    fn from(value: usize) -> Self { Self(value) }
}

pub type SubproofPath = PathSeries<AtomicSubproofPath>;

pub type SubproofInProof<'a,Rule> = ObjAtPath<'a,Proof<Rule>,SubproofPath>;

impl <'a,Rule:'a + InferenceRule> HasChildren<'a,AtomicSubproofPath,Proof<Rule>> for Proof<Rule> {
    fn valid_primitive_paths(&'a self) -> impl IntoIterator<Item = AtomicSubproofPath> {
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
    fn valid_primitive_paths(&'a self) -> impl IntoIterator<Item = AtomicSubproofPath> 
        { (0..self.subproofs.len()).map(|ix| ix.into()) }
    fn get_child(&'a self, path: &AtomicSubproofPath) -> Result<&'a Proof<Rule>,()>
        { self.subproofs.get(path.0).ok_or(()) }
}
