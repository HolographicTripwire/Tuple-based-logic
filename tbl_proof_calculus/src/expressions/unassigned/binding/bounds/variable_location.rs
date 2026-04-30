use proof_calculus::utils::collections::binding::bounds::GetBound;
use ref_cast::RefCast;

use crate::expressions::{assigned::subexpressions::TblSubexpressionInExpressionPath, unassigned::variable::TblExpressionVariable};

#[derive(Default,Clone,PartialEq,Eq,Debug,Hash,RefCast)]
#[repr(transparent)]
pub struct UnassignedTblExpressionBoundVariableExistsAtLocation {
    pub path: TblSubexpressionInExpressionPath,
}
impl UnassignedTblExpressionBoundVariableExistsAtLocation {
    pub fn new(path: TblSubexpressionInExpressionPath) -> Self
        { Self { path } }
}
impl GetBound for UnassignedTblExpressionBoundVariableExistsAtLocation { type ExtraReturnData = TblExpressionVariable; }

pub type UnassignedTblPropositionBoundVariableExistsAtLocation = UnassignedTblExpressionBoundVariableExistsAtLocation;
