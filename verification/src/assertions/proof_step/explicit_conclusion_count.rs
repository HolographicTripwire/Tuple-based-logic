use tbl_structures::{inference::InferenceRule, proof::{OwnedInferenceInProof, ProofStep}};

use crate::errors::specification_error::{Assessor, AssessedStringifier, ProofStepSpecificationError, StringifiablePredicate};


/// Get a [Predicate](NaryPredicate) which takes an [Inference](OwnedInferenceInProof) and checks if it has as expected_count assumptions
fn explicit_conclusion_count_predicate<'a,Rule:InferenceRule>(expected_count: usize) -> impl Assessor<'a,OwnedInferenceInProof<Rule>,()> {
    move |o: OwnedInferenceInProof<Rule>| 
    if o.0.obj().explicit_conclusion_paths().into_iter().count() == expected_count { Ok(()) } else { Err(()) }
}
/// Get a [Stringifier](NaryPredicate) which takes an [Inference](OwnedInferenceInProof) and returns an error message saying that this subproof does not have expected_count explicit conclusions
pub fn explicit_conclusion_count_stringifier<'a,Rule:InferenceRule>(expected_count: usize) -> impl AssessedStringifier<'a,OwnedInferenceInProof<Rule>,()> {
    move |o: OwnedInferenceInProof<Rule>,_| format!(
        "Proof at step {step} has wrong number of explicit conclusions (expected {num_expected}; found {num_actual}",
        step=o.0.path().to_string(), num_expected=expected_count,
        num_actual=o.0.obj().explicit_conclusion_paths().into_iter().count()
    )
}
/// Get a [Checker](StringifiablePredicate) which takes a [Inference](OwnedInferenceInProof) and returns an error message if this subproof does not have expected_count explicit conclusions
pub fn explicit_conclusion_count_check<'a,Rule:InferenceRule>(expected_count: usize) -> StringifiablePredicate<'a,OwnedInferenceInProof<Rule>,()> { StringifiablePredicate::new(
    explicit_conclusion_count_predicate(expected_count),
    explicit_conclusion_count_stringifier(expected_count),
)}
/// Check that the provided [Inference](OwnedInferenceInProof) has expected_count explicit conclusions, returning an error otherwise
pub fn assert_explicit_conclusion_count<'a,Rule:InferenceRule>(step: OwnedInferenceInProof<Rule>, expected_count: usize) -> Result<(), ProofStepSpecificationError<'a>> {
    explicit_conclusion_count_check(expected_count).evaluate(step)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
