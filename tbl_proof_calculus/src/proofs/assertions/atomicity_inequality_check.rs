use crate::expressions::assigned::{compound::CompoundTblExpression, OwnedTblExpressionAtPath, TblExpressionAtPath};

pub struct ExpressionAtomicityInequalityError<C: CompoundTblExpression, Path> {
    pub expr1: OwnedTblExpressionAtPath<C, Path>,
    pub expr2: OwnedTblExpressionAtPath<C, Path>,
}

/// Check that the provided [Expressions](ExpressionInInference) have inequal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_inequality<'a,C: CompoundTblExpression,Path:Clone>(expr1: &TblExpressionAtPath<C, Path>, expr2: &TblExpressionAtPath<C,Path>) -> Result<(), ExpressionAtomicityInequalityError<C,Path>> {
    let first_atomicity = expr1.obj.is_atom();
    let second_atomicity = expr2.obj.is_atom();
    if first_atomicity == second_atomicity { Ok(()) }
    else { Err(ExpressionAtomicityInequalityError{
        expr1: expr1.clone().into(), 
        expr2: expr2.clone().into()
    }) }
}
