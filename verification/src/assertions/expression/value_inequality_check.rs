use std::collections::HashSet;

use tbl_structures::path_composites::{ExpressionInProof, OwnedExpressionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct ExpressionValueInequalityError {
    expressions: Vec<OwnedExpressionInProof>,
}
impl ExpressionValueInequalityError {
    pub fn new(expressions: Vec<OwnedExpressionInProof>) -> Self
        { Self { expressions } }
}


pub fn format_expression_value_inequality_error(err: ExpressionValueInequalityError, style: ExpressionStyle) -> String {
    format!("Expression values expected to all be inequal, but weren't; {values}",
        values = err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}


/// Check that the provided [Expressions](OwnedExpressionInProof) have inequal value, returning an error otherwise
pub fn assert_expression_value_inequality<'a, T: From<ExpressionValueInequalityError>>(exprs: &[ExpressionInProof]) -> Result<(), T> {
    let iter = exprs.iter().map(|o| match o.0.obj().as_slice() {
        Ok(expressions) => Some(expressions.len()),
        Err(_) => None,
    });
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(ExpressionValueInequalityError::new(
            exprs.into_iter().map(|x| x.clone().into_owned()).collect()
        ).into()); } }
    Ok(())
}
