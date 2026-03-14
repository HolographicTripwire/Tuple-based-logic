use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};

pub struct ExpressionLengthEqualityError {
    pub expressions: Vec<OwnedExpressionInInference>
}
/// Check that the provided [Expressions](ExpressionInInference) have equal length, returning an error otherwise
pub fn assert_expression_length_equality<'a>(exprs: &[&'a ExpressionInInference<'a>]) -> Result<Option<usize>, ExpressionLengthEqualityError> {
    let mut iter = exprs.iter().map(|o| o.obj().len() );
    let first_length = iter.next().expect("Cannot check length equality for zero expressions");
    for nth_length in iter {
        if nth_length != first_length { return Err(ExpressionLengthEqualityError {
            expressions: exprs.into_iter().map(|x| (*x).clone().into_owned()).collect()
        }) }
    }
    Ok(first_length)
}

pub struct FixedLengthExpressionLengthEqualityError<const N: usize> {
    pub expressions: [OwnedExpressionInInference; N]
}
/// Check that the provided [Expressions](ExpressionInInference) have equal length, returning an error otherwise
pub fn assert_fixed_length_expression_length_equality<'a,const N: usize>(exprs: &[&'a ExpressionInInference<'a>; N]) -> Result<Option<usize>, FixedLengthExpressionLengthEqualityError<N>> {
    if N == 0 { panic!("Cannot check length equality for zero expressions") } 
    let mut output = [None; N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].obj().len();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthExpressionLengthEqualityError{
                expressions: exprs.clone().map(|x| (*x).clone().into_owned())
            })
        }
    }
    Ok(output[0])
}
