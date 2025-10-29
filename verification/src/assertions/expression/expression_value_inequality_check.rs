use std::collections::HashSet;

use tbl_structures::{path_composites::OwnedExpressionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

use crate::errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate};

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their values are inequal
pub fn expression_value_inequality_predicate<'a,const n: usize>() -> impl NaryPredicate<'a,n,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; n]| { 
        let mut values = HashSet::new();
        for value in os.iter().map(|o| o.obj() )
            { if !values.insert(value) { return false; } }
        true
    }
}

/// Get a [Stringifier](NaryStringifier) which takes n [Expressions](OwnedExpressionInProof) and returns an error message saying that these expression's value aren't inequal
pub fn expression_value_inequality_stringifier<'a,const n:usize>(style: ExpressionStyle<'a>) -> impl NaryStringifier<'a,n,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; n]| format!(
        "Expression values expected to be inequal, but weren't; {values}",
        values = os.map(|o| 
            o.path().to_string()
            + " -> " +
            &style.stringify(o.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expressions](OwnedExpressionInProof) and returns an error message if these expressions values aren't inequal
pub fn expression_value_inequality_check<'a,const n: usize>(style: ExpressionStyle<'a>) -> StringifiablePredicate<'a,n,OwnedExpressionInProof> { StringifiablePredicate::new(
    expression_value_inequality_predicate(),
    expression_value_inequality_stringifier(style),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have inequal values, returning an error otherwise
pub fn assert_expression_value_inequality<'a,const n: usize>(exprs: [OwnedExpressionInProof; n], style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_value_inequality_check(style).evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
