use std::fmt::Display;

use tbl_structures::{expressions::Expression, path_composites::{ExpressionInProof, OwnedExpressionInProof}};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};


pub struct ExpressionValueEqualityError<'a> {
    expressions: Vec<OwnedExpressionInProof>,
    expression_style: ExpressionStyle<'a>
}
impl <'a> ExpressionValueEqualityError<'a> {
    pub fn new(expressions: Vec<OwnedExpressionInProof>, style: ExpressionStyle<'a>) -> Self
        { Self { expressions, expression_style: style } }
}
impl <'a> Display for ExpressionValueEqualityError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Expression values expected to all be equal, but weren't; {atomicities}",
            atomicities = self.expressions.iter().map(|o|
                o.0.path().to_string()
                + " -> " +
                &self.expression_style.stringify(o.0.obj())
            ).collect::<Vec<_>>().join(", ")
        )
    }
}

/// Check that the provided [Expressions](OwnedExpressionInProof) have equal value, returning an error otherwise
pub fn assert_expression_value_equality<'a, T: From<ExpressionValueEqualityError<'a>>>(exprs: &[ExpressionInProof], style: ExpressionStyle<'a>) -> Result<Expression, T> {
    let mut iter = exprs.iter().map(|o| o.0.obj() );
    let first_value = iter.next().expect("Cannot check value equality for zero expressions");
    for nth_value in iter {
        if nth_value != first_value { return Err(ExpressionValueEqualityError::new(
            exprs.into_iter().map(|x| x.into_owned()).collect(),
            style
        ).into()) }
    }
    Ok(first_value)
}
