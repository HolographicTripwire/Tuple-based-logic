use tbl_structures::{atoms::AtomId, expressions::Expression, path_composites::{ExpressionInInference, OwnedExpressionInInference}};
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





pub struct FixedLengthExpressionValueEqualityError<const N: usize> {
    pub expressions: [OwnedExpressionInInference; N]
}
pub fn format_fixed_length_expression_value_equality_error<const N: usize>(err: FixedLengthExpressionValueEqualityError<N>, style: ExpressionStyle) -> String {
    format!("Expression values expected to all be equal, but weren't; {atomicities}",
        atomicities = err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).collect::<Vec<_>>().join(", ")
    )
}
/// Check that the provided [Expressions](ExpressionInInference) have equal length, returning an error otherwise
pub fn assert_fixed_length_expression_value_equality<'a,const N: usize>(exprs: &[ExpressionInInference; N]) -> Result<Expression, FixedLengthExpressionValueEqualityError<N>> {
    if N == 0 { panic!("Cannot check value equality for zero expressions") } 
    let mut output = [&Expression::Atomic(AtomId(0)); N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].0.obj();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthExpressionValueEqualityError{
                expressions: exprs.clone().map(|x| x.clone().into_owned())
            })
        }
    }
    Ok(output[0].clone())
}
