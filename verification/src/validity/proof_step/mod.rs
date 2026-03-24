mod assumption_count;
mod explicit_conclusion_count;

pub use assumption_count::*;
pub use explicit_conclusion_count::*;
use path_lib::HasChildren;


pub fn assumptions_as_slice<'a, Rule: InferenceRule>(inference: &'a Inference<Rule>) -> Vec<PropositionInProofStep<'a>> {
    <Inference<Rule> as HasChildren<PropositionInProofStepPath,TblProposition>>::get_located_children(inference)
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<PropositionInProofStep>>()
}

pub fn assumptions_as_sized_slice<'a, const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &'a Inference<Rule>) -> Result<Box<[PropositionInProofStep<'a>; EXPECTED_SIZE]>,AssumptionCountCheckError<Rule>> {
    match assumptions_as_slice(&inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => { Err(AssumptionCountCheckError{
                expected_count: EXPECTED_SIZE,
                inference: inference.clone()
            }) },
        }
}

pub fn explicit_conclusions_as_slice<'a, Rule: InferenceRule>(inference: &'a Inference<Rule>) -> Vec<PropositionInProofStep<'a>> {
    inference.get_located_explicit_conclusions()
        .into_iter()
        .map(|obj| PropositionInProofStep::from(obj.replace_path(|p| p.into())))
        .collect::<Vec<PropositionInProofStep>>()
}

pub fn explicit_conclusions_as_sized_slice<'a, const EXPECTED_SIZE: usize,Rule: InferenceRule>(inference: &'a Inference<Rule>) -> Result<Box<[PropositionInProofStep<'a>; EXPECTED_SIZE]>,ExplicitConclusionCountCheckError<Rule>> {
    match explicit_conclusions_as_slice(&inference)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => { Err(ExplicitConclusionCountCheckError{
                expected_count: EXPECTED_SIZE, 
                inference: inference.clone()
            }) },
        }
}
