use std::collections::{BTreeMap};

use itertools::Itertools;
use proof_calculus::{
    propositions::assignments::{
        PartialPropositionalAssignmentConstructor, PropositionalAssignmentConstructor,
    },
    utils::{
        collections::maps::{KeyConflictError, dense_usize_map::DenseUsizeMap},
        traits::{
            combinable::TryCombine,
            map::{Map, MapWithTransformableValues, MapWithoutConflicts},
            try_from_iter::TryFromIterator,
        },
    },
};

use crate::{
    expressions::{
        assignments::implementations::{
            btree::BTreeTblPartialPropositionAssignment,
            dense::{
                DenseTblExpressionAssignment, DenseTblPartialExpressionAssignment,
                DenseTblPartialPropositionAssignment, DenseTblPropositionAssignment,
            },
        },
        paths::TblSubexpressionInExpressionPath,
        types::{
            assigned::{
                TblExpression, compound::TblExpressionCompound,
                subexpressions::ParentOfSubexpressions,
            },
            unassigned::{
                UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
                subexpressions::ParentOfUnassignedSubexpressions, variable::TblExpressionVariable,
            },
        },
    },
    proof_calculus_derived::aliases::propositions::types::{
        assigned::TblProposition, unassigned::UnassignedTblProposition,
    },
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DenseTblExpressionAssignmentConstructor(
    DenseUsizeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>,
);
pub type DenseTblPropositionAssignmentConstructor = DenseTblExpressionAssignmentConstructor;

impl Default for DenseTblExpressionAssignmentConstructor {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl TryFromIterator<(TblExpressionVariable, TblSubexpressionInExpressionPath)>
    for DenseTblExpressionAssignmentConstructor
{
    type Error = KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
    fn try_from_iter<
        T: IntoIterator<Item = (TblExpressionVariable, TblSubexpressionInExpressionPath)>,
    >(
        iter: T,
    ) -> Result<Self, Self::Error> {
        Ok(Self(DenseUsizeMap::try_from_iter_conflictless(iter)?))
    }
}
impl TryCombine for DenseTblExpressionAssignmentConstructor {
    type CombinationError =
        KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
    fn try_combine<I: IntoIterator<Item = Self>>(
        assignments: I,
    ) -> Result<Self, Self::CombinationError> {
        Ok(Self(DenseUsizeMap::try_combine_conflictless(
            assignments.into_iter().map(|v| v.0),
        )?))
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
    PropositionalAssignmentConstructor<
        UnassignedTblProposition<PreAssignmentUcompound>,
        TblProposition<PostAssignmentCompound>,
        DenseTblPropositionAssignment<C>,
    > for DenseTblExpressionAssignmentConstructor
{
    type Error = ();
    fn try_construct(
        &self,
        prop: &TblProposition<PostAssignmentCompound>,
    ) -> Result<DenseTblPropositionAssignment<C>, ()> {
        let inner = self
            .0
            .try_with_values_transformed(|path| Ok(prop.get_subexpression(path)?.into()))
            .map_err(|(_, _): (TblExpressionVariable, ())| ());
        Ok(DenseTblExpressionAssignment(inner?))
    }
}

pub struct DenseTblPartialExpressionAssignmentConstructor(
    DenseUsizeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>,
);
pub type DenseTblPartiallPropositionAssignmentConstructor = DenseTblExpressionAssignmentConstructor;

impl Default for DenseTblPartialExpressionAssignmentConstructor {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl TryFromIterator<(TblExpressionVariable, TblSubexpressionInExpressionPath)>
    for DenseTblPartialExpressionAssignmentConstructor
{
    type Error = KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
    fn try_from_iter<
        T: IntoIterator<Item = (TblExpressionVariable, TblSubexpressionInExpressionPath)>,
    >(
        iter: T,
    ) -> Result<Self, Self::Error> {
        Ok(Self(DenseUsizeMap::try_from_iter_conflictless(
            iter.into_iter(),
        )?))
    }
}
impl TryCombine for DenseTblPartialExpressionAssignmentConstructor {
    type CombinationError =
        KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
    fn try_combine<I: IntoIterator<Item = Self>>(
        assignments: I,
    ) -> Result<Self, Self::CombinationError> {
        Ok(Self(DenseUsizeMap::try_combine_conflictless(
            assignments.into_iter().map(|v| v.0),
        )?))
    }
}

impl<
    PreAssignmentUcompound: UnassignedTblExpressionCompound,
    PostAssignmentUcompound: UnassignedTblExpressionCompound
        + for<'a> From<&'a PreAssignmentUcompound>
        + for<'a> From<&'a PostAssignmentUcompound>
        + FromIterator<UnassignedTblExpression<PostAssignmentUcompound>>,
>
    PartialPropositionalAssignmentConstructor<
        UnassignedTblProposition<PreAssignmentUcompound>,
        UnassignedTblProposition<PostAssignmentUcompound>,
        DenseTblPartialPropositionAssignment<PostAssignmentUcompound>,
    > for DenseTblExpressionAssignmentConstructor
{
    type Error = ();
    fn try_construct(
        &self,
        prop: &UnassignedTblProposition<PostAssignmentUcompound>,
    ) -> Result<DenseTblPartialPropositionAssignment<PostAssignmentUcompound>, ()> {
        let inner = self
            .0
            .try_with_values_transformed(|path| match prop.get_subexpression(path) {
                Ok(uexpr) => Ok(uexpr.into()),
                Err(err) => Err(err),
            })
            .map_err(|err| err.1)?;
        Ok(DenseTblPartialExpressionAssignment(inner))
    }
}
impl<
    PreAssignmentUcompound: UnassignedTblExpressionCompound,
    PostAssignmentUcompound: UnassignedTblExpressionCompound
        + for<'a> From<&'a PreAssignmentUcompound>
        + for<'a> From<&'a PostAssignmentUcompound>
        + FromIterator<UnassignedTblExpression<PostAssignmentUcompound>>,
>
    PartialPropositionalAssignmentConstructor<
        UnassignedTblProposition<PreAssignmentUcompound>,
        UnassignedTblProposition<PostAssignmentUcompound>,
        BTreeTblPartialPropositionAssignment<PostAssignmentUcompound>,
    > for DenseTblExpressionAssignmentConstructor
{
    type Error = ();
    fn try_construct(
        &self,
        prop: &UnassignedTblProposition<PostAssignmentUcompound>,
    ) -> Result<BTreeTblPartialPropositionAssignment<PostAssignmentUcompound>, ()> {
        let values: BTreeMap<_, _> = self
            .0
            .iter()
            .map(|(variable, path)| {
                Ok((
                    *variable,
                    match prop.get_subexpression_owned(path) {
                        Ok(uexpr) => uexpr.into(),
                        Err(e) => return Err(e),
                    },
                ))
            })
            .try_collect()?;
        Ok(BTreeTblPartialPropositionAssignment::from(values))
    }
}
