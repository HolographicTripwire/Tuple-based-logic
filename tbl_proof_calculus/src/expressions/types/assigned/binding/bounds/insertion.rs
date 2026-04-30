use proof_calculus::utils::collections::binding::bounds::GetBound;

use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::assigned::binding::bounds::{atom_value::TblExpressionBoundAtomExactValue, compound_length::TblExpressionBoundCompoundExactLength, value_duplication::TblExpressionBoundValueDuplicated}};

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum TblExpressionInsertionBound {
    AtomValue(TblExpressionBoundAtomExactValue),
    CompoundLength(TblExpressionBoundCompoundExactLength),
    ValueDuplicated(TblExpressionBoundValueDuplicated)
}
impl TblExpressionInsertionBound {
    fn path(&self) -> (&TblSubexpressionInExpressionPath,Option<&TblSubexpressionInExpressionPath>) { match self {
        TblExpressionInsertionBound::AtomValue(atom_bound) => (&atom_bound.path,None),
        TblExpressionInsertionBound::CompoundLength(compound_bound) => (&compound_bound.path,None),
        TblExpressionInsertionBound::ValueDuplicated(duplication_bound) => (duplication_bound.path1(),Some(duplication_bound.path2())),
    }}
}
impl GetBound for TblExpressionInsertionBound { type ExtraReturnData = (); }

impl From<TblExpressionBoundAtomExactValue> for TblExpressionInsertionBound {
    fn from(bound: TblExpressionBoundAtomExactValue) -> Self
        { Self::AtomValue(bound) }
}
impl From<TblExpressionBoundCompoundExactLength> for TblExpressionInsertionBound {
    fn from(bound: TblExpressionBoundCompoundExactLength) -> Self
        { Self::CompoundLength(bound) }
}
impl From<TblExpressionBoundValueDuplicated> for TblExpressionInsertionBound {
    fn from(bound: TblExpressionBoundValueDuplicated) -> Self
        { Self::ValueDuplicated(bound) }
}

pub type TblPropositionInsertionBound = TblExpressionInsertionBound;
