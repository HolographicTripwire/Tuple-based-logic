use tbl_structures::{inference::InferenceRule, proof::OwnedSubexpressionInProof};

use crate::errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate};

/// Get a [Predicate](NaryPredicate) which takes an [Expression](OwnedSubexpressionInProof) and checks if its atomicity is the expected value
fn expression_atomicity_predicate<'a>(atomicity_expected: bool) -> impl NaryPredicate<1,OwnedSubexpressionInProof> {
    move |o: [OwnedSubexpressionInProof; 1]| 
    o[0].obj().as_atom().is_ok() == atomicity_expected
}
/// Convert atomicity to string
fn stringify_atomicity(is_atomic: bool) -> &'static str {
    if is_atomic { "atomic" } else { "not-atomic" }
}
/// Get a [Stringifier](NaryStringifier) which takes an [Expression](OwnedSubexpressionInProof) and returns an error message saying that this expression's atomicity is not the expected value
fn expression_atomicity_stringifier<'a>(atomicity_expected: bool) -> impl NaryStringifier<1,OwnedSubexpressionInProof> {
    move |o: [OwnedSubexpressionInProof; 1]| format!(
        "Expression at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
        path=o[0].path().to_string(),
        atomicity_expected=stringify_atomicity(atomicity_expected),
        atomicity_actual=stringify_atomicity(o[0].obj().as_atom().is_ok())
    )
}
/// Get a [Checker](StringifiablePredicate) which takes an [Expression](OwnedSubexpressionInProof) and returns an error message if this expression's atomicity is not the expected value
pub fn expression_atomicity_check<'a>(atomicity_expected: bool) -> StringifiablePredicate<1,OwnedSubexpressionInProof> { StringifiablePredicate::new(
    expression_atomicity_predicate(atomicity_expected),
    expression_atomicity_stringifier(atomicity_expected),
)}

/// Check that the provided [Expression](OwnedSubexpressionInProof) has an atomicity equal to atomicty_expected, returning an error otherwise
pub fn assert_expression_atomicity<'a,Rule:InferenceRule>(expr: OwnedSubexpressionInProof, atomicity_expected: bool) -> Result<(), ProofStepSpecificationError> {
    expression_atomicity_check(atomicity_expected).evaluate([expr])
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
