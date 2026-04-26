use proof_calculus::propositions::normalised_unassigned::NormalisedUnassignedProposition;

use crate::expressions::unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression};

pub struct NormalisedUnassignedTblExpression<C: UnassignedCompoundTblExpression>(UnassignedTblExpression<C>);
pub type NormalisedUnassignedTblProposition<C: UnassignedCompoundTblExpression> = NormalisedUnassignedTblExpression<C>;

impl <C: UnassignedCompoundTblExpression> NormalisedUnassignedTblProposition<C> {
    #[inline]
    fn new_unchecked(inner: UnassignedTblExpression<C>) -> Self { Self(inner) }
}
impl <C: UnassignedCompoundTblExpression> NormalisedUnassignedProposition for NormalisedUnassignedTblProposition<C> {
    type Inner = UnassignedTblExpression<C>;
    #[inline]
    fn inner(&self) -> &Self::Inner { &self.0 }
    #[inline]
    fn into_inner(self) -> Self::Inner { self.0 }
}
