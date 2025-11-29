mod assumption_count;
mod explicit_conclusion_count;

pub use assumption_count::*;
pub use explicit_conclusion_count::*;
use path_lib::obj_at_path::ObjAtPathWithChildren;
use tbl_structures::{inference::InferenceRule, path_composites::OwnedPropositionInProof, proof::{InferenceInProof, OwnedInferenceInProof}};

use crate::errors::{specification_error::AssessedErrorStringifier, ProofStepSpecificationError};

pub fn assumptions_as_slice<Rule: InferenceRule>(inference: &InferenceInProof<Rule>) -> Vec<OwnedPropositionInProof> {
    inference.0.get_located_children_owned()
        .into_iter()
        .map(|obj| OwnedPropositionInProof(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedPropositionInProof>>()
}

pub fn assumptions_as_sized_slice<'a,const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &InferenceInProof<Rule>) -> Result<Box<[OwnedPropositionInProof; EXPECTED_SIZE]>,ProofStepSpecificationError<'a>> {
    match assumptions_as_slice(inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => {
                let inference = OwnedInferenceInProof(inference.0.clone().into_owned());
                Err(ProofStepSpecificationError::from_inner(assumption_count_stringifier(EXPECTED_SIZE).assign(inference,())))
            },
        }
}

pub fn explicit_conclusions_as_slice<'a,Rule: InferenceRule>(inference: &InferenceInProof<Rule>) -> Vec<OwnedPropositionInProof> {
    inference.0.get_located_children_owned()
        .into_iter()
        .map(|obj| OwnedPropositionInProof(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedPropositionInProof>>()
}

pub fn explicit_conclusions_as_sized_slice<'a,const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &InferenceInProof<Rule>) -> Result<Box<[OwnedPropositionInProof; EXPECTED_SIZE]>,ProofStepSpecificationError<'a>> {
    match explicit_conclusions_as_slice(inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => {
                let inference = OwnedInferenceInProof(inference.0.clone().into_owned());
                Err(ProofStepSpecificationError::from_inner(explicit_conclusion_count_stringifier(EXPECTED_SIZE).assign(inference,())))
            }
        }
}
