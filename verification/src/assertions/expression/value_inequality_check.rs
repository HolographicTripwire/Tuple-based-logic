use std::{collections::HashSet, fmt::Display};

use tbl_structures::path_composites::{ExpressionInProof, OwnedExpressionInProof};

pub struct ExpressionValueInequalityError {
    expressions: Vec<OwnedExpressionInProof>
}
impl ExpressionValueInequalityError {
    pub fn new(expressions: Vec<OwnedExpressionInProof>) -> Self
        { Self { expressions } }
}
impl Display for ExpressionValueInequalityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Expression values expected to all be inequal, but weren't; {values}",
            values = self.expressions.iter().map(|o|
                o.0.path().to_string()
                + " -> " +
                &o.0.obj()
            ).collect::<Vec<_>>().join(", ")
        )
    }
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
            exprs.into_iter().map(|x| x.into_owned()).collect()
        ).into()); } }
    Ok(())
}
