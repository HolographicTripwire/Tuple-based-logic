mod assumption_count;
mod explicit_conclusion_count;

pub use assumption_count::*;
pub use explicit_conclusion_count::*;
use path_lib::obj_at_path::ObjAtPathWithChildren;
use tbl_structures::{inference::InferenceRule, path_composites::OwnedPropositionInProof, proof::OwnedInferenceInProof};

use crate::errors::{specification_error::NaryStringifier, ProofStepSpecificationError};

pub fn assumptions_as_slice<Rule: InferenceRule>(inference: &OwnedInferenceInProof<Rule>) -> Vec<OwnedPropositionInProof> {
    inference.0.get_located_children_owned()
        .into_iter()
        .map(|obj| OwnedPropositionInProof(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedPropositionInProof>>()
}

pub fn assumptions_as_sized_slice<'a,const expected_size: usize,Rule: InferenceRule>(inference: &OwnedInferenceInProof<Rule>) -> Result<Box<[OwnedPropositionInProof; expected_size]>,ProofStepSpecificationError<'a>> {
    match assumptions_as_slice(inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => Err(ProofStepSpecificationError::from_inner(assumption_count_stringifier(expected_size).assign([inference.to_owned()]))),
        }
}

pub fn explicit_conclusions_as_slice<'a,Rule: InferenceRule>(inference: &OwnedInferenceInProof<Rule>) -> Vec<OwnedPropositionInProof> {
    inference.0.get_located_children_owned()
        .into_iter()
        .map(|obj| OwnedPropositionInProof(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedPropositionInProof>>()
}

pub fn explicit_conclusions_as_sized_slice<'a,const expected_size: usize,Rule: InferenceRule>(inference: &OwnedInferenceInProof<Rule>) -> Result<Box<[OwnedPropositionInProof; expected_size]>,ProofStepSpecificationError<'a>> {
    match explicit_conclusions_as_slice(inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => Err(ProofStepSpecificationError::from_inner(explicit_conclusion_count_stringifier(expected_size).assign([inference.to_owned()]))),
        }
}
