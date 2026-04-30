use crate::expressions::types::assigned::{TblExpression, compound::CompoundTblExpression, OwnedTblExpressionAtPath, TblExpressionAtPath};

#[derive(Clone)]
pub struct ExpressionValueCheckError<C1: CompoundTblExpression, Path,C2:CompoundTblExpression> {
    pub expected_value: TblExpression<C2>,
    pub expression: OwnedTblExpressionAtPath<C1,Path>,
}
impl <C1:CompoundTblExpression,Path,C2:CompoundTblExpression> ExpressionValueCheckError<C1,Path,C2> {
    pub fn get_actual_value(&self) -> &TblExpression<C1> { &self.expression.obj }
    pub fn into_expression(self) -> TblExpression<C1> { self.expression.obj }
}

/// Check that the provided [Expression](ExpressionInInference) has an value equal to expected_value, returning an error otherwise
pub fn assert_expression_value<'a,C1:CompoundTblExpression + PartialEq<C2>,Path: Clone,C2:CompoundTblExpression + PartialEq<C1>>(expr: &TblExpressionAtPath<'a,C1,Path>, expected_value: &TblExpression<C2>) -> Result<(), ExpressionValueCheckError<C1,Path,C2>> {
    if expr.obj == expected_value { Ok(()) }
    else { Err(ExpressionValueCheckError{
        expected_value: expected_value.clone(),
        expression: expr.clone().into()
    }) }
}
