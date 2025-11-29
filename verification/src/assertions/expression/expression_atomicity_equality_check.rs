use tbl_structures::path_composites::OwnedExpressionInProof;

use crate::{assertions::expression::stringify_atomicity, errors::specification_error::{Assessor, AssessedErrorStringifier, ProofStepSpecificationError, ErrorStringifiableAssessor}};

/// Get an [Assessor] which takes n [Expressions](OwnedExpressionInProof) and checks if their atomicities are equal
pub fn expression_atomicity_equality_predicate<'a,const N: usize>() -> impl Assessor<'a,[OwnedExpressionInProof;N],bool,()> {
    move |os: [OwnedExpressionInProof; N]| { 
        let mut iter = os.iter().map(|o| o.0.obj().as_atom().is_ok());
        let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero expressions");
        for nth_atomicity in iter {
            if nth_atomicity != first_atomicity { return Err(()) }
        }
        Ok(first_atomicity)
    }
}
/// Get an [AssessedErrorStringifier] which takes n [Expressions](OwnedExpressionInProof) and returns an error message saying that their atomicities aren't equal
pub fn expression_atomicity_equality_stringifier<'a,const N: usize>() -> impl AssessedErrorStringifier<'a,[OwnedExpressionInProof;N],()> {
    move |os: [OwnedExpressionInProof; N],_| format!(
        "Expression atomicities expected to be equal, but weren't; {atomicities}",
        atomicities = os.map(|o|
            o.0.path().to_string()
            + " -> " +
            stringify_atomicity(o.0.obj().as_atom().is_ok())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Expression](OwnedExpressionInProof) and returns an error message if their atomicities aren't equal
pub fn expression_atomicity_equality_check<'a,const N: usize>() -> ErrorStringifiableAssessor<'a,[OwnedExpressionInProof;N],bool,()> { ErrorStringifiableAssessor::new(
    expression_atomicity_equality_predicate(),
    expression_atomicity_equality_stringifier(),
)}

/// Check that the provided [Expressions](OwnedExpressionInProof) have equal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_equality<'a,const N: usize>(exprs: [OwnedExpressionInProof; N]) -> Result<(), ProofStepSpecificationError<'a>> {
    expression_atomicity_equality_check::<'a>()
        .evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
