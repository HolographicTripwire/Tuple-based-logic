use crate::expressions::{TblExpressionLength, types::assigned::{OwnedTblExpressionAtPath, TblExpression, TblExpressionAtPath, compound::TblExpressionCompound}};

#[derive(Clone)]
pub struct ExpressionLengthCheckError<C: TblExpressionCompound, Path> {
    pub expected_length: usize,
    pub expression: OwnedTblExpressionAtPath<C,Path>
}
impl <C: TblExpressionCompound, Path> ExpressionLengthCheckError<C,Path> {
    pub fn get_actual_length(&self) -> TblExpressionLength { self.expression.obj.len() }
    pub fn into_expression(self) -> TblExpression<C> { self.expression.obj }
}

/// Check that the provided [Expression](ExpressionInInference) has an length equal to expected_length, returning an error otherwise
pub fn assert_expression_length<'a,C: TblExpressionCompound, Path: Clone>(expr: &TblExpressionAtPath<'a,C,Path>, expected_length: usize) -> Result<(), ExpressionLengthCheckError<C,Path>> {
    match expr.obj {
        TblExpression::Atom(_) => Err(ExpressionLengthCheckError {
            expected_length, 
            expression: expr.clone().into()
        }),
        TblExpression::Compound(compound) => if compound.len() == expected_length { Ok(()) }
        else { Err(ExpressionLengthCheckError {
            expected_length,
            expression: expr.clone().into()
        }) }
    }
}
