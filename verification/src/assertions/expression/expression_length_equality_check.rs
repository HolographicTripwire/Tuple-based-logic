use tbl_structures::{path_composites::OwnedExpressionInProof};

use crate::{assertions::expression::stringify_length, errors::{specification_error::{Assessor, AssessedStringifier, StringifiablePredicate}, ProofStepSpecificationError}};

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their lengths are equal
pub fn expression_length_equality_predicate<'a,const N: usize>() -> impl Assessor<'a,[OwnedExpressionInProof;N],()> {
    move |os: [OwnedExpressionInProof; N]| { 
        let mut iter = os.iter().map(|o| o.0.obj().as_slice() );
        let first_length = iter.next().expect("Cannot check length equality for zero expressions");
        for nth_length in iter {
            if nth_length != first_length { return Err(()) }
        }
        Ok(())
    }
}
/// Get a [Stringifier](NaryStringifier) which takes an [Expressions](OwnedExpressionInProof) and returns an error message saying that their lengths aren't equal
pub fn expression_length_equality_stringifier<'a, const N: usize>() -> impl AssessedStringifier<'a,[OwnedExpressionInProof;N],()> {
    move |os: [OwnedExpressionInProof; N],_| format!(
        "Expression lengths expected to be equal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expressions](OwnedExpressionInProof) and returns an error message if their lengths aren't equal
pub fn expression_length_equality_check<'a, const N: usize>() -> StringifiablePredicate<'a,[OwnedExpressionInProof;N],()> { StringifiablePredicate::new(
    expression_length_equality_predicate(),
    expression_length_equality_stringifier(),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have equal length, returning an error otherwise
pub fn assert_expression_length_equality<'a,const N: usize>(exprs: [OwnedExpressionInProof; N]) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_length_equality_check().evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
