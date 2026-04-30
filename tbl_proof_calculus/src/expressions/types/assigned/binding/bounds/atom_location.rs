use proof_calculus::utils::collections::binding::bounds::GetBound;
use ref_cast::RefCast;

use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::assigned::atomic::AtomicTblExpression};

#[derive(Default,Clone,PartialEq,Eq,Debug,Hash,RefCast)]
#[repr(transparent)]
pub struct TblExpressionBoundAtomExistsAtLocation {
    pub path: TblSubexpressionInExpressionPath
}
impl TblExpressionBoundAtomExistsAtLocation {
    pub fn new(path: TblSubexpressionInExpressionPath) -> Self { Self { path } }
}
impl GetBound for TblExpressionBoundAtomExistsAtLocation { type ExtraReturnData = AtomicTblExpression; }

pub type TblPropositionBoundAtomExistsAtLocation = TblExpressionBoundAtomExistsAtLocation;
