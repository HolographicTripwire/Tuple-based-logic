use crate::expressions::assigned::subexpressions::TblSubexpressionInExpressionPath;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct TblExpressionBoundVariableExistsAtLocation {
    pub path: TblSubexpressionInExpressionPath,
}
impl TblExpressionBoundVariableExistsAtLocation {
    #[inline]
    pub fn new(path: TblSubexpressionInExpressionPath) -> Self
        { Self { path } }
}

pub type TblPropositionBoundVariableExists = TblExpressionBoundVariableExistsAtLocation;
