use proof_calculus::utils::collections::binding::bounds::GetBound;
use ref_cast::RefCast;

use crate::expressions::assigned::subexpressions::TblSubexpressionInExpressionPath;

#[derive(Default,Clone,PartialEq,Eq,Debug,Hash,RefCast)]
#[repr(transparent)]
pub struct TblExpressionBoundCompoundExistsAtLocation {
    pub path: TblSubexpressionInExpressionPath,
}
impl TblExpressionBoundCompoundExistsAtLocation {
    pub fn new(path: TblSubexpressionInExpressionPath) -> Self
        { Self { path } }
}
impl GetBound for TblExpressionBoundCompoundExistsAtLocation { type ExtraReturnData = usize; }

pub type TblPropositionBoundCompoundExistsAtLocation = TblExpressionBoundCompoundExistsAtLocation;
