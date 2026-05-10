use proof_calculus::utils::collections::maps::dense_usize_map::DenseUsizeMap;

use crate::expressions::{
    assignments::full::constructor_implementations::dense::DenseTblExpressionAssignmentConstructor,
    paths::TblSubexpressionInExpressionPath, types::unassigned::variable::TblExpressionVariable,
};

pub struct DenseTblPartialExpressionAssignmentConstructor(
    pub DenseUsizeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>,
);
pub type DenseTblPartiallPropositionAssignmentConstructor = DenseTblExpressionAssignmentConstructor;

mod construction {
    use proof_calculus::utils::{
        collections::maps::{KeyConflictError, dense_usize_map::DenseUsizeMap},
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
        },
    };

    use crate::expressions::{
        assignments::partial::constructor_implementations::dense::DenseTblPartialExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

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
}

mod usage {
    use std::collections::BTreeMap;

    use itertools::Itertools;
    use proof_calculus::{
        propositions::assignments::PartialPropositionalAssignmentConstructor,
        utils::traits::map::{Map, MapWithTransformableValues},
    };

    use crate::{
        expressions::{
            assignments::{
                full::constructor_implementations::dense::DenseTblExpressionAssignmentConstructor,
                partial::{
                    constructor_implementations::dense::DenseTblPartialExpressionAssignmentConstructor,
                    implementations::{
                        btree::BTreeTblPartialPropositionAssignment,
                        dense::{
                            DenseTblPartialExpressionAssignment,
                            DenseTblPartialPropositionAssignment,
                        },
                    },
                },
            },
            types::unassigned::{
                UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
                subexpressions::ParentOfUnassignedSubexpressions,
            },
        },
        proof_calculus_derived::aliases::propositions::types::unassigned::UnassignedTblProposition,
    };

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
        > for DenseTblPartialExpressionAssignmentConstructor
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
                        match prop.get_subexpression(path) {
                            Ok(uexpr) => uexpr.into(),
                            Err(e) => return Err(e),
                        },
                    ))
                })
                .try_collect()?;
            Ok(BTreeTblPartialPropositionAssignment::from(values))
        }
    }
}
