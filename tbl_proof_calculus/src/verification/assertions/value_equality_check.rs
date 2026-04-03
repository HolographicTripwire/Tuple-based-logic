
use crate::structures::expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression, located::{OwnedTblExpressionAtPath, TblExpressionAtPath}};

pub struct ExpressionValueEqualityError<C: CompoundTblExpression, Path> {
    pub expressions: Box<[OwnedTblExpressionAtPath<C,Path>]>
}
/// Check that the provided [Expressions](OwnedExpressionInProof) have equal value, returning an error otherwise
pub fn assert_expression_value_equality<'a,C:CompoundTblExpression,Path:Clone>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>]) -> Result<TblExpression<C>, ExpressionValueEqualityError<C,Path>> {
    let mut iter = exprs.iter().map(|o| o.obj );
    let first_value = iter.next().expect("Cannot check value equality for zero expressions");
    for nth_value in iter {
        if nth_value != first_value { return Err(ExpressionValueEqualityError{
            expressions: exprs.iter().map(|x| (*x).clone().into()).collect()
        }) }
    }
    Ok(first_value.clone())
}

pub struct FixedLengthExpressionValueEqualityError<const N: usize,C: CompoundTblExpression,Path> {
    pub expressions: [OwnedTblExpressionAtPath<C,Path>; N]
}
/// Check that the provided [Expressions](ExpressionInInference) have equal length, returning an error otherwise
pub fn assert_fixed_length_expression_value_equality<'a,const N: usize,C:CompoundTblExpression,Path:Clone>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>; N]) -> Result<TblExpression<C>, FixedLengthExpressionValueEqualityError<N,C,Path>> {
    if N == 0 { panic!("Cannot check value equality for zero expressions") } 
    let mut output = [&TblExpression::Atomic(AtomicTblExpression(0)); N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].obj;
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthExpressionValueEqualityError{
                expressions: exprs.clone().map(|x| (*x).clone().into())
            })
        }
    }
    Ok(output[0].clone())
}
