use crate::expressions::{assigned::subexpressions::TblSubexpressionInExpressionPath, unassigned::variable::TblExpressionVariable};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct TblExpressionBoundVariableExactValue {
    pub path: TblSubexpressionInExpressionPath,
    pub value: TblExpressionVariable
}
impl TblExpressionBoundVariableExactValue {
    #[inline]
    pub fn new(path: TblSubexpressionInExpressionPath, value: TblExpressionVariable) -> Self
        { Self { path, value } }
}

pub type TblPropositionBoundVariableExactValue = TblExpressionBoundVariableExactValue;
