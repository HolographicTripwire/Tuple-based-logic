use tbl_structures::path_composites::OwnedPropositionInProof;

use crate::{assertions::expression::stringify_length, errors::{specification_error::{NaryPredicate, NaryStringifier, StringifiablePredicate}, ProofStepSpecificationError}};

/// Get a [Predicate](NaryPredicate) which takes an [Proposition](OwnedPropositionInProof) and checks if its length is not the expected value
fn proposition_length_predicate<'a>(expected_length: usize) -> impl NaryPredicate<'a,1,OwnedPropositionInProof> {
    move |o: [OwnedPropositionInProof; 1]| { 
        match o[0].obj().as_slice() {
            Ok(tuple) => tuple.len() == expected_length,
            Err(()) => false
        }
    }
}
/// Get a [Stringifier](NaryStringifier) which takes an [Proposition](OwnedPropositionInProof) and returns an error message saying that this proposition's length is not the expected value
pub fn proposition_length_stringifier<'a>(length_expected: usize) -> impl NaryStringifier<'a,1,OwnedPropositionInProof> {
    move |o: [OwnedPropositionInProof; 1]| format!(
        "Proposition at {path} has wrong length (expected {length_expected}; found {length_actual})",
        path=o[0].path().to_string(),
        length_actual=stringify_length(o[0].obj())
    )
}
/// Get a [Checker](StringifiablePredicate) which takes an [Proposition](OwnedPropositionInProof) and returns an error message if this proposition's length is not the expected value
pub fn proposition_length_check<'a>(length_expected: usize) -> StringifiablePredicate<'a,1,OwnedPropositionInProof> { StringifiablePredicate::new(
    proposition_length_predicate(length_expected),
    proposition_length_stringifier(length_expected),
)}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an length equal to length_expected, returning an error otherwise
pub fn assert_proposition_length<'a>(expr: OwnedPropositionInProof, length_expected: usize) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_length_check(length_expected).evaluate([expr])
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
