use std::collections::HashMap;

use crate::expressions::{
    assignments::full::constructor_implementations::hash::HashTblExpressionAssignmentConstructor,
    paths::TblSubexpressionInExpressionPath, types::unassigned::variable::TblExpressionVariable,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HashTblPartialExpressionAssignmentConstructor(
    pub HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath>,
);
pub type BTreeTblPartiallPropositionAssignmentConstructor = HashTblExpressionAssignmentConstructor;

mod construction {
    use std::collections::HashMap;

    use proof_calculus::utils::{
        collections::maps::KeyConflictError,
        traits::{
            combinable::TryCombine, map::MapWithoutConflicts, try_from_iter::TryFromIterator,
        },
    };

    use crate::expressions::{
        assignments::partial::constructor_implementations::hash::HashTblPartialExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

    impl Default for HashTblPartialExpressionAssignmentConstructor {
        fn default() -> Self {
            Self(Default::default())
        }
    }
    impl From<HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath>>
        for HashTblPartialExpressionAssignmentConstructor
    {
        fn from(map: HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath>) -> Self {
            Self(HashMap::from(map))
        }
    }
    impl TryFromIterator<(TblExpressionVariable, TblSubexpressionInExpressionPath)>
        for HashTblPartialExpressionAssignmentConstructor
    {
        type Error = KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
        fn try_from_iter<
            T: IntoIterator<Item = (TblExpressionVariable, TblSubexpressionInExpressionPath)>,
        >(
            iter: T,
        ) -> Result<Self, Self::Error> {
            Ok(Self(HashMap::try_from_iter_conflictless(iter.into_iter())?))
        }
    }
    impl TryCombine for HashTblPartialExpressionAssignmentConstructor {
        type CombinationError =
            KeyConflictError<TblExpressionVariable, TblSubexpressionInExpressionPath>;
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
        assignments::partial::constructor_implementations::hash::HashTblPartialExpressionAssignmentConstructor,
        paths::TblSubexpressionInExpressionPath,
        types::unassigned::variable::TblExpressionVariable,
    };

    impl Into<HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath>>
        for HashTblPartialExpressionAssignmentConstructor
    {
        fn into(self) -> HashMap<TblExpressionVariable, TblSubexpressionInExpressionPath> {
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
            assignments::partial::{
                constructor_implementations::hash::HashTblPartialExpressionAssignmentConstructor,
                implementations::{
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
        > for HashTblPartialExpressionAssignmentConstructor
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
        > for HashTblPartialExpressionAssignmentConstructor
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
