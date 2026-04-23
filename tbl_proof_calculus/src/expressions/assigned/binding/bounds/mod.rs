use crate::expressions::assigned::{binding::bounds::{atom_value::TblExpressionBoundAtomExactValue, compound_length::TblExpressionBoundCompoundExactLength}, subexpressions::TblSubexpressionInExpressionPath};

pub mod atom_value;
pub mod compound_length;
pub mod value_duplication;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum TblExpressionIdentityBound {
    AtomValue(TblExpressionBoundAtomExactValue),
    CompoundLength(TblExpressionBoundCompoundExactLength),
}
impl TblExpressionIdentityBound {
    fn path(&self) -> &TblSubexpressionInExpressionPath { match self {
        TblExpressionIdentityBound::AtomValue(atom_bound) => &atom_bound.path,
        TblExpressionIdentityBound::CompoundLength(compound_bound) => &compound_bound.path,
    } }
}
impl From<TblExpressionBoundAtomExactValue> for TblExpressionIdentityBound {
    fn from(bound: TblExpressionBoundAtomExactValue) -> Self
        { Self::AtomValue(bound) }
}
impl From<TblExpressionBoundCompoundExactLength> for TblExpressionIdentityBound {
    fn from(bound: TblExpressionBoundCompoundExactLength) -> Self
        { Self::CompoundLength(bound) }
}

pub type TblPropositionIdentityBound = TblExpressionIdentityBound;
