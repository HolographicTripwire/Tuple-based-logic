use tbl_structures::{expressions::Proposition, path_composites::OwnedPropositionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

use crate::errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate};

/// Get a [Predicate](NaryPredicate) which takes an [Proposition](OwnedPropositionInProof) and checks if its value is the expected value
fn proposition_value_predicate<'a>(value_expected: Proposition) -> impl NaryPredicate<'a,OwnedPropositionInProof> {
    move |o: OwnedPropositionInProof| 
    o.0.obj() == &value_expected
}

/// Get a [Stringifier](NaryStringifier) which takes an [Proposition](OwnedPropositionInProof) and returns an error message saying that this proposition's value is not the expected value
fn proposition_value_stringifier<'a>(value_expected: Proposition, style: ExpressionStyle<'a>) -> impl NaryStringifier<'a,OwnedPropositionInProof> {
    move |o: OwnedPropositionInProof| {
    format!(
        "Proposition at {path} has wrong value (expected {value_expected_styled}; found {value_actual_styled})",
        path=o.0.path().to_string(),
        value_expected_styled=style.stringify(&value_expected),
        value_actual_styled=style.stringify(o.0.obj())
    )}
}
/// Get a [Checker](StringifiablePredicate) which takes an [Proposition](OwnedPropositionInProof) and returns an error message if this proposition's value is not the expected value
pub fn proposition_value_check<'a>(value_expected: Proposition, style: ExpressionStyle<'a>) -> StringifiablePredicate<'a,OwnedPropositionInProof> { StringifiablePredicate::new(
    proposition_value_predicate(value_expected.clone()),
    proposition_value_stringifier(value_expected, style),
)}

/// Check that the provided [Proposition](OwnedPropositionInProof) has a value equal to value_expected, returning an error otherwise
pub fn assert_proposition_value<'a>(expr: OwnedPropositionInProof, value_expected: Proposition, style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_value_check(value_expected, style).evaluate(expr)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
