use proof_calculus::utils::collections::binding::bounds::GetBound;
use ref_cast::RefCast;

use crate::expressions::{assigned::{atomic::AtomicTblExpression, subexpressions::TblSubexpressionInExpressionPath}, unassigned::variable::TblExpressionVariable};

#[derive(Default,Clone,PartialEq,Eq,Debug,Hash,RefCast)]
#[repr(transparent)]
pub struct UnassignedTblExpressionBoundExpressionExistsAtLocation {
    pub path: TblSubexpressionInExpressionPath
}
impl UnassignedTblExpressionBoundExpressionExistsAtLocation {
    pub fn new(path: TblSubexpressionInExpressionPath) -> Self { Self { path } }
}
impl GetBound for UnassignedTblExpressionBoundExpressionExistsAtLocation { type ExtraReturnData = AtomOrVariableOrCompoundLength; }

pub type UnassignedTblPropositionBoundExpressionExistsAtLocation = UnassignedTblExpressionBoundExpressionExistsAtLocation;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
pub enum AtomOrVariableOrCompoundLength {
    Atom(AtomicTblExpression),
    Variable(TblExpressionVariable),
    CompoundLength(usize)
}
