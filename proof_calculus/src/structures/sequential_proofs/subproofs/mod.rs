use std::fmt::Display;

use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::structures::{Proposition, inferences::InferenceRule, sequential_proofs::{SequentialProof, subproofs::immediate::ImmediateProofInProofPath}};

pub mod at_path_enum;
pub mod immediate;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub struct ProofInProofPath(pub Vec<ImmediateProofInProofPath>);
impl Display for ProofInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let components = self.0.clone().into_iter();
        let component_strings = components.map(|x| x.to_string());
        let joined = component_strings.collect::<Vec<_>>().join(".");
        write!(f,"{}",joined)
    }
}
mod from {
    use crate::structures::sequential_proofs::subproofs::{ProofInProofPath, immediate::ImmediateProofInProofPath};

    impl From<ImmediateProofInProofPath> for ProofInProofPath {
        fn from(value: ImmediateProofInProofPath) -> Self { Self(vec![value]) }
    }
    impl <const N: usize> From<[ImmediateProofInProofPath;N]> for ProofInProofPath {
        fn from(values: [ImmediateProofInProofPath;N]) -> Self { Self(values.into()) }
    }
    impl From<Box<[ImmediateProofInProofPath]>> for ProofInProofPath {
        fn from(values: Box<[ImmediateProofInProofPath]>) -> Self { Self(values.into()) }
    }
    impl From<Vec<ImmediateProofInProofPath>> for ProofInProofPath {
        fn from(values: Vec<ImmediateProofInProofPath>) -> Self { Self(values) }
    }

    impl From<(ImmediateProofInProofPath,ImmediateProofInProofPath)> for ProofInProofPath {
        fn from(values: (ImmediateProofInProofPath,ImmediateProofInProofPath)) -> Self { Self(vec![values.0,values.1]) }
    }
    impl From<(ProofInProofPath,ImmediateProofInProofPath)> for ProofInProofPath {
        fn from(mut values: (ProofInProofPath,ImmediateProofInProofPath)) -> Self {
            values.0.0.push(values.1);
            values.0
        }
    }
    impl From<(ImmediateProofInProofPath,ProofInProofPath)> for ProofInProofPath {
        fn from(mut values: (ImmediateProofInProofPath,ProofInProofPath)) -> Self {
            values.1.0.insert(0,values.0);
            values.1
        }
    }
    impl From<(ProofInProofPath,ProofInProofPath)> for ProofInProofPath {
        fn from(mut values: (ProofInProofPath,ProofInProofPath)) -> Self {
            values.0.0.append(&mut values.1.0);
            values.0
        }
    }
}

generate_parent_of_children_trait!{
    (SequentialProof<P,Rule> where P: Proposition, Rule: InferenceRule<P>), ProofInProofPath,
    "subproof", "subproofs", "Subproofs"
}
