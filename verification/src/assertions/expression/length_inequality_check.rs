use std::collections::HashSet;

use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

use crate::assertions::utils::stringify_length;

pub struct ExpressionLengthInequalityError {
    pub expressions: Vec<OwnedExpressionInInference>
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


/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_expression_length_inequality<'a>(exprs: &[ExpressionInInference]) -> Result<(), ExpressionLengthInequalityError> {
    let iter = exprs.iter().map(|o| match o.0.obj().as_slice() {
        Ok(expressions) => Some(expressions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(ExpressionLengthInequalityError {
            expressions: exprs.into_iter().map(|x| x.clone().into_owned()).collect()
        }); } }
    Ok(())
}
