use tbl_structures::{expressions::Expression, path_composites::OwnedExpressionInProof};

use crate::errors::{specification_error::{NaryPredicate, NaryStringifier, StringifiablePredicate}, ProofStepSpecificationError};

/// Get a [Predicate](NaryPredicate) which takes an [Expression](OwnedSubexpressionInProof) and checks if its length is not the expected value
fn expression_length_predicate<'a>(expected_length: usize) -> impl NaryPredicate<'a,1,OwnedExpressionInProof> {
    move |o: [OwnedExpressionInProof; 1]| { 
        match o[0].obj().as_slice() {
            Ok(tuple) => tuple.len() == expected_length,
            Err(()) => false
        }
    }
}
/// Convert length of an expression to string
fn stringify_length(expr: &Expression) -> String {
    match expr.as_slice() {
        Ok(tuple) => tuple.len().to_string(),
        Err(()) => "atomic".to_string()
    }
}
/// Get a [Stringifier](NaryStringifier) which takes an [Expression](OwnedSubexpressionInProof) and returns an error message saying that this expression's length is not the expected value
pub fn expression_length_stringifier<'a>(length_expected: usize) -> impl NaryStringifier<'a,1,OwnedExpressionInProof> {
    move |o: [OwnedExpressionInProof; 1]| format!(
        "Expression at {path} has wrong length (expected {length_expected}; found {length_actual})",
        path=o[0].path().to_string(),
        length_actual=stringify_length(o[0].obj())
    )
}
/// Get a [Checker](StringifiablePredicate) which takes an [Expression](OwnedSubexpressionInProof) and returns an error message if this expression's length is not the expected value
pub fn expression_length_check<'a>(length_expected: usize) -> StringifiablePredicate<'a,1,OwnedExpressionInProof> { StringifiablePredicate::new(
    expression_length_predicate(length_expected),
    expression_length_stringifier(length_expected),
)}

/// Check that the provided [Subproof](OwnedSubproofInProof) has expected_count assumptions, returning an error otherwise
pub fn assert_expression_length<'a>(expr: OwnedExpressionInProof, length_expected: usize) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_length_check(length_expected).evaluate([expr])
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
