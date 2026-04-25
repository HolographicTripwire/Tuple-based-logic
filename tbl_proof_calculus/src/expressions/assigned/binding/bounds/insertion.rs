use crate::expressions::assigned::{binding::bounds::{atom_value::TblExpressionBoundAtomExactValue, compound_length::TblExpressionBoundCompoundExactLength, value_duplication::TblExpressionBoundValueDuplicated}, subexpressions::TblSubexpressionInExpressionPath};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum TblExpressionSubsumptionBound {
    AtomValue(TblExpressionBoundAtomExactValue),
    CompoundLength(TblExpressionBoundCompoundExactLength),
    ValueDuplicated(TblExpressionBoundValueDuplicated)
}
impl TblExpressionSubsumptionBound {
    fn path(&self) -> (&TblSubexpressionInExpressionPath,Option<&TblSubexpressionInExpressionPath>) { match self {
        TblExpressionSubsumptionBound::AtomValue(atom_bound) => (&atom_bound.path,None),
        TblExpressionSubsumptionBound::CompoundLength(compound_bound) => (&compound_bound.path,None),
        TblExpressionSubsumptionBound::ValueDuplicated(duplication_bound) => (duplication_bound.path1(),Some(duplication_bound.path2())),
    }}
}
impl From<TblExpressionBoundAtomExactValue> for TblExpressionSubsumptionBound {
    fn from(bound: TblExpressionBoundAtomExactValue) -> Self
        { Self::AtomValue(bound) }
}
impl From<TblExpressionBoundCompoundExactLength> for TblExpressionSubsumptionBound {
    fn from(bound: TblExpressionBoundCompoundExactLength) -> Self
        { Self::CompoundLength(bound) }
}
impl From<TblExpressionBoundValueDuplicated> for TblExpressionSubsumptionBound {
    fn from(bound: TblExpressionBoundValueDuplicated) -> Self
        { Self::ValueDuplicated(bound) }
}

pub type TblPropositionInsertionBound = TblExpressionSubsumptionBound;
