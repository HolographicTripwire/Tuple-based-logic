use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::structures::{Proposition, inferences::InferenceRule, sequential_proofs::{SequentialProof, at_path_enum::{OwnedProofAtPathEnum, ProofAtPathEnum}}};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
/// Identifies a particular step iwthin a [`Proof`], and can be given to such a [`Proof`] to retreive the [`SubProof`] at that step
pub struct ImmediateProofInProofPath(pub usize);
impl From<usize> for ImmediateProofInProofPath {
    fn from(value: usize) -> Self { Self(value) }
}
impl Display for ImmediateProofInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

generate_parent_of_children_trait!{
    (SequentialProof<P,Rule> where P: Proposition, Rule: InferenceRule<P>), ImmediateProofInProofPath,
    "immediate_subproof", "immediate_subproofs", "ImmediateSubproofs"
}

pub type ImmediateProofInProof<'a,P,Rule> = ObjAtPath<'a,SequentialProof<P,Rule>,ImmediateProofInProofPath>;
pub type ImmediateSubexpressionInExpressionEnum<'a,P,Rule> = ProofAtPathEnum<'a,P,ImmediateProofInProofPath,Rule>;

pub type OwnedImmediateProofInProof<P,Rule> = OwnedObjAtPath<SequentialProof<P,Rule>,ImmediateProofInProofPath>;
pub type OwnedImmediateProofInProofEnum<P,Rule> = OwnedProofAtPathEnum<P,ImmediateProofInProofPath,Rule>;
