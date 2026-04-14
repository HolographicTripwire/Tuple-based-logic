use proof_calculus::structures::propositions::bounds::PropositionBound;

use crate::structures::{expressions::{compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath}, proof_calculus_derived::aliases::propositions::TblProposition};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct TblExpressionBoundCompoundExactLength {
    pub path: TblSubexpressionInExpressionPath,
    pub length: usize
}
impl TblExpressionBoundCompoundExactLength {
    pub fn new(path: TblSubexpressionInExpressionPath, length: usize) -> Self
        { Self { path, length } }
}

pub type TblPropositionBoundCompoundExactLength = TblExpressionBoundCompoundExactLength;
impl <C: CompoundTblExpression> PropositionBound<TblProposition<C>> for TblPropositionBoundCompoundExactLength {}
