use tbl_structures::{expressions::Expression, path_composites::{ExpressionInProof, OwnedExpressionInProof}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct ExpressionValueEqualityError {
    expressions: Vec<OwnedExpressionInProof>
}
impl ExpressionValueEqualityError {
    pub fn new(expressions: Vec<OwnedExpressionInProof>) -> Self
        { Self { expressions } }
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
pub fn assert_expression_value_equality<'a, T: From<ExpressionValueEqualityError>>(exprs: &[ExpressionInProof]) -> Result<Expression, T> {
    let mut iter = exprs.iter().map(|o| o.0.obj() );
    let first_value = iter.next().expect("Cannot check value equality for zero expressions");
    for nth_value in iter {
        if nth_value != first_value { return Err(ExpressionValueEqualityError::new(
            exprs.into_iter().map(|x| x.clone().into_owned()).collect()
        ).into()) }
    }
    Ok(first_value.clone())
}
