use std::collections::HashSet;

use tbl_structures::{path_composites::OwnedExpressionInProof};

use crate::{assertions::expression::stringify_length, errors::{specification_error::{Assessor, AssessedStringifier, StringifiablePredicate}, ProofStepSpecificationError}};

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their lengths aren't equal
pub fn expression_length_inequality_predicate<'a,const N: usize>() -> impl Assessor<'a,[OwnedExpressionInProof;N],()> {
    move |os: [OwnedExpressionInProof; N]| { 
        let mut values = HashSet::new();
        for value in os.iter().map(|o| o.0.obj().as_slice() )
            { if !values.insert(value) { return Ok(()); } }
        Err(())
    }
}
/// Get a [Stringifier](NaryStringifier) which takes an [Expressions](OwnedExpressionInProof) and returns an error message saying that their lengths aren't inequal
pub fn expression_length_inequality_stringifier<'a, const N: usize>() -> impl AssessedStringifier<'a,[OwnedExpressionInProof;N],()> {
    move |os: [OwnedExpressionInProof; N],_| format!(
        "Expression lengths expected to be inequal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expressions](OwnedExpressionInProof) and returns an error message if their lengths aren't inequal
pub fn expression_length_inequality_check<'a, const N: usize>() -> StringifiablePredicate<'a,[OwnedExpressionInProof;N],()> { StringifiablePredicate::new(
    expression_length_inequality_predicate(),
    expression_length_inequality_stringifier(),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have inequal length, returning an error otherwise
pub fn assert_expression_length_inequality<'a,const N: usize>(exprs: [OwnedExpressionInProof; N]) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_length_inequality_check().evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
