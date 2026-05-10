use std::collections::HashMap;

use crate::expressions::{
    paths::TblSubexpressionInExpressionPath, types::unassigned::variable::TblExpressionVariable,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HashTblExpressionAssignmentConstructor(
    HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath>,
);
pub type HashTblPropositionAssignmentConstructor = HashTblExpressionAssignmentConstructor;

mod construction {
    use std::collections::HashMap;

    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
        },
    };

    use crate::expressions::{
        assignments::full::constructor_implementations::hash::HashTblExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

    impl Default for HashTblExpressionAssignmentConstructor {
        fn default() -> Self {
            Self(Default::default())
        }
    }
    impl From<HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath>>
        for HashTblExpressionAssignmentConstructor
    {
        fn from(map: HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath>) -> Self {
            Self(HashMap::from(map))
        }
    }
    impl TryFromIterator<(TblExpressionVariable, TblSubexpressionInExpressionPath)>
        for HashTblExpressionAssignmentConstructor
    {
        type Error = KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
        fn try_from_iter<
            T: IntoIterator<Item = (TblExpressionVariable, TblSubexpressionInExpressionPath)>,
        >(
            iter: T,
        ) -> Result<Self, Self::Error> {
            Self::try_from_iter_conflictless(iter)
        }
    }
    impl TryCombine for HashTblExpressionAssignmentConstructor {
        type CombinationError =
            KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
        fn try_combine<I: IntoIterator<Item = Self>>(
            assignments: I,
        ) -> Result<Self, Self::CombinationError> {
            Self::try_combine_conflictless(assignments)
        }
    }
}
mod deconstruction {
    use std::collections::HashMap;

    use crate::expressions::{
        assignments::full::constructor_implementations::hash::HashTblExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

    impl Into<HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath>>
        for HashTblExpressionAssignmentConstructor
    {
        fn into(self) -> HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath> {
            self.0.into()
        }
    }
}
mod map_implementation {
    use std::collections::HashMap;

    use proof_calculus::utils::traits::map::{Map, MapWithoutConflicts};

    use crate::expressions::{
        assignments::full::constructor_implementations::hash::HashTblExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };
    impl Map<TblExpressionVariable, TblSubexpressionInExpressionPath>
        for HashTblExpressionAssignmentConstructor
    {
        fn get(&self, key: &TblExpressionVariable) -> Option<&TblSubexpressionInExpressionPath> {
            self.0.get(key)
        }
        fn get_mut(
            &mut self,
            key: &TblExpressionVariable,
        ) -> Option<&mut TblSubexpressionInExpressionPath> {
            self.0.get_mut(key)
        }
        fn insert(
            &mut self,
            key: TblExpressionVariable,
            value: TblSubexpressionInExpressionPath,
        ) -> Option<TblSubexpressionInExpressionPath> {
            self.0.insert(key, value)
        }
        fn remove(
            &mut self,
            key: &TblExpressionVariable,
        ) -> Option<TblSubexpressionInExpressionPath> {
            self.0.remove(key)
        }
        fn iter<'a>(
            &'a self,
        ) -> impl Iterator<
            Item = (
                &'a TblExpressionVariable,
                &'a TblSubexpressionInExpressionPath,
            ),
        >
        where
            TblExpressionVariable: 'a,
            TblSubexpressionInExpressionPath: 'a,
        {
            self.0.iter()
        }
    }
    impl MapWithoutConflicts<TblExpressionVariable, TblSubexpressionInExpressionPath>
        for HashTblExpressionAssignmentConstructor
    {
        fn insert_conflictless(
            &mut self,
            key: TblExpressionVariable,
            value: TblSubexpressionInExpressionPath,
        ) -> Result<
            (),
            proof_calculus::utils::collections::maps::KeyConflictError<
                TblExpressionVariable,
                TblSubexpressionInExpressionPath,
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
                TblSubexpressionInExpressionPath,
            >,
        > {
            Ok(Self(HashMap::try_combine_conflictless(
                maps.into_iter().map(|v| v.0),
            )?))
        }
        fn try_from_iter_conflictless<
            T: IntoIterator<Item = (TblExpressionVariable, TblSubexpressionInExpressionPath)>,
        >(
            iter: T,
        ) -> Result<
            Self,
            proof_calculus::utils::collections::maps::KeyConflictError<
                TblExpressionVariable,
                TblSubexpressionInExpressionPath,
            >,
        > {
            Ok(Self(HashMap::try_from_iter_conflictless(iter)?))
        }
    }
}

mod usage {
    use std::collections::HashMap;

    use itertools::Itertools;
    use proof_calculus::propositions::assignments::PropositionalAssignmentConstructor;

    use crate::{
        expressions::{
            assignments::full::{
                constructor_implementations::hash::HashTblExpressionAssignmentConstructor,
                implementations::hash::{
                    HashTblExpressionAssignment, HashTblPropositionAssignment,
                },
            },
            types::{
                assigned::{
                    TblExpression, compound::TblExpressionCompound,
                    subexpressions::ParentOfSubexpressions,
                },
                unassigned::compound::UnassignedTblExpressionCompound,
            },
        },
        proof_calculus_derived::aliases::propositions::types::{
            assigned::TblProposition, unassigned::UnassignedTblProposition,
        },
    };

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
            HashTblPropositionAssignment<C>,
        > for HashTblExpressionAssignmentConstructor
    {
        type Error = ();
        fn try_construct(
            &self,
            prop: &TblProposition<PostAssignmentCompound>,
        ) -> Result<HashTblExpressionAssignment<C>, ()> {
            let inner: HashMap<_, _> = self
                .0
                .iter()
                .map(|(variable, path)| Ok((*variable, prop.get_subexpression(&path)?.into())))
                .try_collect()
                .map_err(|_: ()| ())?;
            Ok(HashTblExpressionAssignment(inner.into()))
        }
    }
}
