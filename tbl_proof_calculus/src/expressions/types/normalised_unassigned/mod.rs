use proof_calculus::propositions::types::normalised_unassigned::NormalisedUnassignedProposition;

use crate::expressions::types::unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression};

pub struct NormalisedUnassignedTblExpression<C: UnassignedCompoundTblExpression>(UnassignedTblExpression<C>);
pub type NormalisedUnassignedTblProposition<C: UnassignedCompoundTblExpression> = NormalisedUnassignedTblExpression<C>;

impl <C: UnassignedCompoundTblExpression> NormalisedUnassignedTblProposition<C> {
    fn new_unchecked(inner: UnassignedTblExpression<C>) -> Self { Self(inner) }
}
impl <C: UnassignedCompoundTblExpression> Into<UnassignedTblExpression<C>> for NormalisedUnassignedTblExpression<C>
    { fn into(self) -> UnassignedTblExpression<C> { self.0 }
}
impl <C: UnassignedCompoundTblExpression> NormalisedUnassignedProposition for NormalisedUnassignedTblProposition<C> {
    type Inner = UnassignedTblExpression<C>;
    fn inner(&self) -> &Self::Inner { &self.0 }
    fn into_inner(self) -> Self::Inner { self.0 }
}
