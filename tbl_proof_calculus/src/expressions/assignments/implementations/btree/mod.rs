use std::collections::BTreeMap;

use proof_calculus::{
    propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment},
    utils::{
        collections::maps::KeyConflictError,
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
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

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct BTreeTblExpressionAssignment<C: TblExpressionCompound>(
    pub BTreeMap<TblExpressionVariable, TblExpression<C>>,
);
pub type BTreeTblPropositionAssignment<C: TblExpressionCompound> = BTreeTblExpressionAssignment<C>;

impl<C: TblExpressionCompound> Default for BTreeTblExpressionAssignment<C> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<C: TblExpressionCompound> From<BTreeMap<TblExpressionVariable, TblExpression<C>>>
    for BTreeTblExpressionAssignment<C>
{
    fn from(map: BTreeMap<TblExpressionVariable, TblExpression<C>>) -> Self {
        Self(BTreeMap::from(map))
    }
}
impl<C: TblExpressionCompound> Into<BTreeMap<TblExpressionVariable, TblExpression<C>>>
    for BTreeTblExpressionAssignment<C>
{
    fn into(self) -> BTreeMap<TblExpressionVariable, TblExpression<C>> {
        self.0.into()
    }
}
impl<C: TblExpressionCompound> TryFromIterator<(TblExpressionVariable, TblExpression<C>)>
    for BTreeTblExpressionAssignment<C>
{
    type Error = KeyConflictError<TblExpressionVariable, TblExpression<C>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable, TblExpression<C>)>>(
        iter: T,
    ) -> Result<Self, Self::Error> {
        Ok(Self(BTreeMap::try_from_iter_conflictless(
            iter.into_iter(),
        )?))
    }
}
impl<C: TblExpressionCompound> TryCombine for BTreeTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable, TblExpression<C>>;
    fn try_combine<I: IntoIterator<Item = Self>>(
        assignments: I,
    ) -> Result<Self, Self::CombinationError> {
        Ok(Self(BTreeMap::try_combine_conflictless(
            assignments.into_iter().map(|v| v.0),
        )?))
    }
}

impl<C: TblExpressionCompound> TblAssignmentHelper<C> for BTreeTblExpressionAssignment<C> {
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
    > for BTreeTblExpressionAssignment<C>
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

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct BTreeTblPartialExpressionAssignment<
    PostAssignmentUcompound: UnassignedTblExpressionCompound,
>(pub BTreeMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>);
pub type BTreeTblPartialPropositionAssignment<PostAssignmentUcompound: TblExpressionCompound> =
    BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>;

impl<PostAssignmentUcompound: UnassignedTblExpressionCompound> Default
    for BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>
{
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<PostAssignmentUcompound: UnassignedTblExpressionCompound>
    From<BTreeMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>>
    for BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>
{
    fn from(
        map: BTreeMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>,
    ) -> Self {
        Self(map)
    }
}
impl<PostAssignmentUcompound: UnassignedTblExpressionCompound>
    Into<BTreeMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>>
    for BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>
{
    fn into(
        self,
    ) -> BTreeMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>> {
        self.0
    }
}
impl<PostAssignmentUcompound: UnassignedTblExpressionCompound>
    TryFromIterator<(
        TblExpressionVariable,
        UnassignedTblExpression<PostAssignmentUcompound>,
    )> for BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>
{
    type Error =
        KeyConflictError<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>;
    fn try_from_iter<
        T: IntoIterator<
            Item = (
                TblExpressionVariable,
                UnassignedTblExpression<PostAssignmentUcompound>,
            ),
        >,
    >(
        iter: T,
    ) -> Result<Self, Self::Error> {
        Ok(Self(BTreeMap::try_from_iter_conflictless(
            iter.into_iter(),
        )?))
    }
}
impl<PostAssignmentUcompound: UnassignedTblExpressionCompound> TryCombine
    for BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>
{
    type CombinationError =
        KeyConflictError<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>;
    fn try_combine<I: IntoIterator<Item = Self>>(
        assignments: I,
    ) -> Result<Self, Self::CombinationError> {
        Ok(Self(BTreeMap::try_combine_conflictless(
            assignments.into_iter().map(|v| v.0),
        )?))
    }
}

impl<Uc: UnassignedTblExpressionCompound> TblPartialAssignmentHelper<Uc>
    for BTreeTblPartialExpressionAssignment<Uc>
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
    > for BTreeTblPartialExpressionAssignment<Uc>
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
