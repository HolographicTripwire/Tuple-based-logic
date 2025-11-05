use tbl_structures::{path_composites::OwnedExpressionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

use crate::errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate};

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their values are equal
pub fn expression_value_equality_predicate<'a,const N: usize>() -> impl NaryPredicate<'a,N,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; N]| { 
        let mut iter = os.iter().map(|o| o.0.obj() );
        let first_value = iter.next().expect("Cannot check value equality for zero expressions");
        for nth_value in iter {
            if nth_value != first_value { return false }
        }
        true
    }
}

/// Get a [Stringifier](NaryStringifier) which takes n [Expressions](OwnedExpressionInProof) and returns an error message saying that these expression's value aren't equal
pub fn expression_value_equality_stringifier<'a,const N:usize>(style: ExpressionStyle<'a>) -> impl NaryStringifier<'a,N,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; N]| format!(
        "Expression values expected to be equal, but weren't; {values}",
        values = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expressions](OwnedExpressionInProof) and returns an error message if these expressions values are not equal
pub fn expression_value_equality_check<'a,const N: usize>(style: ExpressionStyle<'a>) -> StringifiablePredicate<'a,N,OwnedExpressionInProof> { StringifiablePredicate::new(
    expression_value_equality_predicate(),
    expression_value_equality_stringifier(style),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have equal values, returning an error otherwise
pub fn assert_expression_value_equality<'a,const N: usize>(exprs: [OwnedExpressionInProof; N], style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_value_equality_check(style).evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
