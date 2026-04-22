use crate::expressions::assigned::{TblExpression, compound::CompoundTblExpression, OwnedTblExpressionAtPath, TblExpressionAtPath};

#[derive(Clone)]
pub struct ExpressionAtomicityCheckError<C: CompoundTblExpression, Path> {
    pub expected_atomicity: bool,
    pub expression: OwnedTblExpressionAtPath<C,Path>
}
impl <C: CompoundTblExpression,Path> ExpressionAtomicityCheckError<C,Path> {
    pub fn get_actual_atomicity(&self) -> bool { self.expression.obj.len().is_none() }
    pub fn into_expression(self) -> TblExpression<C> { self.expression.obj }
}

/// Check that the provided [Expression](ExpressionInInference) has an atomicity equal to expected_atomicity, returning an error otherwise
pub fn assert_expression_atomicity<'a,C,Path>(expr: &TblExpressionAtPath<'a,C,Path>, expected_atomicity: bool) -> Result<(), ExpressionAtomicityCheckError<C,Path>> where
C: CompoundTblExpression, Path: Clone {
    if expr.obj.is_atom() == expected_atomicity { Ok(()) }
    else { Err(ExpressionAtomicityCheckError{
        expected_atomicity, 
        expression: expr.clone().into() 
    }) }
}
