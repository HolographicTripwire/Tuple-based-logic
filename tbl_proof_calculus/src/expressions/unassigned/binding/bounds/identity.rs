use proof_calculus::utils::collections::binding::bounds::GetBound;

use crate::expressions::{assigned::{binding::bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundCompoundExactLength}, subexpressions::TblSubexpressionInExpressionPath}, unassigned::binding::bounds::UnassignedTblExpressionBoundVariableExactValue};


#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum UnassignedTblExpressionIdentityBound {
    AtomValue(TblExpressionBoundAtomExactValue),
    VariableValue(UnassignedTblExpressionBoundVariableExactValue),
    CompoundLength(TblExpressionBoundCompoundExactLength),
}
impl UnassignedTblExpressionIdentityBound {
    fn path(&self) -> &TblSubexpressionInExpressionPath { match self {
        UnassignedTblExpressionIdentityBound::AtomValue(atom_bound) => &atom_bound.path,
        UnassignedTblExpressionIdentityBound::VariableValue(variable_bound) => &variable_bound.path,
        UnassignedTblExpressionIdentityBound::CompoundLength(compound_bound) => &compound_bound.path,
    } }
}
impl GetBound for UnassignedTblExpressionIdentityBound { type ExtraReturnData = (); }

impl From<TblExpressionBoundAtomExactValue> for UnassignedTblExpressionIdentityBound {
    fn from(bound: TblExpressionBoundAtomExactValue) -> Self
        { Self::AtomValue(bound) }
}
impl From<UnassignedTblExpressionBoundVariableExactValue> for UnassignedTblExpressionIdentityBound {
    fn from(bound: UnassignedTblExpressionBoundVariableExactValue) -> Self
        { Self::VariableValue(bound) }
}
impl From<TblExpressionBoundCompoundExactLength> for UnassignedTblExpressionIdentityBound {
    fn from(bound: TblExpressionBoundCompoundExactLength) -> Self
        { Self::CompoundLength(bound) }
}

pub type UnassignedTblPropositionIdentityBound = UnassignedTblExpressionIdentityBound;
