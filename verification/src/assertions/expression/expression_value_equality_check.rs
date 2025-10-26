use tbl_structures::{path_composites::OwnedExpressionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

use crate::errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate};

/// Get a [Predicate](NaryPredicate) which takes an [Expression](OwnedExpressionInProof) and checks if its atomicity is the expected value
fn expression_value_equality_predicate<'a,const n: usize>() -> impl NaryPredicate<'a,n,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; n]| { 
        let mut iter = os.iter().map(|o| o.obj() );
        let first_value = iter.next().expect("Cannot check value equality for zero expressions");
        for nth_value in iter {
            if nth_value != first_value { return false }
        }
        true
    }
}

/// Get a [Stringifier](NaryStringifier) which takes an [Expression](OwnedExpressionInProof) and returns an error message saying that this expression's atomicity is not the expected value
fn expression_value_equality_stringifier<'a,const n:usize>(style: ExpressionStyle<'a>) -> impl NaryStringifier<'a,n,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; n]| format!(
        "Expression values expected to be equal, but weren't; {values}",
        values = os.map(|o| 
            o.path().to_string()
            + " -> " +
            &style.stringify(o.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes an [Expression](OwnedExpressionInProof) and returns an error message if this expression's atomicity is not the expected value
pub fn expression_value_equality_check<'a,const n: usize>(style: ExpressionStyle<'a>) -> StringifiablePredicate<'a,n,OwnedExpressionInProof> { StringifiablePredicate::new(
    expression_value_equality_predicate(),
    expression_value_equality_stringifier(style),
)}

/// Check that the provided [Expression](OwnedExpressionInProof) has an atomicity equal to atomicty_expected, returning an error otherwise
pub fn assert_expression_value_equality<'a,const n: usize>(expr: OwnedExpressionInProof, style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_value_equality_check(style).evaluate([expr])
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
