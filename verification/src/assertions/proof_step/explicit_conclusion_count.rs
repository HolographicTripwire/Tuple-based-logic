use tbl_structures::{inference::InferenceRule, proof::{OwnedInferenceInProof, ProofStep}};

use crate::errors::specification_error::{Assessor, AssessedErrorStringifier, ProofStepSpecificationError, ErrorStringifiableAssessor};


/// Get a [Predicate](NaryPredicate) which takes an [Inference](OwnedInferenceInProof) and checks if it has as expected_count assumptions
fn explicit_conclusion_count_predicate<'a,Rule:InferenceRule>(expected_count: usize) -> impl Assessor<'a,OwnedInferenceInProof<Rule>,(),usize> {
    move |o: OwnedInferenceInProof<Rule>| {
        let actual_count = o.0.obj().explicit_conclusion_paths().into_iter().count();
        if actual_count == expected_count { Ok(()) } else { Err(actual_count) }
    }
}
/// Get a [Stringifier](NaryPredicate) which takes an [Inference](OwnedInferenceInProof) and returns an error message saying that this subproof does not have expected_count explicit conclusions
pub fn explicit_conclusion_count_stringifier<'a,Rule:InferenceRule>(expected_count: usize) -> impl AssessedErrorStringifier<'a,OwnedInferenceInProof<Rule>,usize> {
    move |o: OwnedInferenceInProof<Rule>,num_actual| format!(
        "Proof at step {step} has wrong number of explicit conclusions (expected {num_expected}; found {num_actual}",
        step=o.0.path().to_string(), num_expected=expected_count,
    )
}
/// Get a [Checker](StringifiablePredicate) which takes a [Inference](OwnedInferenceInProof) and returns an error message if this subproof does not have expected_count explicit conclusions
pub fn explicit_conclusion_count_check<'a,Rule:InferenceRule>(expected_count: usize) -> ErrorStringifiableAssessor<'a,OwnedInferenceInProof<Rule>,usize,()> { ErrorStringifiableAssessor::new(
    explicit_conclusion_count_predicate(expected_count),
    explicit_conclusion_count_stringifier(expected_count),
)}
/// Check that the provided [Inference](OwnedInferenceInProof) has expected_count explicit conclusions, returning an error otherwise
pub fn assert_explicit_conclusion_count<'a,Rule:InferenceRule>(step: OwnedInferenceInProof<Rule>, expected_count: usize) -> Result<(), ProofStepSpecificationError<'a>> {
    explicit_conclusion_count_check(expected_count).evaluate(step)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
