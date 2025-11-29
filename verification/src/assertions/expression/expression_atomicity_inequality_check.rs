use tbl_structures::path_composites::OwnedExpressionInProof;

use crate::errors::specification_error::{Assessor, AssessedErrorStringifier, ProofStepSpecificationError, ErrorStringifiableAssessor};

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their atomicities are equal
pub fn expression_atomicity_inequality_predicate<'a>() -> impl Assessor<'a,[OwnedExpressionInProof;2],(),bool> {
    move |os: [OwnedExpressionInProof; 2]| { 
        let first_atomicity = os[0].0.obj().as_atom().is_ok();
        let second_atomicity = os[1].0.obj().as_atom().is_ok();
        if first_atomicity != second_atomicity { Ok(()) }
        else { Err(first_atomicity) }
    }
}
/// Get an [AssessedErrorStringifier] which takes n [Expressions](OwnedExpressionInProof) and returns an error message saying that their atomicities aren't equal
pub fn expression_atomicity_inequality_stringifier<'a>() -> impl AssessedErrorStringifier<'a,[OwnedExpressionInProof;2],bool> {
    move |os: [OwnedExpressionInProof; 2],value| format!(
        "Atomicity of expressions {expr_1} and {expr_2} expected to be inequal, but both were {value}",
        expr_1 = os[0].0.path(),
        expr_2 = os[1].0.path()
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expression](OwnedExpressionInProof) and returns an error message if their atomicities aren't equal
pub fn expression_atomicity_inequality_check<'a>() -> ErrorStringifiableAssessor<'a,[OwnedExpressionInProof;2],(),()> { ErrorStringifiableAssessor::new(
    expression_atomicity_inequality_predicate(),
    expression_atomicity_inequality_stringifier(),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have inequal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_inequality<'a>(exprs: [OwnedExpressionInProof; 2]) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_atomicity_inequality_check::<'a>()
        .evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
