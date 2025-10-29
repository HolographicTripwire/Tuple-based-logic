use std::collections::HashSet;

use tbl_structures::{path_composites::OwnedExpressionInProof};

use crate::{assertions::expression::stringify_length, errors::{specification_error::{NaryPredicate, NaryStringifier, StringifiablePredicate}, ProofStepSpecificationError}};

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their lengths aren't equal
pub fn expression_length_inequality_predicate<'a,const n: usize>() -> impl NaryPredicate<'a,n,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; n]| { 
        let mut values = HashSet::new();
        for value in os.iter().map(|o| o.obj().as_slice() )
            { if !values.insert(value) { return false; } }
        true
    }
}
/// Get a [Stringifier](NaryStringifier) which takes an [Expressions](OwnedExpressionInProof) and returns an error message saying that their lengths aren't inequal
pub fn expression_length_inequality_stringifier<'a, const n: usize>() -> impl NaryStringifier<'a,n,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; n]| format!(
        "Expression lengths expected to be inequal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.path().to_string()
            + " -> " +
            &stringify_length(o.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expressions](OwnedExpressionInProof) and returns an error message if their lengths aren't inequal
pub fn expression_length_inequality_check<'a, const n: usize>() -> StringifiablePredicate<'a,n,OwnedExpressionInProof> { StringifiablePredicate::new(
    expression_length_inequality_predicate(),
    expression_length_inequality_stringifier(),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have inequal length, returning an error otherwise
pub fn assert_expression_length_inequality<'a,const n: usize>(exprs: [OwnedExpressionInProof; n]) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_length_inequality_check().evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
