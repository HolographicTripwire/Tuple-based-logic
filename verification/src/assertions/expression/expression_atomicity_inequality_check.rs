use tbl_structures::path_composites::OwnedExpressionInProof;

use crate::{assertions::expression::stringify_atomicity, errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate}};

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their atomicities are equal
pub fn expression_atomicity_inequality_predicate<'a>() -> impl NaryPredicate<'a,2,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; 2]| { 
        os[0].0.obj().as_atom().is_ok() != os[1].0.obj().as_atom().is_ok()
    }
}
/// Get a [Stringifier](NaryStringifier) which takes n [Expressions](OwnedExpressionInProof) and returns an error message saying that their atomicities aren't equal
pub fn expression_atomicity_inequality_stringifier<'a>() -> impl NaryStringifier<'a,2,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; 2]| format!(
        "Expression atomicities expected to be inequal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            stringify_atomicity(o.0.obj().as_atom().is_ok())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expression](OwnedExpressionInProof) and returns an error message if their atomicities aren't equal
pub fn expression_atomicity_inequality_check<'a>() -> StringifiablePredicate<'a,2,OwnedExpressionInProof> { StringifiablePredicate::new(
    expression_atomicity_inequality_predicate(),
    expression_atomicity_inequality_stringifier(),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have inequal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_inequality<'a>(exprs: [OwnedExpressionInProof; 2]) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_atomicity_inequality_check::<'a>()
        .evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
