use proof_calculus::utils::collections::maps::dense_usize_map::DenseUsizeMap;

use crate::expressions::{
    paths::TblSubexpressionInExpressionPath, types::unassigned::variable::TblExpressionVariable,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DenseTblExpressionAssignmentConstructor(
    pub DenseUsizeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>,
);
pub type DenseTblPropositionAssignmentConstructor = DenseTblExpressionAssignmentConstructor;

mod construction {
    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
        },
    };

    use crate::expressions::{
        assignments::full::constructor_implementations::dense::DenseTblExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

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
            Self::try_from_iter_conflictless(iter)
        }
    }
    impl TryCombine for DenseTblExpressionAssignmentConstructor {
        type CombinationError =
            KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
        fn try_combine<I: IntoIterator<Item = Self>>(
            assignments: I,
        ) -> Result<Self, Self::CombinationError> {
            Self::try_combine_conflictless(assignments)
        }
    }
}

mod map_implementation {
    use proof_calculus::utils::{
        collections::maps::dense_usize_map::DenseUsizeMap,
        traits::map::{Map, MapWithoutConflicts},
    };

    use crate::expressions::{
        assignments::full::constructor_implementations::dense::DenseTblExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };
    impl Map<TblExpressionVariable, TblSubexpressionInExpressionPath>
        for DenseTblExpressionAssignmentConstructor
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
        for DenseTblExpressionAssignmentConstructor
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
            Ok(Self(DenseUsizeMap::try_combine_conflictless(
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
            Ok(Self(DenseUsizeMap::try_from_iter_conflictless(iter)?))
        }
    }
}

mod usage {
    use proof_calculus::{
        propositions::assignments::PropositionalAssignmentConstructor,
        utils::traits::map::MapWithTransformableValues,
    };

    use crate::{
        expressions::{
            assignments::full::{
                constructor_implementations::dense::DenseTblExpressionAssignmentConstructor,
                implementations::dense::{
                    DenseTblExpressionAssignment, DenseTblPropositionAssignment,
                },
            },
            types::{
                assigned::{
                    TblExpression, compound::TblExpressionCompound,
                    subexpressions::ParentOfSubexpressions,
                },
                unassigned::{
                    compound::UnassignedTblExpressionCompound, variable::TblExpressionVariable,
                },
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
}
