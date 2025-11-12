use std::fmt::Display;

use tbl_structures::path_composites::{ExpressionInProof, OwnedExpressionInProof};

use crate::assertions::stringify_length;


pub struct ExpressionLengthEqualityError {
    expressions: Vec<OwnedExpressionInProof>
}
impl ExpressionLengthEqualityError {
    pub fn new(expressions: Vec<OwnedExpressionInProof>) -> Self
        { Self { expressions } }
}
impl Display for ExpressionLengthEqualityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Expression lengths expected to all be equal, but weren't; {atomicities}",
            atomicities = self.expressions.iter().map(|o|
                o.0.path().to_string()
                + " -> " +
                &stringify_length(o.0.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
}

/// Check that the provided [Expressions](OwnedExpressionInProof) have equal length, returning an error otherwise
pub fn assert_expression_length_equality<'a, T: From<ExpressionLengthEqualityError>>(exprs: &[ExpressionInProof]) -> Result<Option<usize>, T> {
    let mut iter = exprs.iter().map(|o| match o.0.obj().as_slice() {
        Ok(expressions) => Some(expressions.len()),
        Err(_) => None,
    });
    let first_length = iter.next().expect("Cannot check length equality for zero expressions");
    for nth_length in iter {
        if nth_length != first_length { return Err(ExpressionLengthEqualityError::new(
            exprs.into_iter().map(|x| x.into_owned()).collect()
        ).into()) }
    }
    Ok(first_length)
}
