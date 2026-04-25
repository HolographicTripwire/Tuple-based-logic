use crate::expressions::{assigned::{binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength, TblExpressionBoundValueDuplicated}, subexpressions::TblSubexpressionInExpressionPath}, unassigned::binding::bounds::TblExpressionBoundVariableExistsAtLocation};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum UnassignedTblExpressionEquivalenceBound {
    AtomValue(TblExpressionBoundAtomExactValue),
    Variable(TblExpressionBoundVariableExistsAtLocation),
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
impl From<TblExpressionBoundAtomExactValue> for UnassignedTblExpressionEquivalenceBound {
    fn from(bound: TblExpressionBoundAtomExactValue) -> Self
        { Self::AtomValue(bound) }
}
impl From<TblExpressionBoundVariableExistsAtLocation> for UnassignedTblExpressionEquivalenceBound {
    fn from(bound: TblExpressionBoundVariableExistsAtLocation) -> Self
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
