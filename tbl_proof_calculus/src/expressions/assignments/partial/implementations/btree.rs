use std::collections::BTreeMap;

use crate::expressions::types::{
    assigned::compound::TblExpressionCompound,
    unassigned::{
        UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
        variable::TblExpressionVariable,
    },
};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct BTreeTblPartialExpressionAssignment<
    PostAssignmentUcompound: UnassignedTblExpressionCompound,
>(pub BTreeMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>);
pub type BTreeTblPartialPropositionAssignment<PostAssignmentUcompound: TblExpressionCompound> =
    BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>;

mod construction {
    use std::collections::BTreeMap;

    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
        },
    };

    use crate::expressions::{
        assignments::partial::implementations::btree::BTreeTblPartialExpressionAssignment,
        types::unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    };

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
        TryFromIterator<(
            TblExpressionVariable,
            UnassignedTblExpression<PostAssignmentUcompound>,
        )> for BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>
    {
        type Error = KeyConflictError<
            TblExpressionVariable,
            UnassignedTblExpression<PostAssignmentUcompound>,
        >;
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
        type CombinationError = KeyConflictError<
            TblExpressionVariable,
            UnassignedTblExpression<PostAssignmentUcompound>,
        >;
        fn try_combine<I: IntoIterator<Item = Self>>(
            assignments: I,
        ) -> Result<Self, Self::CombinationError> {
            Ok(Self(BTreeMap::try_combine_conflictless(
                assignments.into_iter().map(|v| v.0),
            )?))
        }
    }
}
mod deconstruction {
    use std::collections::BTreeMap;

    use crate::expressions::{
        assignments::partial::implementations::btree::BTreeTblPartialExpressionAssignment,
        types::unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    };

    impl<PostAssignmentUcompound: UnassignedTblExpressionCompound>
        Into<BTreeMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>>
        for BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>
    {
        fn into(
            self,
        ) -> BTreeMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>
        {
            self.0
        }
    }
}

mod map_implementation {
    use std::collections::BTreeMap;

    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::map::{Map, MapWithoutConflicts},
    };

    use crate::expressions::{
        assignments::partial::implementations::btree::BTreeTblPartialExpressionAssignment,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    };
    impl<Uc: UnassignedTblExpressionCompound>
        Map<TblExpressionVariable, UnassignedTblExpression<Uc>>
        for BTreeTblPartialExpressionAssignment<Uc>
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
        for BTreeTblPartialExpressionAssignment<Uc>
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
            Ok(Self(BTreeMap::try_combine_conflictless(
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
            Ok(Self(BTreeMap::try_from_iter_conflictless(iter)?))
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
                TblPartialAssignmentHelper, btree::BTreeTblPartialExpressionAssignment,
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
        > for BTreeTblPartialExpressionAssignment<Uc>
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
