use proof_calculus::{
    propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment},
    utils::{
        collections::maps::{KeyConflictError, dense_usize_map::DenseUsizeMap},
        traits::{
            combinable::TryCombine,
            map::{Map, MapWithoutConflicts},
            try_from_iter::TryFromIterator,
        },
    },
};

use crate::expressions::{
    assignments::{
        errors::{
            assignment::TblAssignmentError, partial_assignment::TblPartialAssignmentError,
            partial_reverse_assignment::TblPartialReverseAssignmentError,
            reverse_assignment::TblReverseAssignmentError,
        },
        implementations::helpers::{TblAssignmentHelper, TblPartialAssignmentHelper},
    },
    types::{
        assigned::{TblExpression, compound::TblExpressionCompound},
        unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    },
};

pub mod constructors;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DenseTblExpressionAssignment<C: TblExpressionCompound>(
    pub DenseUsizeMap<TblExpressionVariable, TblExpression<C>>,
);
pub type DenseTblPropositionAssignment<C: TblExpressionCompound> = DenseTblExpressionAssignment<C>;
impl<C: TblExpressionCompound> DenseTblExpressionAssignment<C> {
    pub fn from_iter_unchecked<
        T: IntoIterator<Item = (TblExpressionVariable, TblExpression<C>)>,
    >(
        iter: T,
    ) -> Self {
        Self(DenseUsizeMap::from_iter(iter))
    }
}
impl<C: TblExpressionCompound> Default for DenseTblExpressionAssignment<C> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<C: TblExpressionCompound> TryFromIterator<(TblExpressionVariable, TblExpression<C>)>
    for DenseTblExpressionAssignment<C>
{
    type Error = KeyConflictError<TblExpressionVariable, TblExpression<C>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable, TblExpression<C>)>>(
        iter: T,
    ) -> Result<Self, Self::Error> {
        Ok(Self(DenseUsizeMap::try_from_iter_conflictless(
            iter.into_iter(),
        )?))
    }
}
impl<C: TblExpressionCompound> TryCombine for DenseTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable, TblExpression<C>>;
    fn try_combine<I: IntoIterator<Item = Self>>(
        assignments: I,
    ) -> Result<Self, Self::CombinationError> {
        Ok(Self(DenseUsizeMap::try_combine_conflictless(
            assignments.into_iter().map(|v| v.0),
        )?))
    }
}
impl<C: TblExpressionCompound> TblAssignmentHelper<C> for DenseTblExpressionAssignment<C> {
    fn get(&self, var: &TblExpressionVariable) -> Option<&TblExpression<C>> {
        self.0.get(var)
    }
    fn insert(
        &mut self,
        var: TblExpressionVariable,
        expr: TblExpression<C>,
    ) -> Result<(), KeyConflictError<TblExpressionVariable, TblExpression<C>>> {
        self.0.insert_conflictless(var, expr)
    }
}
impl<
    C: TblExpressionCompound + for<'a> From<&'a PostAssignmentCompound>,
    PreAssignmentUcompound: UnassignedTblExpressionCompound,
    PostAssignmentCompound: TblExpressionCompound
        + for<'a> From<&'a C>
        + for<'a> From<&'a PreAssignmentUcompound>
        + FromIterator<TblExpression<PostAssignmentCompound>>,
>
    PropositionalAssignment<
        UnassignedTblExpression<PreAssignmentUcompound>,
        TblExpression<PostAssignmentCompound>,
    > for DenseTblExpressionAssignment<C>
{
    type AssignmentError = TblAssignmentError;
    type ReverseAssignmentError = TblReverseAssignmentError<C>;
    fn assign(
        &self,
        pre_assignment_uprop: &UnassignedTblExpression<PreAssignmentUcompound>,
    ) -> Result<TblExpression<PostAssignmentCompound>, Self::AssignmentError> {
        self.assign_helper::<PreAssignmentUcompound, PostAssignmentCompound>(pre_assignment_uprop)
    }
    fn reverse_assign(
        pre_assignment_uprop: &UnassignedTblExpression<PreAssignmentUcompound>,
        post_assignment_prop: &TblExpression<PostAssignmentCompound>,
    ) -> Result<Self, Self::ReverseAssignmentError> {
        Self::reverse_assign_helper::<PreAssignmentUcompound, PostAssignmentCompound>(
            pre_assignment_uprop,
            post_assignment_prop,
        )
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DenseTblPartialExpressionAssignment<UC: UnassignedTblExpressionCompound>(
    pub DenseUsizeMap<TblExpressionVariable, UnassignedTblExpression<UC>>,
);
pub type DenseTblPartialPropositionAssignment<UC: TblExpressionCompound> =
    DenseTblPartialExpressionAssignment<UC>;
impl<UC: UnassignedTblExpressionCompound> DenseTblPartialExpressionAssignment<UC> {
    pub fn from_iter_unchecked<
        T: IntoIterator<Item = (TblExpressionVariable, UnassignedTblExpression<UC>)>,
    >(
        iter: T,
    ) -> Self {
        Self(DenseUsizeMap::from_iter(iter))
    }
}
impl<UC: UnassignedTblExpressionCompound> Default for DenseTblPartialExpressionAssignment<UC> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<UC: UnassignedTblExpressionCompound>
    TryFromIterator<(TblExpressionVariable, UnassignedTblExpression<UC>)>
    for DenseTblPartialExpressionAssignment<UC>
{
    type Error = KeyConflictError<TblExpressionVariable, UnassignedTblExpression<UC>>;
    fn try_from_iter<
        T: IntoIterator<Item = (TblExpressionVariable, UnassignedTblExpression<UC>)>,
    >(
        iter: T,
    ) -> Result<Self, Self::Error> {
        Ok(Self(DenseUsizeMap::try_from_iter_conflictless(
            iter.into_iter(),
        )?))
    }
}
impl<UC: UnassignedTblExpressionCompound> TryCombine for DenseTblPartialExpressionAssignment<UC> {
    type CombinationError = KeyConflictError<TblExpressionVariable, UnassignedTblExpression<UC>>;
    fn try_combine<I: IntoIterator<Item = Self>>(
        assignments: I,
    ) -> Result<Self, Self::CombinationError> {
        Ok(Self(DenseUsizeMap::try_combine_conflictless(
            assignments.into_iter().map(|v| v.0),
        )?))
    }
}

impl<Uc: UnassignedTblExpressionCompound> TblPartialAssignmentHelper<Uc>
    for DenseTblPartialExpressionAssignment<Uc>
{
    fn get(&self, var: &TblExpressionVariable) -> Option<&UnassignedTblExpression<Uc>> {
        self.0.get(var)
    }
    fn insert(
        &mut self,
        var: TblExpressionVariable,
        expr: UnassignedTblExpression<Uc>,
    ) -> Result<(), KeyConflictError<TblExpressionVariable, UnassignedTblExpression<Uc>>> {
        self.0.insert_conflictless(var, expr)
    }
}
impl<
    Uc: UnassignedTblExpressionCompound + for<'a> From<&'a PostAssignmentUcompound>,
    PreAssignmentUcompound: UnassignedTblExpressionCompound,
    PostAssignmentUcompound: UnassignedTblExpressionCompound
        + for<'a> From<&'a Uc>
        + for<'a> From<&'a PreAssignmentUcompound>
        + FromIterator<UnassignedTblExpression<PostAssignmentUcompound>>,
>
    PartialPropositionalAssignment<
        UnassignedTblExpression<PreAssignmentUcompound>,
        UnassignedTblExpression<PostAssignmentUcompound>,
    > for DenseTblPartialExpressionAssignment<Uc>
{
    type AssignmentError = TblPartialAssignmentError;
    type ReverseAssignmentError = TblPartialReverseAssignmentError<Uc>;
    fn assign(
        &self,
        unassigned: &UnassignedTblExpression<PreAssignmentUcompound>,
    ) -> Result<UnassignedTblExpression<PostAssignmentUcompound>, Self::AssignmentError> {
        self.partial_assign_helper(unassigned)
    }
    fn reverse_assign(
        unassigned: &UnassignedTblExpression<PreAssignmentUcompound>,
        assigned: &UnassignedTblExpression<PostAssignmentUcompound>,
    ) -> Result<Self, Self::ReverseAssignmentError> {
        Self::partial_reverse_assign_helper(unassigned, assigned)
    }
}

// TODO: consider performance implications of having a From<Vec> implementation for these to leverage From<Vec> of underlying DenseUsizeMap
