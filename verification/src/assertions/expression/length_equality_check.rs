use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

use crate::assertions::utils::stringify_length;


pub struct ExpressionLengthEqualityError {
    pub expressions: Vec<OwnedExpressionInInference>
}

pub fn format_expression_length_equality_error(err: ExpressionLengthEqualityError) -> String {
    format!("Expression lengths expected to all be equal, but weren't; {atomicities}",
        atomicities = err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}

/// Check that the provided [Expressions](ExpressionInInference) have equal length, returning an error otherwise
pub fn assert_expression_length_equality<'a>(exprs: &[ExpressionInInference]) -> Result<Option<usize>, ExpressionLengthEqualityError> {
    let mut iter = exprs.iter().map(|o| match o.0.obj().as_slice() {
        Ok(expressions) => Some(expressions.len()),
        Err(_) => None,
    });
    let first_length = iter.next().expect("Cannot check length equality for zero expressions");
    for nth_length in iter {
        if nth_length != first_length { return Err(ExpressionLengthEqualityError {
            expressions: exprs.into_iter().map(|x| x.clone().into_owned()).collect()
        }) }
    }
    Ok(first_length)
}
