use proof_calculus::utils::collections::binding::bounds::GetBound;

use crate::expressions::{assigned::subexpressions::TblSubexpressionInExpressionPath, unassigned::variable::TblExpressionVariable};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct UnassignedTblExpressionBoundVariableExactValue {
    pub path: TblSubexpressionInExpressionPath,
    pub value: TblExpressionVariable
}
impl UnassignedTblExpressionBoundVariableExactValue {
    pub fn new(path: TblSubexpressionInExpressionPath, value: TblExpressionVariable) -> Self
        { Self { path, value } }
}
impl GetBound for UnassignedTblExpressionBoundVariableExactValue { type ExtraReturnData = (); }

pub type UnassignedTblPropositionBoundVariableExactValue = UnassignedTblExpressionBoundVariableExactValue;
