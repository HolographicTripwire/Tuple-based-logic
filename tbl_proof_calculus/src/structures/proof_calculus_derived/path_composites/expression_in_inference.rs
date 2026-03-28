use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::structures::propositions::paths::PropositionInSequentialProofStepPath;

use crate::structures::expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression, subexpressions::SubexpressionInExpressionPath};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ExpressionInInferencePath {
    pub proposition_path: PropositionInSequentialProofStepPath,
    pub subexpression_path: SubexpressionInExpressionPath,
}
impl Display for ExpressionInInferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.proposition_path,self.subexpression_path)
    }
}
pub type AtomicExpressionInInference<'a> = ObjAtPath<'a,AtomicTblExpression,ExpressionInInferencePath>;
pub type OwnedAtomicExpressionInInference = OwnedObjAtPath<AtomicTblExpression,ExpressionInInferencePath>;

pub type CompoundExpressionInInference<'a,C: CompoundTblExpression> = ObjAtPath<'a,C,ExpressionInInferencePath>;
pub type OwnedCompoundExpressionInInference<C: CompoundTblExpression> = OwnedObjAtPath<C,ExpressionInInferencePath>;

pub type ExpressionInInference<'a,C: CompoundTblExpression> = ObjAtPath<'a,TblExpression<C>,ExpressionInInferencePath>;
pub type OwnedExpressionInInference<C: CompoundTblExpression> = OwnedObjAtPath<TblExpression<C>,ExpressionInInferencePath>;

mod from {
    use crate::structures::expressions::subexpressions::immediate::ImmediateSubexpressionInExpressionPath;

    use super::*;

    impl From<PropositionInSequentialProofStepPath> for ExpressionInInferencePath {
        fn from(path: PropositionInSequentialProofStepPath) -> Self { Self {
            proposition_path: path,
            subexpression_path: SubexpressionInExpressionPath::default(),
        }}
    }
    impl From<(ExpressionInInferencePath,ImmediateSubexpressionInExpressionPath)> for ExpressionInInferencePath {
        fn from(mut value: (ExpressionInInferencePath,ImmediateSubexpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.push(value.1);
            value.0
        }
    }
    impl From<(ExpressionInInferencePath,SubexpressionInExpressionPath)> for ExpressionInInferencePath {
        fn from(mut value: (ExpressionInInferencePath,SubexpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.append(&mut value.1.0);
            value.0
        }
    }
    impl From<(PropositionInSequentialProofStepPath,SubexpressionInExpressionPath)> for ExpressionInInferencePath {
        fn from(value: (PropositionInSequentialProofStepPath,SubexpressionInExpressionPath)) -> Self { Self {
            proposition_path: value.0,
            subexpression_path: value.1
        }}
    }
    impl From<(PropositionInSequentialProofStepPath,ImmediateSubexpressionInExpressionPath)> for ExpressionInInferencePath {
        fn from(value: (PropositionInSequentialProofStepPath,ImmediateSubexpressionInExpressionPath)) -> Self { Self {
            proposition_path: value.0,
            subexpression_path: SubexpressionInExpressionPath(vec![value.1])
        }}
    }
}
