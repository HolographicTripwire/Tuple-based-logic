use crate::structures::{expressions::{compound::CompoundTblExpression, located::{OwnedTblExpressionAtPath, TblExpressionAtPath}}};

pub struct ExpressionLengthEqualityError<C: CompoundTblExpression, Path> {
    pub expressions: Box<[OwnedTblExpressionAtPath<C,Path>]>
}
/// Check that the provided [Expressions](ExpressionInInference) have equal length, returning an error otherwise
pub fn assert_expression_length_equality<'a,C: CompoundTblExpression, Path: Clone>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>]) -> Result<Option<usize>, ExpressionLengthEqualityError<C,Path>> {
    let mut iter = exprs.iter().map(|o| o.obj.len() );
    let first_length = iter.next().expect("Cannot check length equality for zero expressions");
    for nth_length in iter {
        if nth_length != first_length { return Err(ExpressionLengthEqualityError {
            expressions: exprs.into_iter().map(|x| (*x).clone().into()).collect()
        }) }
    }
    Ok(first_length)
}

pub struct FixedLengthExpressionLengthEqualityError<const N: usize,C: CompoundTblExpression,Path> {
    pub expressions: [OwnedTblExpressionAtPath<C,Path>; N]
}
/// Check that the provided [Expressions](ExpressionInInference) have equal length, returning an error otherwise
pub fn assert_fixed_length_expression_length_equality<'a,const N: usize,C: CompoundTblExpression,Path: Clone>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>; N]) -> Result<Option<usize>, FixedLengthExpressionLengthEqualityError<N,C,Path>> {
    if N == 0 { panic!("Cannot check length equality for zero expressions") } 
    let mut output = [None; N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].obj.len();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthExpressionLengthEqualityError{
                expressions: exprs.clone().map(|x| (*x).clone().into())
            })
        }
    }
    Ok(output[0])
}
