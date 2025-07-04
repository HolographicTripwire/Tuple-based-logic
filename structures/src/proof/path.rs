use path_lib::{paths::{PathPrimitive, PathSeries}, HasChildren};

use crate::{inference::{Inference, InferenceRule}, proof::{CompositeProof, Proof}};

#[derive(Clone,PartialEq,Eq,Debug)]
/// Identifies a particular step iwthin a [`Proof`], and can be given to such a [`Proof`] to retreive the [`SubProof`] at that step
pub struct AtomicSubproofPath(usize);
impl PathPrimitive for AtomicSubproofPath {}
impl From<usize> for AtomicSubproofPath {
    fn from(value: usize) -> Self { Self(value) }
}

pub type SubproofPath = PathSeries<AtomicSubproofPath>;

impl <'a,Rule:'a + InferenceRule> HasChildren<'a,AtomicSubproofPath,Proof<Rule>> for Proof<Rule> {
    fn children(&'a self) -> impl IntoIterator<Item = &'a Proof<Rule>> {
        if let Proof::Composite(composite) = self
            { composite.subproofs.iter().collect() }
        else { vec![] }
    }

    fn get_child(&'a self, path: &AtomicSubproofPath) -> Result<&'a Proof<Rule>,()> {
        if let Proof::Composite(composite) = self
            { composite.get_child(path) }
        else { Err(()) }
    }
}

impl <'a,Rule:'a + InferenceRule> HasChildren<'a,AtomicSubproofPath,Proof<Rule>> for CompositeProof<Rule> {
    fn children(&'a self) -> impl IntoIterator<Item = &'a Proof<Rule>> { self.subproofs.iter() }
    fn get_child(&'a self, path: &AtomicSubproofPath) -> Result<&'a Proof<Rule>,()>
        { self.subproofs.get(path.0).ok_or(()) }
}

impl <'a,Rule:'a + InferenceRule> HasChildren<'a,AtomicSubproofPath,Proof<Rule>> for Inference<Rule> {
    fn children(&'a self) -> impl IntoIterator<Item = &'a Proof<Rule>> { vec![] }
    fn get_child(&'a self, _: &AtomicSubproofPath) -> Result<&'a Proof<Rule>,()> { Err(()) }
}
