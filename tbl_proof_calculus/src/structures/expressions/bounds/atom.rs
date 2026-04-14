use proof_calculus::structures::propositions::bounds::PropositionBound;

use crate::structures::{expressions::{atomic::AtomicTblExpression, compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath}, proof_calculus_derived::aliases::propositions::TblProposition};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct TblExpressionBoundAtomExactValue{
    pub path: TblSubexpressionInExpressionPath,
    pub value: AtomicTblExpression
}
impl TblExpressionBoundAtomExactValue {
    #[inline]
    pub fn new(path: TblSubexpressionInExpressionPath, value: AtomicTblExpression) -> Self
        { Self { path, value } }
}

pub type TblPropositionBoundAtomExactValue = TblExpressionBoundAtomExactValue;
impl <C: CompoundTblExpression> PropositionBound<TblProposition<C>> for TblPropositionBoundAtomExactValue {}



