use proof_calculus::propositions::types::normalised_unassigned::NormalisedUnassignedProposition;

use crate::expressions::types::unassigned::{
    UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
};

pub struct NormalisedUnassignedTblExpression<C: UnassignedTblExpressionCompound>(
    UnassignedTblExpression<C>,
);
pub type NormalisedUnassignedTblProposition<C: UnassignedTblExpressionCompound> =
    NormalisedUnassignedTblExpression<C>;

impl<C: UnassignedTblExpressionCompound> NormalisedUnassignedTblProposition<C> {
    fn new_unchecked(inner: UnassignedTblExpression<C>) -> Self {
        Self(inner)
    }
}
impl<C: UnassignedTblExpressionCompound> Into<UnassignedTblExpression<C>>
    for NormalisedUnassignedTblExpression<C>
{
    fn into(self) -> UnassignedTblExpression<C> {
        self.0
    }
}
impl<C: UnassignedTblExpressionCompound> NormalisedUnassignedProposition
    for NormalisedUnassignedTblProposition<C>
{
    type Inner = UnassignedTblExpression<C>;
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    fn into_inner(self) -> Self::Inner {
        self.0
    }
}
