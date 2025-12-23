use tbl_structures::{expressions::Expression, path_composites::{ExpressionInInference, OwnedExpressionInInference}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct ExpressionValueEqualityError {
    pub expressions: Vec<OwnedExpressionInInference>
}

pub fn format_expression_value_equality_error(err: ExpressionValueEqualityError, style: ExpressionStyle) -> String {
    format!("Expression values expected to all be equal, but weren't; {atomicities}",
        atomicities = err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}

/// Check that the provided [Expressions](OwnedExpressionInProof) have equal value, returning an error otherwise
pub fn assert_expression_value_equality<'a>(exprs: &[ExpressionInInference]) -> Result<Expression, ExpressionValueEqualityError> {
    let mut iter = exprs.iter().map(|o| o.0.obj() );
    let first_value = iter.next().expect("Cannot check value equality for zero expressions");
    for nth_value in iter {
        if nth_value != first_value { return Err(ExpressionValueEqualityError{
            expressions: exprs.into_iter().map(|x| x.clone().into_owned()).collect()
        }) }
    }
    Ok(first_value.clone())
}
