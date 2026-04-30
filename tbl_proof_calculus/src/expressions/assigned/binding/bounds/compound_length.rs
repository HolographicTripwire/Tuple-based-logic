use proof_calculus::utils::collections::binding::bounds::GetBound;

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
impl GetBound for TblExpressionBoundCompoundExactLength { type ExtraReturnData = (); }

pub type TblPropositionBoundCompoundExactLength = TblExpressionBoundCompoundExactLength;
