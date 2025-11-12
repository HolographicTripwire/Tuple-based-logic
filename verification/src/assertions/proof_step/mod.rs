mod assumption_count;
mod explicit_conclusion_count;

pub use explicit_conclusion_count::*;
use path_lib::obj_at_path::ObjAtPathWithChildren;
use tbl_structures::{inference::InferenceRule, path_composites::OwnedPropositionInProof, proof::InferenceInProof};

use crate::{assertions::proof_step::assumption_count::AssumptionCountCheckError};

pub fn assumptions_as_slice<Rule: InferenceRule>(inference: &InferenceInProof<Rule>) -> Vec<OwnedPropositionInProof> {
    inference.0.get_located_children_owned()
        .into_iter()
        .map(|obj| OwnedPropositionInProof(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedPropositionInProof>>()
}

pub fn assumptions_as_sized_slice<'a,const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &InferenceInProof<Rule>) -> Result<Box<[OwnedPropositionInProof; EXPECTED_SIZE]>,AssumptionCountCheckError<Rule>> {
    match assumptions_as_slice(inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => { Err(AssumptionCountCheckError::new(EXPECTED_SIZE, inference.into_owned())) },
        }
}

pub fn explicit_conclusions_as_slice<'a,Rule: InferenceRule>(inference: &InferenceInProof<Rule>) -> Vec<OwnedPropositionInProof> {
    inference.0.get_located_children_owned()
        .into_iter()
        .map(|obj| OwnedPropositionInProof(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedPropositionInProof>>()
}

pub fn explicit_conclusions_as_sized_slice<'a,const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &InferenceInProof<Rule>) -> Result<Box<[OwnedPropositionInProof; EXPECTED_SIZE]>,AssumptionCountCheckError<Rule>> {
    match explicit_conclusions_as_slice(inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => { Err(ExplicitConclusionCountCheckError::new(EXPECTED_SIZE, inference.into_owned())) },
        }
}
