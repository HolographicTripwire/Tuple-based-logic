use crate::expressions::types::assigned::{compound::CompoundTblExpression, OwnedTblExpressionAtPath, TblExpressionAtPath};

pub struct ExpressionAtomicityEqualityError<C: CompoundTblExpression,Path> {
    pub expressions: Box<[OwnedTblExpressionAtPath<C,Path>]>
}
/// Check that the provided [Expressions](ExpressionInInference) have equal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_equality<'a,C: CompoundTblExpression,Path:Clone>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>]) -> Result<bool, ExpressionAtomicityEqualityError<C,Path>> {
    let mut iter = exprs.iter().map(|o| o.obj.is_atom());
    let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero expressions");
    for nth_atomicity in iter {
        if nth_atomicity != first_atomicity { return Err(ExpressionAtomicityEqualityError{
            expressions: exprs.into_iter().map(|x| (*x).clone().into()).collect()
        }) }
    }
    Ok(first_atomicity)
}

pub struct FixedLengthExpressionAtomicityEqualityError<const N: usize,C: CompoundTblExpression,Path> {
    pub expressions: [OwnedTblExpressionAtPath<C,Path>; N]
}
/// Check that the provided [Expressions](ExpressionInInference) have equal atomicity, returning an error otherwise
pub fn assert_fixed_length_expression_atomicity_equality<'a,const N: usize,C,Path>(exprs: &[&'a TblExpressionAtPath<'a,C,Path>; N]) -> Result<bool, FixedLengthExpressionAtomicityEqualityError<N,C,Path>> where
C: CompoundTblExpression, Path: Clone {
    if N == 0 { panic!("Cannot check atomicity equality for zero expressions") } 
    let mut output = [false; N];  // Initialize the output array
    for i in 0..N {
        output[i] = exprs[i].obj.is_atom();
        // Throw error if atomicities are not equal
        if output[i] != output[0] {
            return Err(FixedLengthExpressionAtomicityEqualityError{
                expressions: exprs.clone().map(|x| (*x).clone().into())
            })
        }
    }
    Ok(output[0])
}
