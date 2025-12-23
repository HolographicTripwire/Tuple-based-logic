use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

use crate::assertions::utils::stringify_length;

pub struct ExpressionLengthCheckError {
    pub expected_length: usize,
    pub expression: OwnedExpressionInInference
}

pub fn format_expression_length_check_error(err: ExpressionLengthCheckError) -> String {
    let expression = err.expression.0.obj();
    format!("Expression at {path} has wrong length (expected {length_expected}; found {length_actual})",
        path=err.expression.0.path(),
        length_expected=stringify_length(expression),
        length_actual=stringify_length(expression)
    )
}

/// Check that the provided [Expression](ExpressionInInference) has an length equal to expected_length, returning an error otherwise
pub fn assert_expression_length<'a>(expr: &ExpressionInInference, expected_length: usize) -> Result<(), ExpressionLengthCheckError> {
    match expr.0.obj().as_slice() {
        Ok(tuple) => if tuple.len() == expected_length { Ok(()) }
        else { Err(ExpressionLengthCheckError {
            expected_length,
            expression: expr.clone().into_owned()
        }) },
        Err(()) => Err(ExpressionLengthCheckError {
            expected_length, 
            expression: expr.clone().into_owned()
        })
    }
}
