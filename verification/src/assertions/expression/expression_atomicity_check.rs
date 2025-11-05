use tbl_structures::path_composites::OwnedExpressionInProof;

use crate::{assertions::expression::stringify_atomicity, errors::specification_error::{Assessor, AssessedStringifier, ProofStepSpecificationError, StringifiablePredicate}};

/// Get a [Predicate](NaryPredicate) which takes an [Expression](OwnedExpressionInProof) and checks if its atomicity is the expected value
pub fn expression_atomicity_predicate<'a>(atomicity_expected: bool) -> impl Assessor<'a,OwnedExpressionInProof,()> {
    move |o: OwnedExpressionInProof| 
    if o.0.obj().as_atom().is_ok() == atomicity_expected { Ok(()) }
    else { Err(()) }
}

/// Get a [Stringifier](NaryStringifier) which takes an [Expression](OwnedExpressionInProof) and returns an error message saying that this expression's atomicity is not the expected value
pub fn expression_atomicity_stringifier<'a>(atomicity_expected: bool) -> impl AssessedStringifier<'a,OwnedExpressionInProof,()> {
    move |o: OwnedExpressionInProof,_| format!(
        "Expression at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
        path=o.0.path().to_string(),
        atomicity_expected=stringify_atomicity(atomicity_expected),
        atomicity_actual=stringify_atomicity(o.0.obj().as_atom().is_ok())
    )
}
/// Get a [Checker](StringifiablePredicate) which takes an [Expression](OwnedExpressionInProof) and returns an error message if this expression's atomicity is not the expected value
pub fn expression_atomicity_check<'a>(atomicity_expected: bool) -> StringifiablePredicate<'a,OwnedExpressionInProof,()> { StringifiablePredicate::new(
    expression_atomicity_predicate(atomicity_expected),
    expression_atomicity_stringifier(atomicity_expected),
)}

/// Check that the provided [Expression](OwnedExpressionInProof) has an atomicity equal to atomicty_expected, returning an error otherwise
pub fn assert_expression_atomicity<'a>(expr: OwnedExpressionInProof, atomicity_expected: bool) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_atomicity_check::<'a>(atomicity_expected)
        .evaluate(expr)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
