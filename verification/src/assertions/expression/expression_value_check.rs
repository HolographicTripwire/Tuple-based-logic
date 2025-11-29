use tbl_structures::{expressions::Expression, path_composites::OwnedExpressionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

use crate::errors::specification_error::{Assessor, AssessedErrorStringifier, ProofStepSpecificationError, ErrorStringifiableAssessor};

/// Get a [Predicate](NaryPredicate) which takes an [Expression](OwnedExpressionInProof) and checks if its value is the expected value
pub fn expression_value_predicate<'a>(value_expected: Expression) -> impl Assessor<'a,OwnedExpressionInProof,Expression,()> {
    move |o: OwnedExpressionInProof| {
        let value_actual = o.0.obj();
        if value_actual == &value_expected { Ok(value_actual.clone()) } else { Err(()) }
    }
}

/// Get an [AssessedErrorStringifier] which takes an [Expression](OwnedExpressionInProof) and returns an error message saying that this expression's value is not the expected value
pub fn expression_value_stringifier<'a>(value_expected: Expression, style: ExpressionStyle<'a>) -> impl AssessedErrorStringifier<'a,OwnedExpressionInProof,Expression> {
    move |o: OwnedExpressionInProof,value_actual| {
    format!(
        "Expression at {path} has wrong value (expected {value_expected_styled}; found {value_actual_styled})",
        path=o.0.path().to_string(),
        value_expected_styled=style.stringify(&value_expected),
        value_actual_styled=style.stringify(&value_actual)
    )}
}
/// Get a [Checker](StringifiablePredicate) which takes an [Expression](OwnedExpressionInProof) and returns an error message if this expression's value is not the expected value
pub fn expression_value_check<'a>(value_expected: Expression, style: ExpressionStyle<'a>) -> ErrorStringifiableAssessor<'a,OwnedExpressionInProof,(),Expression> { ErrorStringifiableAssessor::new(
    expression_value_predicate(value_expected.clone()),
    expression_value_stringifier(value_expected, style),
)}

/// Check that the provided [Expression](OwnedExpressionInProof) has a value equal to value_expected, returning an error otherwise
pub fn assert_expression_value<'a>(expr: OwnedExpressionInProof, value_expected: Expression, style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_value_check(value_expected, style).evaluate(expr)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
