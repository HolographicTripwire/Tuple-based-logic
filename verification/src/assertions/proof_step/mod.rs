mod assumption_count;
mod explicit_conclusion_count;

pub use assumption_count::*;
pub use explicit_conclusion_count::*;

use path_lib::{HasChildren};
use tbl_structures::{expressions::Proposition, inference::{Inference, InferenceRule}, proof::{ProofStep, PropositionInInference, PropositionInInferencePath}};


pub fn assumptions_as_slice<'a, Rule: InferenceRule>(inference: &'a Inference<Rule>) -> Vec<PropositionInInference<'a>> {
    <Inference<Rule> as HasChildren<'_,PropositionInInferencePath,Proposition>>::get_located_children(inference)
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<PropositionInInference>>()
}

pub fn assumptions_as_sized_slice<const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<Box<[PropositionInInference; EXPECTED_SIZE]>,AssumptionCountCheckError<Rule>> {
    match assumptions_as_slice(&inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => { Err(AssumptionCountCheckError{
                expected_count: EXPECTED_SIZE,
                inference: inference.clone()
            }) },
        }
}

pub fn explicit_conclusions_as_slice<'a, Rule: InferenceRule>(inference: &'a Inference<Rule>) -> Vec<PropositionInInference<'a>> {
    inference.get_located_explicit_conclusions()
        .into_iter()
        .map(|obj| PropositionInInference(obj.replace_path(|p| p.into())))
        .collect::<Vec<PropositionInInference>>()
}

pub fn explicit_conclusions_as_sized_slice<const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<Box<[PropositionInInference; EXPECTED_SIZE]>,ExplicitConclusionCountCheckError<Rule>> {
    match explicit_conclusions_as_slice(&inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => { Err(ExplicitConclusionCountCheckError{
                expected_count: EXPECTED_SIZE, 
                inference: inference.clone()
            }) },
        }
}
