use crate::expressions::types::assigned::{TblExpression, compound::CompoundTblExpression, OwnedTblExpressionAtPath, TblExpressionAtPath};

#[derive(Clone)]
pub struct ExpressionLengthCheckError<C: CompoundTblExpression, Path> {
    pub expected_length: usize,
    pub expression: OwnedTblExpressionAtPath<C,Path>
}
impl <C: CompoundTblExpression, Path> ExpressionLengthCheckError<C,Path> {
    pub fn get_actual_length(&self) -> Option<usize> { self.expression.obj.len() }
    pub fn into_expression(self) -> TblExpression<C> { self.expression.obj }
}

/// Check that the provided [Expression](ExpressionInInference) has an length equal to expected_length, returning an error otherwise
pub fn assert_expression_length<'a,C: CompoundTblExpression, Path: Clone>(expr: &TblExpressionAtPath<'a,C,Path>, expected_length: usize) -> Result<(), ExpressionLengthCheckError<C,Path>> {
    match expr.obj {
        TblExpression::Atomic(_) => Err(ExpressionLengthCheckError {
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
