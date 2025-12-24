use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

use crate::assertions::utils::stringify_atomicity;

pub struct ExpressionAtomicityEqualityError {
    pub expressions: Vec<OwnedExpressionInInference>
}
pub fn format_expression_atomicity_equality_error(err: ExpressionAtomicityEqualityError) -> String {
    format!("Expression atomicities expected to all be equal, but weren't; {atomicities}",
        atomicities = itertools::join(err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            stringify_atomicity(o.0.obj().as_atom().is_ok())
        ),", ")
    )
}
/// Check that the provided [Expressions](ExpressionInInference) have equal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_equality<'a>(exprs: &[ExpressionInInference]) -> Result<bool, ExpressionAtomicityEqualityError> {
    let mut iter = exprs.iter().map(|o| o.0.obj().as_atom().is_ok());
    let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero expressions");
    for nth_atomicity in iter {
        if nth_atomicity != first_atomicity { return Err(ExpressionAtomicityEqualityError{
            expressions: exprs.into_iter().map(|x| x.clone().into_owned()).collect()
        }) }
    }
    Ok(first_atomicity)
}





pub struct FixedLengthExpressionAtomicityEqualityError<const N: usize> {
    pub expressions: [OwnedExpressionInInference; N]
}
pub fn format_fixed_length_expression_atomicity_equality_error<const N: usize>(err: FixedLengthExpressionAtomicityEqualityError<N>) -> String {
    format!("Expression atomicities expected to all be equal, but weren't; {atomicities}",
        atomicities = itertools::join(err.expressions.iter().map(|o|
            o.0.path().to_string()
            + " -> " +
            stringify_atomicity(o.0.obj().as_atom().is_ok())
        ),", ")
    )
}
/// Check that the provided [Expressions](ExpressionInInference) have equal atomicity, returning an error otherwise
pub fn assert_fixed_length_expression_atomicity_equality<'a,const N: usize>(exprs: &[ExpressionInInference; N]) -> Result<bool, FixedLengthExpressionAtomicityEqualityError<N>> {
    if N == 0 { panic!("Cannot check atomicity equality for zero expressions") } 
    let mut output = [false; N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].0.obj().as_atom().is_ok();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthExpressionAtomicityEqualityError{
                expressions: exprs.clone().map(|x| x.clone().into_owned())
            })
        }
    }
    Ok(output[0])
}
