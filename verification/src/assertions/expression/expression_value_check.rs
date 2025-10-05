use tbl_structures::{expressions::Expression, proof::OwnedSubexpressionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

use crate::errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate};

/// Get a [Predicate](NaryPredicate) which takes an [Expression](OwnedSubexpressionInProof) and checks if its atomicity is the expected value
fn expression_value_predicate<'a>(value_expected: Expression) -> impl NaryPredicate<'a,1,OwnedSubexpressionInProof> {
    move |o: [OwnedSubexpressionInProof; 1]| 
    o[0].obj() == &value_expected
}

/// Get a [Stringifier](NaryStringifier) which takes an [Expression](OwnedSubexpressionInProof) and returns an error message saying that this expression's atomicity is not the expected value
fn expression_value_stringifier<'a>(value_expected: Expression, style: ExpressionStyle<'a>) -> impl NaryStringifier<'a,1,OwnedSubexpressionInProof> {
    move |o: [OwnedSubexpressionInProof; 1]| {
    format!(
        "Expression at {path} has wrong value (expected {value_expected_styled}; found {value_actual_styled})",
        path=o[0].path().to_string(),
        value_expected_styled=style.stringify(&value_expected),
        value_actual_styled=style.stringify(o[0].obj())
    )}
}
/// Get a [Checker](StringifiablePredicate) which takes an [Expression](OwnedSubexpressionInProof) and returns an error message if this expression's atomicity is not the expected value
pub fn expression_value_check<'a>(value_expected: Expression, style: ExpressionStyle<'a>) -> StringifiablePredicate<'a,1,OwnedSubexpressionInProof> { StringifiablePredicate::new(
    expression_value_predicate(value_expected.clone()),
    expression_value_stringifier(value_expected, style),
)}

/// Check that the provided [Expression](OwnedSubexpressionInProof) has an atomicity equal to atomicty_expected, returning an error otherwise
pub fn assert_expression_value<'a>(expr: OwnedSubexpressionInProof, value_expected: Expression, style: ExpressionStyle<'static>) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_value_check(value_expected, style).evaluate([expr])
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
