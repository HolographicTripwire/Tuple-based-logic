use proof_calculus::utils::collections::binding::bounds::GetBound;

use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::{assigned::binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength, TblExpressionBoundValueDuplicated}, unassigned::binding::bounds::UnassignedTblExpressionBoundVariableExistsAtLocation}};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum UnassignedTblExpressionEquivalenceBound {
    AtomValue(TblExpressionBoundAtomExactValue),
    Variable(UnassignedTblExpressionBoundVariableExistsAtLocation),
    CompoundLength(TblExpressionBoundCompoundExactLength),
    ValueDuplicated(TblExpressionBoundValueDuplicated)
}
impl UnassignedTblExpressionEquivalenceBound {
    fn path(&self) -> (&TblSubexpressionInExpressionPath,Option<&TblSubexpressionInExpressionPath>) { match self {
        UnassignedTblExpressionEquivalenceBound::AtomValue(atom_bound) => (&atom_bound.path,None),
        UnassignedTblExpressionEquivalenceBound::Variable(variable_bound) => (&variable_bound.path,None),
        UnassignedTblExpressionEquivalenceBound::CompoundLength(compound_bound) => (&compound_bound.path,None),
        UnassignedTblExpressionEquivalenceBound::ValueDuplicated(duplication_bound) => (duplication_bound.path1(),Some(duplication_bound.path2())),
    }}
}
impl GetBound for UnassignedTblExpressionEquivalenceBound { type ExtraReturnData = (); }

impl From<TblExpressionBoundAtomExactValue> for UnassignedTblExpressionEquivalenceBound {
    fn from(bound: TblExpressionBoundAtomExactValue) -> Self
        { Self::AtomValue(bound) }
}
impl From<UnassignedTblExpressionBoundVariableExistsAtLocation> for UnassignedTblExpressionEquivalenceBound {
    fn from(bound: UnassignedTblExpressionBoundVariableExistsAtLocation) -> Self
        { Self::Variable(bound) }
}
impl From<TblExpressionBoundCompoundExactLength> for UnassignedTblExpressionEquivalenceBound {
    fn from(bound: TblExpressionBoundCompoundExactLength) -> Self
        { Self::CompoundLength(bound) }
}
impl From<TblExpressionBoundValueDuplicated> for UnassignedTblExpressionEquivalenceBound {
    fn from(bound: TblExpressionBoundValueDuplicated) -> Self
        { Self::ValueDuplicated(bound) }
}

pub type TblPropositionEquivalenceBound = UnassignedTblExpressionEquivalenceBound;
