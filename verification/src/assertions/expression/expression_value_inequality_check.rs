use std::collections::HashSet;

use tbl_structures::{path_composites::OwnedExpressionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

use crate::errors::specification_error::{Assessor, AssessedErrorStringifier, ProofStepSpecificationError, ErrorStringifiableAssessor};

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their values are inequal
pub fn expression_value_inequality_predicate<'a,const N: usize>() -> impl Assessor<'a,[OwnedExpressionInProof;N],(),()> {
    move |os: [OwnedExpressionInProof; N]| { 
        let mut values = HashSet::new();
        for value in os.iter().map(|o| o.0.obj() )
            { if !values.insert(value) { return Err(()); } }
        Ok(())
    }
}

/// Get an [AssessedErrorStringifier] which takes n [Expressions](OwnedExpressionInProof) and returns an error message saying that these expression's value aren't inequal
pub fn expression_value_inequality_stringifier<'a,const N:usize>(style: ExpressionStyle<'a>) -> impl AssessedErrorStringifier<'a,[OwnedExpressionInProof;N],()> {
    move |os: [OwnedExpressionInProof; N],_| format!(
        "Expression values expected to be inequal, but weren't; {values}",
        values = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expressions](OwnedExpressionInProof) and returns an error message if these expressions values aren't inequal
pub fn expression_value_inequality_check<'a,const N: usize>(style: ExpressionStyle<'a>) -> ErrorStringifiableAssessor<'a,[OwnedExpressionInProof;N],(),()> { ErrorStringifiableAssessor::new(
    expression_value_inequality_predicate(),
    expression_value_inequality_stringifier(style),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have inequal values, returning an error otherwise
pub fn assert_expression_value_inequality<'a,const N: usize>(exprs: [OwnedExpressionInProof; N], style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_value_inequality_check(style).evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
