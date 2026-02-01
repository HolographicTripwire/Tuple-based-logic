use std::collections::HashSet;

use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

pub struct ExpressionValueInequalityError {
    pub expressions: Vec<OwnedExpressionInInference>,
}

pub fn format_expression_value_inequality_error(err: ExpressionValueInequalityError, style: ExpressionStyle) -> String {
    format!("Proposition values expected to all be inequal, but weren't; {values}",
        values = err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}

/// Check that the provided [Propositions](PropositionInInference) have inequal value, returning an error otherwise
pub fn assert_expression_value_inequality<'a>(exprs: &[&'a ExpressionInInference<'a>]) -> Result<(), ExpressionValueInequalityError> {
    let iter = exprs.iter().map(|o| o.0.obj());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(ExpressionValueInequalityError{
            expressions: exprs.into_iter().map(|x| (*x).clone().into_owned()).collect()
        }); } }
    Ok(())
}





pub struct FixedLengthExpressionValueInequalityError<const N: usize> {
    pub expressions: [OwnedExpressionInInference; N]
}
pub fn format_fixed_length_expression_value_inequality_error<const N: usize>(err: FixedLengthExpressionValueInequalityError<N>, style: ExpressionStyle) -> String {
    format!("Expression lengths expected to all be equal, but weren't; {atomicities}",
        atomicities = err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}
/// Check that the provided [Expressions](ExpressionInInference) have inequal length, returning an error otherwise
pub fn assert_fixed_length_expression_value_inequality<'a,const N: usize>(exprs: &[&'a ExpressionInInference<'a>; N]) -> Result<(), FixedLengthExpressionValueInequalityError<N>> {
    if N == 0 { panic!("Cannot check length inequality for zero expressions") } 
    let iter = exprs.iter().map(|o| o.0.obj());
    let mut values = HashSet::new();
    for value in iter
        { if !values.insert(value) { return Err(FixedLengthExpressionValueInequalityError {
            expressions: exprs.clone().map(|x| (*x).clone().into_owned())
        }); } }
    Ok(())
}
