use crate::expressions::assigned::{atomic::AtomicTblExpression, subexpressions::TblSubexpressionInExpressionPath};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct TblExpressionBoundAtomExactValue {
    pub path: TblSubexpressionInExpressionPath,
    pub value: AtomicTblExpression
}
impl TblExpressionBoundAtomExactValue {
    #[inline]
    pub fn new(path: TblSubexpressionInExpressionPath, value: AtomicTblExpression) -> Self
        { Self { path, value } }
}

pub type TblPropositionBoundAtomExactValue = TblExpressionBoundAtomExactValue;
