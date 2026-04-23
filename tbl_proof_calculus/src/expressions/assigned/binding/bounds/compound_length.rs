use crate::expressions::assigned::subexpressions::TblSubexpressionInExpressionPath;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct TblExpressionBoundCompoundExactLength {
    pub path: TblSubexpressionInExpressionPath,
    pub length: usize
}
impl TblExpressionBoundCompoundExactLength {
    pub fn new(path: TblSubexpressionInExpressionPath, length: usize) -> Self
        { Self { path, length } }
}

pub type TblPropositionBoundCompoundExactLength = TblExpressionBoundCompoundExactLength;
