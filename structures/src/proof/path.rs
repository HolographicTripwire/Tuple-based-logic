use path_lib::{paths::{PathPrimitive, PathSeries}, HasChildren};

use crate::{inference::InferenceRule, proof::Proof};

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
        if let Proof::Composite(_,subproofs,_) = self
            { subproofs.into_iter().collect() }
        else { vec![] }
    }

    fn get_child(&'a self, path: &AtomicSubproofPath) -> Result<&'a Proof<Rule>,()> {
        if let Proof::Composite(_,subproofs,_) = self
            { subproofs.get(path.0).ok_or(()) }
        else { Err(()) }
    }
}
