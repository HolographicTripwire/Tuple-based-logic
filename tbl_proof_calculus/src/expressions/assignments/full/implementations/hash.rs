use std::collections::HashMap;

use proof_calculus::propositions::assignments::PropositionalAssignment;

use crate::expressions::{
    assignments::full::{
        errors::{assignment::TblAssignmentError, reverse_assignment::TblReverseAssignmentError},
        implementations::TblAssignmentHelper,
    },
    types::{
        assigned::{TblExpression, compound::TblExpressionCompound},
        unassigned::{
            UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
            variable::TblExpressionVariable,
        },
    },
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HashTblExpressionAssignment<C: TblExpressionCompound>(
    pub HashMap<TblExpressionVariable, TblExpression<C>>,
);
pub type HashTblPropositionAssignment<C: TblExpressionCompound> = HashTblExpressionAssignment<C>;
mod construction {
    use std::collections::HashMap;

    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
        },
    };

    use crate::expressions::{
        assignments::full::implementations::hash::HashTblExpressionAssignment,
        types::{
            assigned::{TblExpression, compound::TblExpressionCompound},
            unassigned::variable::TblExpressionVariable,
        },
    };

    impl<C: TblExpressionCompound> Default for HashTblExpressionAssignment<C> {
        fn default() -> Self {
            Self(Default::default())
        }
    }
    impl<C: TblExpressionCompound> TryFromIterator<(TblExpressionVariable, TblExpression<C>)>
        for HashTblExpressionAssignment<C>
    {
        type Error = KeyConflictError<TblExpressionVariable, TblExpression<C>>;
        fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable, TblExpression<C>)>>(
            iter: T,
        ) -> Result<Self, Self::Error> {
            Ok(Self(HashMap::try_from_iter_conflictless(iter.into_iter())?))
        }
    }
    impl<C: TblExpressionCompound> TryCombine for HashTblExpressionAssignment<C> {
        type CombinationError = KeyConflictError<TblExpressionVariable, TblExpression<C>>;
        fn try_combine<I: IntoIterator<Item = Self>>(
            assignments: I,
        ) -> Result<Self, Self::CombinationError> {
            Ok(Self(HashMap::try_combine_conflictless(
                assignments.into_iter().map(|v| v.0),
            )?))
        }
    }
    impl<C: TblExpressionCompound> From<HashMap<TblExpressionVariable, TblExpression<C>>>
        for HashTblExpressionAssignment<C>
    {
        fn from(map: HashMap<TblExpressionVariable, TblExpression<C>>) -> Self {
            Self(HashMap::from(map))
        }
    }
}
mod deconstruction {
    use std::collections::HashMap;

    use crate::expressions::{
        assignments::full::implementations::hash::HashTblExpressionAssignment,
        types::{
            assigned::{TblExpression, compound::TblExpressionCompound},
            unassigned::variable::TblExpressionVariable,
        },
    };

    impl<C: TblExpressionCompound> Into<HashMap<TblExpressionVariable, TblExpression<C>>>
        for HashTblExpressionAssignment<C>
    {
        fn into(self) -> HashMap<TblExpressionVariable, TblExpression<C>> {
            self.0.into()
        }
    }
}
mod map_implementation {
    use std::collections::HashMap;

    use proof_calculus::utils::traits::map::{Map, MapWithoutConflicts};

    use crate::expressions::{
        assignments::full::implementations::hash::HashTblExpressionAssignment,
        paths::TblSubexpressionInExpressionPath,
        types::{
            assigned::{TblExpression, compound::TblExpressionCompound},
            unassigned::variable::TblExpressionVariable,
        },
    };
    impl<C: TblExpressionCompound> Map<TblExpressionVariable, TblExpression<C>>
        for HashTblExpressionAssignment<C>
    {
        fn get(&self, key: &TblExpressionVariable) -> Option<&TblExpression<C>> {
            self.0.get(key)
        }
        fn get_mut(&mut self, key: &TblExpressionVariable) -> Option<&mut TblExpression<C>> {
            self.0.get_mut(key)
        }
        fn insert(
            &mut self,
            key: TblExpressionVariable,
            value: TblExpression<C>,
        ) -> Option<TblExpression<C>> {
            self.0.insert(key, value)
        }
        fn remove(&mut self, key: &TblExpressionVariable) -> Option<TblExpression<C>> {
            self.0.remove(key)
        }
        fn iter<'a>(
            &'a self,
        ) -> impl Iterator<Item = (&'a TblExpressionVariable, &'a TblExpression<C>)>
        where
            TblExpressionVariable: 'a,
            TblSubexpressionInExpressionPath: 'a,
        {
            self.0.iter()
        }
    }
    impl<C: TblExpressionCompound> MapWithoutConflicts<TblExpressionVariable, TblExpression<C>>
        for HashTblExpressionAssignment<C>
    {
        fn insert_conflictless(
            &mut self,
            key: TblExpressionVariable,
            value: TblExpression<C>,
        ) -> Result<
            (),
            proof_calculus::utils::collections::maps::KeyConflictError<
                TblExpressionVariable,
                TblExpression<C>,
            >,
        >
        where
            TblSubexpressionInExpressionPath: PartialEq<TblSubexpressionInExpressionPath>,
        {
            self.0.insert_conflictless(key, value)
        }
        fn try_combine_conflictless<I: IntoIterator<Item = Self>>(
            maps: I,
        ) -> Result<
            Self,
            proof_calculus::utils::collections::maps::KeyConflictError<
                TblExpressionVariable,
                TblExpression<C>,
            >,
        > {
            Ok(Self(HashMap::try_combine_conflictless(
                maps.into_iter().map(|v| v.0),
            )?))
        }
        fn try_from_iter_conflictless<
            T: IntoIterator<Item = (TblExpressionVariable, TblExpression<C>)>,
        >(
            iter: T,
        ) -> Result<
            Self,
            proof_calculus::utils::collections::maps::KeyConflictError<
                TblExpressionVariable,
                TblExpression<C>,
            >,
        > {
            Ok(Self(HashMap::try_from_iter_conflictless(iter)?))
        }
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
    > for HashTblExpressionAssignment<C>
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
