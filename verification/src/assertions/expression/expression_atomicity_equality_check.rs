use tbl_structures::{inference::InferenceRule, path_composites::OwnedExpressionInProof};

use crate::{assertions::expression::stringify_atomicity, errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate}};

/// Get a [Predicate](NaryPredicate) which takes n [Expressions](OwnedExpressionInProof) and checks if their atomicities are equal
pub fn expression_atomicity_equality_predicate<'a,const n: usize>() -> impl NaryPredicate<'a,n,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; n]| { 
        let mut iter = os.iter().map(|o| o.obj().as_atom().is_ok());
        let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero expressions");
        for nth_atomicity in iter {
            if nth_atomicity != first_atomicity { return false }
        }
        true
    }
}
/// Get a [Stringifier](NaryStringifier) which takes n [Expressions](OwnedExpressionInProof) and returns an error message saying that their atomicities aren't equal
pub fn expression_atomicity_equality_stringifier<'a,const n: usize>() -> impl NaryStringifier<'a,n,OwnedExpressionInProof> {
    move |os: [OwnedExpressionInProof; n]| format!(
        "Expression atomicities expected to be equal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.path().to_string()
            + " -> " +
            stringify_atomicity(o.obj().as_atom().is_ok())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expression](OwnedExpressionInProof) and returns an error message if their atomicities aren't equal
pub fn expression_atomicity_equality_check<'a,const n: usize>() -> StringifiablePredicate<'a,n,OwnedExpressionInProof> { StringifiablePredicate::new(
    expression_atomicity_equality_predicate(),
    expression_atomicity_equality_stringifier(),
)}

/// Check that the provided [Expression](OwnedExpressionInProof) has an atomicity equal to atomicty_expected, returning an error otherwise
pub fn assert_expression_atomicity_equality<'a,const n: usize, Rule:InferenceRule>(exprs: [OwnedExpressionInProof; n]) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_atomicity_equality_check::<'a>()
        .evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
