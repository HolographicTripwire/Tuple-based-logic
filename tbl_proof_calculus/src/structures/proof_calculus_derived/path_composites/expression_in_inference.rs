use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::structures::propositions::paths::PropositionInSequentialProofStepPath;

use crate::structures::expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct TblExpressionInInferencePath {
    pub proposition_path: PropositionInSequentialProofStepPath,
    pub subexpression_path: TblSubexpressionInExpressionPath,
}
impl Display for TblExpressionInInferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.proposition_path,self.subexpression_path)
    }
}
pub type AtomicTblExpressionInInference<'a> = ObjAtPath<'a,AtomicTblExpression,TblExpressionInInferencePath>;
pub type OwnedAtomicTblExpressionInInference = OwnedObjAtPath<AtomicTblExpression,TblExpressionInInferencePath>;

pub type CompoundTblExpressionInInference<'a,C: CompoundTblExpression> = ObjAtPath<'a,C,TblExpressionInInferencePath>;
pub type OwnedCompoundTblExpressionInInference<C: CompoundTblExpression> = OwnedObjAtPath<C,TblExpressionInInferencePath>;

pub type TblExpressionInInference<'a,C: CompoundTblExpression> = ObjAtPath<'a,TblExpression<C>,TblExpressionInInferencePath>;
pub type OwnedTblExpressionInInference<C: CompoundTblExpression> = OwnedObjAtPath<TblExpression<C>,TblExpressionInInferencePath>;

mod from {
    use proof_calculus::structures::propositions::paths::{AssumptionInSequentialProofStepPath, ExplicitConclusionInSequentialProofStepPath};

    use crate::structures::expressions::subexpressions::immediate::ImmediateSubexpressionInExpressionPath;

    use super::*;

    impl From<PropositionInSequentialProofStepPath> for TblExpressionInInferencePath {
        fn from(path: PropositionInSequentialProofStepPath) -> Self { Self {
            proposition_path: path,
            subexpression_path: TblSubexpressionInExpressionPath::default(),
        }}
    }
    impl From<(TblExpressionInInferencePath,ImmediateSubexpressionInExpressionPath)> for TblExpressionInInferencePath {
        fn from(mut value: (TblExpressionInInferencePath,ImmediateSubexpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.push(value.1);
            value.0
        }
    }
    impl From<(TblExpressionInInferencePath,TblSubexpressionInExpressionPath)> for TblExpressionInInferencePath {
        fn from(mut value: (TblExpressionInInferencePath,TblSubexpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.append(&mut value.1.0);
            value.0
        }
    }
    impl From<(PropositionInSequentialProofStepPath,TblSubexpressionInExpressionPath)> for TblExpressionInInferencePath {
        fn from(value: (PropositionInSequentialProofStepPath,TblSubexpressionInExpressionPath)) -> Self { Self {
            proposition_path: value.0,
            subexpression_path: value.1
        }}
    }
    impl From<(PropositionInSequentialProofStepPath,ImmediateSubexpressionInExpressionPath)> for TblExpressionInInferencePath {
        fn from(value: (PropositionInSequentialProofStepPath,ImmediateSubexpressionInExpressionPath)) -> Self { Self {
            proposition_path: value.0,
            subexpression_path: TblSubexpressionInExpressionPath(vec![value.1])
        }}
    }
    
    impl From<(AssumptionInSequentialProofStepPath)> for TblExpressionInInferencePath {
        fn from(value: AssumptionInSequentialProofStepPath) -> Self { 
            (PropositionInSequentialProofStepPath::from(value),TblSubexpressionInExpressionPath::from(vec![])).into()
        }
    }
    impl From<(AssumptionInSequentialProofStepPath,ImmediateSubexpressionInExpressionPath)> for TblExpressionInInferencePath {
        fn from(value: (AssumptionInSequentialProofStepPath,ImmediateSubexpressionInExpressionPath)) -> Self { 
            (PropositionInSequentialProofStepPath::from(value.0),value.1).into()
        }
    }

    impl From<(ExplicitConclusionInSequentialProofStepPath)> for TblExpressionInInferencePath {
        fn from(value: ExplicitConclusionInSequentialProofStepPath) -> Self { 
            (PropositionInSequentialProofStepPath::from(value),TblSubexpressionInExpressionPath::from(vec![])).into()
        }
    }
    impl From<(ExplicitConclusionInSequentialProofStepPath,ImmediateSubexpressionInExpressionPath)> for TblExpressionInInferencePath {
        fn from(value: (ExplicitConclusionInSequentialProofStepPath,ImmediateSubexpressionInExpressionPath)) -> Self { 
            (PropositionInSequentialProofStepPath::from(value.0),value.1).into()
        }
    }
}
