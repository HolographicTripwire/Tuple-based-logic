use std::collections::HashSet;

use tbl_structures::path_composites::{ExpressionInProof, OwnedExpressionInProof};

use crate::assertions::utils::stringify_length;

pub struct ExpressionLengthInequalityError {
    expressions: Vec<OwnedExpressionInProof>
}
impl ExpressionLengthInequalityError {
    pub fn new(expressions: Vec<OwnedExpressionInProof>) -> Self
        { Self { expressions } }
}

pub fn format_expression_length_inequality_error(err: ExpressionLengthInequalityError) -> String {
    format!("Expression lengths expected to all be inequal, but weren't; {lengths}",
        lengths = err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}


/// Check that the provided [Expressions](OwnedExpressionInProof) have inequal length, returning an error otherwise
pub fn assert_expression_length_inequality<'a, T: From<ExpressionLengthInequalityError>>(exprs: &[ExpressionInProof]) -> Result<(), T> {
    let iter = exprs.iter().map(|o| match o.0.obj().as_slice() {
        Ok(expressions) => Some(expressions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(ExpressionLengthInequalityError::new(
            exprs.into_iter().map(|x| x.clone().into_owned()).collect()
        ).into()); } }
    Ok(())
}
