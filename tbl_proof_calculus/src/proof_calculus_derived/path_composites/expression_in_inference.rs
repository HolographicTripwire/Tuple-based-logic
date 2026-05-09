use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::propositions::types::assigned::paths::PropositionInSequentialProofStepPath;

use crate::expressions::{
    paths::TblSubexpressionInExpressionPath,
    types::assigned::{TblExpression, atom::TblExpressionAtom, compound::TblExpressionCompound},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TblExpressionInInferencePath {
    pub proposition_path: PropositionInSequentialProofStepPath,
    pub subexpression_path: TblSubexpressionInExpressionPath,
}
impl Display for TblExpressionInInferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.proposition_path, self.subexpression_path)
    }
}
pub type AtomicTblExpressionInInference<'a> =
    ObjAtPath<'a, TblExpressionAtom, TblExpressionInInferencePath>;
pub type OwnedAtomicTblExpressionInInference =
    OwnedObjAtPath<TblExpressionAtom, TblExpressionInInferencePath>;

pub type CompoundTblExpressionInInference<'a, C: TblExpressionCompound> =
    ObjAtPath<'a, C, TblExpressionInInferencePath>;
pub type OwnedCompoundTblExpressionInInference<C: TblExpressionCompound> =
    OwnedObjAtPath<C, TblExpressionInInferencePath>;

pub type TblExpressionInInference<'a, C: TblExpressionCompound> =
    ObjAtPath<'a, TblExpression<C>, TblExpressionInInferencePath>;
pub type OwnedTblExpressionInInference<C: TblExpressionCompound> =
    OwnedObjAtPath<TblExpression<C>, TblExpressionInInferencePath>;

mod from {
    use proof_calculus::propositions::types::assigned::paths::{
        AssumptionInSequentialProofStepPath, ExplicitConclusionInSequentialProofStepPath,
    };

    use crate::expressions::paths::immediate::ImmediateTblSubexpressionInExpressionPath;

    use super::*;

    impl From<PropositionInSequentialProofStepPath> for TblExpressionInInferencePath {
        fn from(path: PropositionInSequentialProofStepPath) -> Self {
            Self {
                proposition_path: path,
                subexpression_path: TblSubexpressionInExpressionPath::default(),
            }
        }
    }
    impl
        From<(
            TblExpressionInInferencePath,
            ImmediateTblSubexpressionInExpressionPath,
        )> for TblExpressionInInferencePath
    {
        fn from(
            mut value: (
                TblExpressionInInferencePath,
                ImmediateTblSubexpressionInExpressionPath,
            ),
        ) -> Self {
            value.0.subexpression_path.0.push(value.1);
            value.0
        }
    }
    impl
        From<(
            TblExpressionInInferencePath,
            TblSubexpressionInExpressionPath,
        )> for TblExpressionInInferencePath
    {
        fn from(
            mut value: (
                TblExpressionInInferencePath,
                TblSubexpressionInExpressionPath,
            ),
        ) -> Self {
            value.0.subexpression_path.0.append(&mut value.1.0);
            value.0
        }
    }
    impl
        From<(
            PropositionInSequentialProofStepPath,
            TblSubexpressionInExpressionPath,
        )> for TblExpressionInInferencePath
    {
        fn from(
            value: (
                PropositionInSequentialProofStepPath,
                TblSubexpressionInExpressionPath,
            ),
        ) -> Self {
            Self {
                proposition_path: value.0,
                subexpression_path: value.1,
            }
        }
    }
    impl
        From<(
            PropositionInSequentialProofStepPath,
            ImmediateTblSubexpressionInExpressionPath,
        )> for TblExpressionInInferencePath
    {
        fn from(
            value: (
                PropositionInSequentialProofStepPath,
                ImmediateTblSubexpressionInExpressionPath,
            ),
        ) -> Self {
            Self {
                proposition_path: value.0,
                subexpression_path: TblSubexpressionInExpressionPath(vec![value.1]),
            }
        }
    }

    impl From<AssumptionInSequentialProofStepPath> for TblExpressionInInferencePath {
        fn from(value: AssumptionInSequentialProofStepPath) -> Self {
            (
                PropositionInSequentialProofStepPath::from(value),
                TblSubexpressionInExpressionPath::from(vec![]),
            )
                .into()
        }
    }
    impl
        From<(
            AssumptionInSequentialProofStepPath,
            ImmediateTblSubexpressionInExpressionPath,
        )> for TblExpressionInInferencePath
    {
        fn from(
            value: (
                AssumptionInSequentialProofStepPath,
                ImmediateTblSubexpressionInExpressionPath,
            ),
        ) -> Self {
            (PropositionInSequentialProofStepPath::from(value.0), value.1).into()
        }
    }

    impl From<ExplicitConclusionInSequentialProofStepPath> for TblExpressionInInferencePath {
        fn from(value: ExplicitConclusionInSequentialProofStepPath) -> Self {
            (
                PropositionInSequentialProofStepPath::from(value),
                TblSubexpressionInExpressionPath::from(vec![]),
            )
                .into()
        }
    }
    impl
        From<(
            ExplicitConclusionInSequentialProofStepPath,
            ImmediateTblSubexpressionInExpressionPath,
        )> for TblExpressionInInferencePath
    {
        fn from(
            value: (
                ExplicitConclusionInSequentialProofStepPath,
                ImmediateTblSubexpressionInExpressionPath,
            ),
        ) -> Self {
            (PropositionInSequentialProofStepPath::from(value.0), value.1).into()
        }
    }
}
