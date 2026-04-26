use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::{propositions::assigned::Proposition, proofs::inferences::InferenceRule, proofs::sequential::{SequentialProof, at_path_enum::{OwnedSequentialProofAtPathEnum, SequentialProofAtPathEnum}}};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
/// Identifies a particular step iwthin a [`Proof`], and can be given to such a [`Proof`] to retreive the [`SubProof`] at that step
pub struct ImmediateSequentialProofInProofPath(pub usize);
impl From<usize> for ImmediateSequentialProofInProofPath {
    fn from(value: usize) -> Self { Self(value) }
}
impl Display for ImmediateSequentialProofInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

generate_parent_of_children_trait!{
    SequentialProof<P,Rule>, ImmediateSequentialProofInProofPath, (P: Proposition, Rule: InferenceRule<P>),
    "immediate_subproof", "immediate_subproofs", "ImmediateSubproofs"
}

pub type ImmediateProofInProof<'a,P,Rule> = ObjAtPath<'a,SequentialProof<P,Rule>,ImmediateSequentialProofInProofPath>;
pub type ImmediateProofInProofEnum<'a,P,Rule> = SequentialProofAtPathEnum<'a,P,ImmediateSequentialProofInProofPath,Rule>;

pub type OwnedImmediateProofInProof<P,Rule> = OwnedObjAtPath<SequentialProof<P,Rule>,ImmediateSequentialProofInProofPath>;
pub type OwnedImmediateProofInProofEnum<P,Rule> = OwnedSequentialProofAtPathEnum<P,ImmediateSequentialProofInProofPath,Rule>;
