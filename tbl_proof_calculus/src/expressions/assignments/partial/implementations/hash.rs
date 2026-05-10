use std::collections::HashMap;

use crate::expressions::types::{
    assigned::compound::TblExpressionCompound,
    unassigned::{
        UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
        variable::TblExpressionVariable,
    },
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HashTblPartialExpressionAssignment<
    PostAssignmentUcompound: UnassignedTblExpressionCompound,
>(pub HashMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>);
pub type HashTblPartialPropositionAssignment<PostAssignmentUcompound: TblExpressionCompound> =
    HashTblPartialExpressionAssignment<PostAssignmentUcompound>;

mod construction {
    use std::collections::HashMap;

    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
        },
    };

    use crate::expressions::{
        assignments::partial::implementations::hash::HashTblPartialExpressionAssignment,
        types::unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    };

    impl<PostAssignmentUcompound: UnassignedTblExpressionCompound> Default
        for HashTblPartialExpressionAssignment<PostAssignmentUcompound>
    {
        fn default() -> Self {
            Self(Default::default())
        }
    }
    impl<PostAssignmentUcompound: UnassignedTblExpressionCompound>
        From<HashMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>>
        for HashTblPartialExpressionAssignment<PostAssignmentUcompound>
    {
        fn from(
            map: HashMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>,
        ) -> Self {
            Self(map)
        }
    }
    impl<PostAssignmentUcompound: UnassignedTblExpressionCompound>
        TryFromIterator<(
            TblExpressionVariable,
            UnassignedTblExpression<PostAssignmentUcompound>,
        )> for HashTblPartialExpressionAssignment<PostAssignmentUcompound>
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
            Ok(Self(HashMap::try_from_iter_conflictless(iter.into_iter())?))
        }
    }
    impl<PostAssignmentUcompound: UnassignedTblExpressionCompound> TryCombine
        for HashTblPartialExpressionAssignment<PostAssignmentUcompound>
    {
        type CombinationError = KeyConflictError<
            TblExpressionVariable,
            UnassignedTblExpression<PostAssignmentUcompound>,
        >;
        fn try_combine<I: IntoIterator<Item = Self>>(
            assignments: I,
        ) -> Result<Self, Self::CombinationError> {
            Ok(Self(HashMap::try_combine_conflictless(
                assignments.into_iter().map(|v| v.0),
            )?))
        }
    }
}
mod deconstruction {
    use std::collections::HashMap;

    use crate::expressions::{
        assignments::partial::implementations::hash::HashTblPartialExpressionAssignment,
        types::unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    };

    impl<PostAssignmentUcompound: UnassignedTblExpressionCompound>
        Into<HashMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>>
        for HashTblPartialExpressionAssignment<PostAssignmentUcompound>
    {
        fn into(
            self,
        ) -> HashMap<TblExpressionVariable, UnassignedTblExpression<PostAssignmentUcompound>>
        {
            self.0
        }
    }
}

mod map_implementation {
    use std::collections::HashMap;

    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::map::{Map, MapWithoutConflicts},
    };

    use crate::expressions::{
        assignments::partial::implementations::hash::HashTblPartialExpressionAssignment,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    };
    impl<Uc: UnassignedTblExpressionCompound>
        Map<TblExpressionVariable, UnassignedTblExpression<Uc>>
        for HashTblPartialExpressionAssignment<Uc>
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
        for HashTblPartialExpressionAssignment<Uc>
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
            Ok(Self(HashMap::try_combine_conflictless(
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
            Ok(Self(HashMap::try_from_iter_conflictless(iter)?))
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
                TblPartialAssignmentHelper, hash::HashTblPartialExpressionAssignment,
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
        > for HashTblPartialExpressionAssignment<Uc>
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
