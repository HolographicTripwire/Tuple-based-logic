mod assumption_count;
mod explicit_conclusion_count;

pub use assumption_count::*;
pub use explicit_conclusion_count::*;

use path_lib::{HasChildren};
use tbl_structures::{expressions::Proposition, inference::{Inference, InferenceRule}, proof::{OwnedPropositionInInference, ProofStep, PropositionInInferencePath}};


pub fn assumptions_as_slice<Rule: InferenceRule>(inference: &Inference<Rule>) -> Vec<OwnedPropositionInInference> {
    <Inference<Rule> as HasChildren<'_,PropositionInInferencePath,Proposition>>::get_located_children_owned(inference)
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<OwnedPropositionInInference>>()
}

pub fn assumptions_as_sized_slice<const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<Box<[OwnedPropositionInInference; EXPECTED_SIZE]>,AssumptionCountCheckError<Rule>> {
    match assumptions_as_slice(&inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => { Err(AssumptionCountCheckError{
                expected_count: EXPECTED_SIZE,
                inference: inference.clone()
            }) },
        }
}

pub fn explicit_conclusions_as_slice<Rule: InferenceRule>(inference: &Inference<Rule>) -> Vec<OwnedPropositionInInference> {
    inference.get_located_explicit_conclusions_owned()
        .into_iter()
        .map(|obj| OwnedPropositionInInference(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedPropositionInInference>>()
}

pub fn explicit_conclusions_as_sized_slice<const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<Box<[OwnedPropositionInInference; EXPECTED_SIZE]>,ExplicitConclusionCountCheckError<Rule>> {
    match explicit_conclusions_as_slice(&inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => { Err(ExplicitConclusionCountCheckError{
                expected_count: EXPECTED_SIZE, 
                inference: inference.clone()
            }) },
        }
}
