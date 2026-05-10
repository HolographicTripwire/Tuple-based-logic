use proof_calculus::utils::collections::maps::dense_usize_map::DenseUsizeMap;

use crate::expressions::types::{
    assigned::compound::TblExpressionCompound,
    unassigned::{
        UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
        variable::TblExpressionVariable,
    },
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DenseTblPartialExpressionAssignment<UC: UnassignedTblExpressionCompound>(
    pub DenseUsizeMap<TblExpressionVariable, UnassignedTblExpression<UC>>,
);
pub type DenseTblPartialPropositionAssignment<UC: TblExpressionCompound> =
    DenseTblPartialExpressionAssignment<UC>;

mod construction {
    use proof_calculus::utils::{
        collections::maps::{KeyConflictError, dense_usize_map::DenseUsizeMap},
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
        },
    };

    use crate::expressions::{
        assignments::partial::implementations::dense::DenseTblPartialExpressionAssignment,
        types::unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    };

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
        type CombinationError =
            KeyConflictError<TblExpressionVariable, UnassignedTblExpression<UC>>;
        fn try_combine<I: IntoIterator<Item = Self>>(
            assignments: I,
        ) -> Result<Self, Self::CombinationError> {
            Ok(Self(DenseUsizeMap::try_combine_conflictless(
                assignments.into_iter().map(|v| v.0),
            )?))
        }
    }
}
mod map_implementation {
    use proof_calculus::utils::{
        collections::maps::{KeyConflictError, dense_usize_map::DenseUsizeMap},
        traits::map::{Map, MapWithoutConflicts},
    };

    use crate::expressions::{
        assignments::partial::implementations::dense::DenseTblPartialExpressionAssignment,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    };
    impl<Uc: UnassignedTblExpressionCompound>
        Map<TblExpressionVariable, UnassignedTblExpression<Uc>>
        for DenseTblPartialExpressionAssignment<Uc>
    {
        fn get(&self, key: &TblExpressionVariable) -> Option<&UnassignedTblExpression<Uc>> {
            self.0.get(key)
        }
        fn get_mut(
            &mut self,
            key: &TblExpressionVariable,
        ) -> Option<&mut UnassignedTblExpression<Uc>> {
            self.0.get_mut(key)
        }
        fn insert(
            &mut self,
            key: TblExpressionVariable,
            value: UnassignedTblExpression<Uc>,
        ) -> Option<UnassignedTblExpression<Uc>> {
            self.0.insert(key, value)
        }
        fn remove(&mut self, key: &TblExpressionVariable) -> Option<UnassignedTblExpression<Uc>> {
            self.0.remove(key)
        }
        fn iter<'a>(
            &'a self,
        ) -> impl Iterator<Item = (&'a TblExpressionVariable, &'a UnassignedTblExpression<Uc>)>
        where
            TblExpressionVariable: 'a,
            TblSubexpressionInExpressionPath: 'a,
        {
            self.0.iter()
        }
    }
    impl<Uc: UnassignedTblExpressionCompound>
        MapWithoutConflicts<TblExpressionVariable, UnassignedTblExpression<Uc>>
        for DenseTblPartialExpressionAssignment<Uc>
    {
        fn insert_conflictless(
            &mut self,
            key: TblExpressionVariable,
            value: UnassignedTblExpression<Uc>,
        ) -> Result<(), KeyConflictError<TblExpressionVariable, UnassignedTblExpression<Uc>>>
        {
            self.0.insert_conflictless(key, value)
        }
        fn try_combine_conflictless<I: IntoIterator<Item = Self>>(
            maps: I,
        ) -> Result<Self, KeyConflictError<TblExpressionVariable, UnassignedTblExpression<Uc>>>
        {
            Ok(Self(DenseUsizeMap::try_combine_conflictless(
                maps.into_iter().map(|v| v.0),
            )?))
        }
        fn try_from_iter_conflictless<
            T: IntoIterator<Item = (TblExpressionVariable, UnassignedTblExpression<Uc>)>,
        >(
            iter: T,
        ) -> Result<
            Self,
            proof_calculus::utils::collections::maps::KeyConflictError<
                TblExpressionVariable,
                UnassignedTblExpression<Uc>,
            >,
        > {
            Ok(Self(DenseUsizeMap::try_from_iter_conflictless(iter)?))
        }
    }
}
mod usage {
    use proof_calculus::propositions::assignments::PartialPropositionalAssignment;

    use crate::expressions::{
        assignments::partial::{
            errors::{
                assignment::TblPartialAssignmentError,
                reverse_assignment::TblPartialReverseAssignmentError,
            },
            implementations::{
                TblPartialAssignmentHelper, dense::DenseTblPartialExpressionAssignment,
            },
        },
        types::unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound},
    };

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
        ) -> Result<UnassignedTblExpression<PostAssignmentUcompound>, Self::AssignmentError>
        {
            self.partial_assign_helper(unassigned)
        }
        fn reverse_assign(
            unassigned: &UnassignedTblExpression<PreAssignmentUcompound>,
            assigned: &UnassignedTblExpression<PostAssignmentUcompound>,
        ) -> Result<Self, Self::ReverseAssignmentError> {
            Self::partial_reverse_assign_helper(unassigned, assigned)
        }
    }
}
// TODO: consider performance implications of having a From<Vec> implementation for these to leverage From<Vec> of underlying DenseUsizeMap
