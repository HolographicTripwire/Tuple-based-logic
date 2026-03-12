use std::fmt::Display;

use itertools::Either;
use path_lib::{obj_at_path::{OwnedObjAtPath}, paths::{PathPrimitive, PathSeries}, HasChildren};
use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{DisplayExt, proof::{CompositeProof, Proof, inference::{InferenceInProof, InferenceRule, OwnedInferenceInProof}}};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
/// Identifies a particular step iwthin a [`Proof`], and can be given to such a [`Proof`] to retreive the [`SubProof`] at that step
pub struct AtomicProofInProofPath(pub(crate) usize);
impl PathPrimitive for AtomicProofInProofPath {}
impl Display for AtomicProofInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

pub type ProofInProofPath = PathSeries<AtomicProofInProofPath>;
impl DisplayExt for ProofInProofPath {
    fn display(&self) -> String {
        self.paths().iter()
            .map(|atom| atom.to_string())
            .collect::<Vec<String>>().join(".")
    }
}

generate_obj_at_path_wrappers!{
    (Proof<Rule> where Rule: InferenceRule), AtomicProofInProofPath,
    "ImmediateProofInProof", [Clone, PartialEq, Eq, Debug],
    "OwnedImmediateProofInProof", [Clone, PartialEq, Eq, Debug]
}

generate_obj_at_path_wrappers!{
    (Proof<Rule> where Rule: InferenceRule), ProofInProofPath,
    "ProofInProof", [Clone, PartialEq, Eq, Debug],
    "OwnedProofInProof", [Clone, PartialEq, Eq, Debug]
}
impl <'a,Rule: InferenceRule> ProofInProof<'a,Rule> {
    pub fn into_inference_or_composite(self) -> Either<InferenceInProof<'a,Rule>,CompositeProofInProof<'a,Rule>> {
        let (obj,path) = self.0.into_obj_and_path();
        match obj {
            Proof::Inference(inference) => Either::Left(InferenceInProof::from_inner(inference, path)),
            Proof::Composite(composite) => Either::Right(CompositeProofInProof::from_inner(composite, path)),
        }
    }
}
impl <Rule: InferenceRule> OwnedProofInProof<Rule> {
    pub fn into_inference_or_composite(self) -> Either<OwnedInferenceInProof<Rule>,OwnedCompositeProofInProof<Rule>> {
        let (obj,path) = self.0.into_obj_and_path();
        match obj {
            Proof::Inference(inference) => Either::Left(OwnedInferenceInProof::from_inner(inference, path)),
            Proof::Composite(composite) => Either::Right(OwnedCompositeProofInProof::from_inner(composite, path)),
        }
    }
}

generate_obj_at_path_wrappers!{
    (CompositeProof<Rule> where Rule: InferenceRule), ProofInProofPath,
    "CompositeProofInProof", [Clone, PartialEq, Eq, Debug],
    "OwnedCompositeProofInProof", [Clone, PartialEq, Eq, Debug]
}

impl <Rule:InferenceRule> HasChildren<AtomicProofInProofPath,Proof<Rule>> for Proof<Rule> {
    fn valid_primitive_paths(&self) -> Vec<AtomicProofInProofPath> {
        let max = if let Proof::Composite(composite) = self
            { composite.subproofs.len() } else { 0 };
        (0..max).map(|ix| ix.into()).collect()
    }
    
    fn get_child(&self, path: &AtomicProofInProofPath) -> Result<&Proof<Rule>,()> {
        if let Proof::Composite(composite) = self
            { composite.get_child(path) }
        else { Err(()) }
    }
    
    fn get_child_owned(&self, path: &AtomicProofInProofPath) -> Result<Proof<Rule>,()> where Proof<Rule>: Clone {
        if let Proof::Composite(composite) = self
            { composite.get_child_owned(path) }
        else { Err(()) }
    }
    
    fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicProofInProofPath>> where Proof<Rule>: Clone, Self: Sized {
        match self {
            Proof::Inference(_) => vec![],
            Proof::Composite(composite_proof) => <CompositeProof<Rule> as HasChildren<AtomicProofInProofPath,Proof<Rule>>>
                ::into_located_children_owned(composite_proof).into_iter().collect(),
        }
    }
}

mod from {
    use crate::proof::{AtomicProofInProofPath};
    
    impl <I: Into<usize>> From<I> for AtomicProofInProofPath {
        fn from(value: I) -> Self { Self(value.into()) }
    }
}
