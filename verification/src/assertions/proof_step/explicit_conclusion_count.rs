use path_lib::obj_at_path::OwnedObjAtPath;
use tbl_structures::{inference::InferenceRule, proof::{OwnedSubproofInProof, Proof, ProofStep, SubproofPath}, DisplayExt};

use crate::errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate};


/// Get a [Predicate](NaryPredicate) which takes a [Subproof](OwnedSubproofInProof) and checks if it has as expected_count assumptions
fn explicit_conclusion_count_predicate<'a,Rule:InferenceRule>(expected_count: usize) -> impl NaryPredicate<1,OwnedSubproofInProof<Rule>> {
    move |o: [OwnedObjAtPath<Proof<Rule>, SubproofPath>; 1]| 
    o[0].obj().explicit_conclusion_paths().into_iter().count() == expected_count
}
/// Get a [Stringifier](NaryPredicate) which takes a [Subproof](OwnedSubproofInProof) and returns an error message saying that this subproof does not have expected_count assumptions
fn explicit_conclusion_count_stringifier<'a,Rule:InferenceRule>(expected_count: usize) -> impl NaryStringifier<1,OwnedSubproofInProof<Rule>> {
    move |o: [OwnedObjAtPath<Proof<Rule>, SubproofPath>; 1]| format!(
        "Proof at step {step} has wrong number of explicit conclusions (expected {num_expected}; found {num_actual}",
        step=o[0].path().display(), num_expected=expected_count,
        num_actual=o[0].obj().explicit_conclusion_paths().into_iter().count()
    )
}
/// Get a [Checker](StringifiablePredicate) which takes a [Subproof](OwnedSubproofInProof) and returns an error message if this subproof does not have expected_count assumptions
fn explicit_conclusion_count_check<'a,Rule:InferenceRule>(expected_count: usize) -> StringifiablePredicate<1,OwnedSubproofInProof<Rule>> { StringifiablePredicate::new(
    explicit_conclusion_count_predicate(expected_count),
    explicit_conclusion_count_stringifier(expected_count),
)}
/// Check that the provided [Subproof](OwnedSubproofInProof) has expected_count assumptions, returning an error otherwise
pub fn assert_explicit_conclusion_count<'a,Rule:InferenceRule>(step: OwnedSubproofInProof<Rule>, expected_count: usize) -> Result<(), ProofStepSpecificationError> {
    explicit_conclusion_count_check(expected_count).evaluate([step])
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
