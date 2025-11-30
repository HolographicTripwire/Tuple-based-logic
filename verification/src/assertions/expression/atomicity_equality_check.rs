use tbl_structures::path_composites::{ExpressionInProof, OwnedExpressionInProof};

use crate::assertions::utils::stringify_atomicity;

pub struct ExpressionAtomicityEqualityError {
    expressions: Vec<OwnedExpressionInProof>
}
impl ExpressionAtomicityEqualityError {
    pub fn new(expressions: Vec<OwnedExpressionInProof>) -> Self
        { Self { expressions } }
}

pub fn format_expression_atomicity_equality_error(err: ExpressionAtomicityEqualityError) -> String {
    format!("Expression atomicities expected to all be equal, but weren't; {atomicities}",
        atomicities = itertools::join(err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            stringify_atomicity(o.0.obj().as_atom().is_ok())
        ),", ")
    )
}

/// Check that the provided [Expressions](OwnedExpressionInProof) have equal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_equality<'a, T: From<ExpressionAtomicityEqualityError>>(exprs: &[ExpressionInProof]) -> Result<(), T> {
    let mut iter = exprs.iter().map(|o| o.0.obj().as_atom().is_ok());
    let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero expressions");
    for nth_atomicity in iter {
        if nth_atomicity != first_atomicity { return Err(ExpressionAtomicityEqualityError::new(
            exprs.into_iter().map(|x| x.clone().into_owned()).collect()
        ).into()) }
    }
    Ok(())
}
