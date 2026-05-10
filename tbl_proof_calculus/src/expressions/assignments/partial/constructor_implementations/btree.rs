use std::collections::BTreeMap;

use crate::expressions::{
    assignments::full::constructor_implementations::btree::BTreeTblExpressionAssignmentConstructor,
    paths::TblSubexpressionInExpressionPath, types::unassigned::variable::TblExpressionVariable,
};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct BTreeTblPartialExpressionAssignmentConstructor(
    BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>,
);
pub type BTreeTblPartiallPropositionAssignmentConstructor = BTreeTblExpressionAssignmentConstructor;

mod construction {
    use std::collections::BTreeMap;

    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
        },
    };

    use crate::expressions::{
        assignments::partial::constructor_implementations::btree::BTreeTblPartialExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

    impl Default for BTreeTblPartialExpressionAssignmentConstructor {
        fn default() -> Self {
            Self(Default::default())
        }
    }
    impl From<BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>>
        for BTreeTblPartialExpressionAssignmentConstructor
    {
        fn from(map: BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>) -> Self {
            Self(BTreeMap::from(map))
        }
    }
    impl TryFromIterator<(TblExpressionVariable, TblSubexpressionInExpressionPath)>
        for BTreeTblPartialExpressionAssignmentConstructor
    {
        type Error = KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
        fn try_from_iter<
            T: IntoIterator<Item = (TblExpressionVariable, TblSubexpressionInExpressionPath)>,
        >(
            iter: T,
        ) -> Result<Self, Self::Error> {
            Ok(Self(BTreeMap::try_from_iter_conflictless(
                iter.into_iter(),
            )?))
        }
    }
    impl TryCombine for BTreeTblPartialExpressionAssignmentConstructor {
        type CombinationError =
            KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
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
        assignments::partial::constructor_implementations::btree::BTreeTblPartialExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

    impl Into<BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath>>
        for BTreeTblPartialExpressionAssignmentConstructor
    {
        fn into(self) -> BTreeMap<TblExpressionVariable, TblSubexpressionInExpressionPath> {
            self.0.into()
        }
    }
}

mod usage {
    use std::collections::BTreeMap;

    use itertools::Itertools;
    use proof_calculus::propositions::assignments::PartialPropositionalAssignmentConstructor;

    use crate::{
        expressions::{
            assignments::{
                full::constructor_implementations::btree::BTreeTblExpressionAssignmentConstructor,
                partial::implementations::{
                    btree::{
                        BTreeTblPartialExpressionAssignment, BTreeTblPartialPropositionAssignment,
                    },
                    dense::DenseTblPartialPropositionAssignment,
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
            BTreeTblPartialPropositionAssignment<PostAssignmentUcompound>,
        > for BTreeTblExpressionAssignmentConstructor
    {
        type Error = ();
        fn try_construct(
            &self,
            prop: &UnassignedTblExpression<PostAssignmentUcompound>,
        ) -> Result<BTreeTblPartialExpressionAssignment<PostAssignmentUcompound>, ()> {
            let inner: BTreeMap<_, _> = self
                .0
                .iter()
                .map(|(variable, path)| {
                    Ok((
                        *variable,
                        match prop.get_subexpression(&path) {
                            Ok(uexpr) => uexpr.into(),
                            Err(err) => return Err(err),
                        },
                    ))
                })
                .try_collect()
                .map_err(|_: ()| ())?;
            Ok(BTreeTblPartialExpressionAssignment(inner))
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
        > for BTreeTblExpressionAssignmentConstructor
    {
        type Error = ();
        fn try_construct(
            &self,
            prop: &UnassignedTblProposition<PostAssignmentUcompound>,
        ) -> Result<DenseTblPartialPropositionAssignment<PostAssignmentUcompound>, ()> {
            let values: Vec<_> = self
                .0
                .iter()
                .map(|(variable, path)| {
                    Ok((
                        *variable,
                        match prop.get_subexpression(path) {
                            Ok(uexpr) => uexpr.into(),
                            Err(err) => return Err(err),
                        },
                    ))
                })
                .try_collect()?;
            Ok(DenseTblPartialPropositionAssignment::from_iter_unchecked(
                values,
            ))
        }
    }
}
