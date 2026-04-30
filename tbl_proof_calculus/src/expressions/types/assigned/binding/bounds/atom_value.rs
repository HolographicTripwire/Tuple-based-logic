use proof_calculus::utils::collections::binding::bounds::GetBound;

use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::assigned::atomic::AtomicTblExpression};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct TblExpressionBoundAtomExactValue {
    pub path: TblSubexpressionInExpressionPath,
    pub value: AtomicTblExpression
}
impl TblExpressionBoundAtomExactValue {
    pub fn new(path: TblSubexpressionInExpressionPath, value: AtomicTblExpression) -> Self
        { Self { path, value } }
}
impl GetBound for TblExpressionBoundAtomExactValue { type ExtraReturnData = (); }

pub type TblPropositionBoundAtomExactValue = TblExpressionBoundAtomExactValue;
