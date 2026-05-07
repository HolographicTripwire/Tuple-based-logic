use proof_calculus::utils::collections::binding::bounds::GetBound;
use ref_cast::RefCast;

use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::assigned::atom::TblExpressionAtom};

#[derive(Default,Clone,PartialEq,Eq,Debug,Hash,RefCast)]
#[repr(transparent)]
pub struct TblExpressionBoundExpressionExistsAtLocation {
    pub path: TblSubexpressionInExpressionPath
}
impl TblExpressionBoundExpressionExistsAtLocation {
    pub fn new(path: TblSubexpressionInExpressionPath) -> Self { Self { path } }
}
impl GetBound for TblExpressionBoundExpressionExistsAtLocation { type ExtraReturnData = AtomOrCompoundLength; }

pub type TblPropositionBoundExpressionExistsAtLocation = TblExpressionBoundExpressionExistsAtLocation;

