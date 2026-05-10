use std::collections::BTreeMap;

use crate::expressions::{
    paths::TblSubexpressionInExpressionPath, types::unassigned::variable::TblExpressionVariable,
};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct BTreeTblExpressionAssignmentConstructor(
    pub BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>,
);
pub type BTreeTblPropositionAssignmentConstructor = BTreeTblExpressionAssignmentConstructor;

mod construction {
    use std::collections::BTreeMap;

    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::{map::MapWithoutConflicts, try_from_iter::TryFromIterator},
    };

    use crate::expressions::{
        assignments::full::constructor_implementations::btree::BTreeTblExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

    impl Default for BTreeTblExpressionAssignmentConstructor {
        fn default() -> Self {
            Self(Default::default())
        }
    }
    impl From<BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>>
        for BTreeTblExpressionAssignmentConstructor
    {
        fn from(map: BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>) -> Self {
            Self(BTreeMap::from(map))
        }
    }
    impl TryFromIterator<(TblExpressionVariable, TblSubexpressionInExpressionPath)>
        for BTreeTblExpressionAssignmentConstructor
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
}
mod deconstruction {
    use std::collections::BTreeMap;

    use crate::expressions::{
        assignments::full::constructor_implementations::btree::BTreeTblExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

    impl Into<BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>>
        for BTreeTblExpressionAssignmentConstructor
    {
        fn into(self) -> BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath> {
            self.0.into()
        }
    }
}

mod map_implementation {
    use std::collections::BTreeMap;

    use proof_calculus::utils::traits::map::{Map, MapWithoutConflicts};

    use crate::expressions::{
        assignments::full::constructor_implementations::btree::BTreeTblExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };
    impl Map<TblExpressionVariable, TblSubexpressionInExpressionPath>
        for BTreeTblExpressionAssignmentConstructor
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
        for BTreeTblExpressionAssignmentConstructor
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
            Ok(Self(BTreeMap::try_combine_conflictless(
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
            Ok(Self(BTreeMap::try_from_iter_conflictless(iter)?))
        }
    }
}

mod usage {
    use std::collections::BTreeMap;

    use itertools::Itertools;
    use proof_calculus::propositions::assignments::PropositionalAssignmentConstructor;

    use crate::{
        expressions::{
            assignments::full::{
                constructor_implementations::btree::BTreeTblExpressionAssignmentConstructor,
                implementations::btree::{
                    BTreeTblExpressionAssignment, BTreeTblPropositionAssignment,
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
            BTreeTblPropositionAssignment<C>,
        > for BTreeTblExpressionAssignmentConstructor
    {
        type Error = ();
        fn try_construct(
            &self,
            prop: &TblProposition<PostAssignmentCompound>,
        ) -> Result<BTreeTblPropositionAssignment<C>, ()> {
            let inner: BTreeMap<_, _> = self
                .0
                .iter()
                .map(|(variable, path)| Ok((*variable, prop.get_subexpression(&path)?.into())))
                .try_collect()
                .map_err(|_: ()| ())?;
            Ok(BTreeTblExpressionAssignment(inner.into()))
        }
    }
}
