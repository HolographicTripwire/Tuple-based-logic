use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::structures::{inferences::InferenceRule, propositions::Proposition, sequential_proofs::{SequentialProof, at_path_enum::{OwnedSequentialProofAtPathEnum, SequentialProofAtPathEnum}, subproofs::immediate::ImmediateSequentialProofInProofPath}};

pub mod immediate;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct SequentialProofInProofPath(pub Vec<ImmediateSequentialProofInProofPath>);
impl Display for SequentialProofInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let components = self.0.clone().into_iter();
        let component_strings = components.map(|x| x.to_string());
        let joined = component_strings.collect::<Vec<_>>().join(".");
        write!(f,"{}",joined)
    }
}
mod from {
    use crate::structures::sequential_proofs::subproofs::{SequentialProofInProofPath, immediate::ImmediateSequentialProofInProofPath};

    impl From<ImmediateSequentialProofInProofPath> for SequentialProofInProofPath {
        fn from(value: ImmediateSequentialProofInProofPath) -> Self { Self(vec![value]) }
    }
    impl <const N: usize> From<[ImmediateSequentialProofInProofPath;N]> for SequentialProofInProofPath {
        fn from(values: [ImmediateSequentialProofInProofPath;N]) -> Self { Self(values.into()) }
    }
    impl From<Box<[ImmediateSequentialProofInProofPath]>> for SequentialProofInProofPath {
        fn from(values: Box<[ImmediateSequentialProofInProofPath]>) -> Self { Self(values.into()) }
    }
    impl From<Vec<ImmediateSequentialProofInProofPath>> for SequentialProofInProofPath {
        fn from(values: Vec<ImmediateSequentialProofInProofPath>) -> Self { Self(values) }
    }

    impl From<(ImmediateSequentialProofInProofPath,ImmediateSequentialProofInProofPath)> for SequentialProofInProofPath {
        fn from(values: (ImmediateSequentialProofInProofPath,ImmediateSequentialProofInProofPath)) -> Self { Self(vec![values.0,values.1]) }
    }
    impl From<(SequentialProofInProofPath,ImmediateSequentialProofInProofPath)> for SequentialProofInProofPath {
        fn from(mut values: (SequentialProofInProofPath,ImmediateSequentialProofInProofPath)) -> Self {
            values.0.0.push(values.1);
            values.0
        }
    }
    impl From<(ImmediateSequentialProofInProofPath,SequentialProofInProofPath)> for SequentialProofInProofPath {
        fn from(mut values: (ImmediateSequentialProofInProofPath,SequentialProofInProofPath)) -> Self {
            values.1.0.insert(0,values.0);
            values.1
        }
    }
    impl From<(SequentialProofInProofPath,SequentialProofInProofPath)> for SequentialProofInProofPath {
        fn from(mut values: (SequentialProofInProofPath,SequentialProofInProofPath)) -> Self {
            values.0.0.append(&mut values.1.0);
            values.0
        }
    }
}

generate_parent_of_children_trait!{
    SequentialProof<P,Rule>, SequentialProofInProofPath, (P: Proposition, Rule: InferenceRule<P>),
    "subproof", "subproofs", "Subproofs"
}

pub type SequentialProofAtPath<'a,P,Rule,Path> = ObjAtPath<'a,SequentialProof<P,Rule>,Path>;
pub type OwnedSequentialProofAtPath<P,Rule,Path> = OwnedObjAtPath<SequentialProof<P,Rule>,Path>;

pub type SequentialProofInProof<'a,P,Rule> = SequentialProofAtPath<'a,P,Rule,SequentialProofInProofPath>;
pub type SequentialProofInProofEnum<'a,P,Rule> = SequentialProofAtPathEnum<'a,P,SequentialProofInProofPath,Rule>;

pub type OwnedImmediateProofInProof<P,Rule> = OwnedSequentialProofAtPath<P,Rule,SequentialProofInProofPath>;
pub type OwnedImmediateProofInProofEnum<P,Rule> = OwnedSequentialProofAtPathEnum<P,SequentialProofInProofPath,Rule>;
