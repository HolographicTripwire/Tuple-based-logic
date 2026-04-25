use crate::expressions::{assigned::{binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength, TblExpressionBoundValueDuplicated}, subexpressions::TblSubexpressionInExpressionPath}, unassigned::binding::bounds::TblExpressionBoundVariableExactValue};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum UnassignedTblExpressionInsertionBound {
    AtomValue(TblExpressionBoundAtomExactValue),
    VariableValue(TblExpressionBoundVariableExactValue),
    CompoundLength(TblExpressionBoundCompoundExactLength),
    ValueDuplicated(TblExpressionBoundValueDuplicated)
}
impl UnassignedTblExpressionInsertionBound {
    fn path(&self) -> (&TblSubexpressionInExpressionPath,Option<&TblSubexpressionInExpressionPath>) { match self {
        UnassignedTblExpressionInsertionBound::AtomValue(atom_bound) => (&atom_bound.path,None),
        UnassignedTblExpressionInsertionBound::VariableValue(variable_bound) => (&variable_bound.path,None),
        UnassignedTblExpressionInsertionBound::CompoundLength(compound_bound) => (&compound_bound.path,None),
        UnassignedTblExpressionInsertionBound::ValueDuplicated(duplication_bound) => (duplication_bound.path1(),Some(duplication_bound.path2())),
    }}
}
impl From<TblExpressionBoundAtomExactValue> for UnassignedTblExpressionInsertionBound {
    fn from(bound: TblExpressionBoundAtomExactValue) -> Self
        { Self::AtomValue(bound) }
}
impl From<TblExpressionBoundVariableExactValue> for UnassignedTblExpressionInsertionBound {
    fn from(bound: TblExpressionBoundVariableExactValue) -> Self
        { Self::VariableValue(bound) }
}
impl From<TblExpressionBoundCompoundExactLength> for UnassignedTblExpressionInsertionBound {
    fn from(bound: TblExpressionBoundCompoundExactLength) -> Self
        { Self::CompoundLength(bound) }
}
impl From<TblExpressionBoundValueDuplicated> for UnassignedTblExpressionInsertionBound {
    fn from(bound: TblExpressionBoundValueDuplicated) -> Self
        { Self::ValueDuplicated(bound) }
}

pub type UnassignedTblPropositionSubsumedBound = UnassignedTblExpressionInsertionBound;
