use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression, subexpression::ExpressionInExpressionPath}, sequential_proofs::PropositionInProofStepPath};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ExpressionInInferencePath {
    pub proposition_path: PropositionInProofStepPath,
    pub subexpression_path: ExpressionInExpressionPath,
}
impl Display for ExpressionInInferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.proposition_path,self.subexpression_path)
    }
}
pub type AtomicExpressionInInference<'a> = ObjAtPath<'a,AtomicTblExpression,ExpressionInInferencePath>;
pub type OwnedAtomicExpressionInInference = OwnedObjAtPath<AtomicTblExpression,ExpressionInInferencePath>;

pub type CompoundExpressionInInference<'a> = ObjAtPath<'a,CompoundTblExpression,ExpressionInInferencePath>;
pub type OwnedCompoundExpressionInInference = OwnedObjAtPath<CompoundTblExpression,ExpressionInInferencePath>;

pub type ExpressionInInference<'a> = ObjAtPath<'a,TblExpression,ExpressionInInferencePath>;
pub type OwnedExpressionInInference = OwnedObjAtPath<TblExpression,ExpressionInInferencePath>;

mod from {
    use crate::expressions::subexpression::{ExpressionInExpressionPath, immediate::ImmediateExpressionInExpressionPath};

    use super::*;

    impl From<PropositionInProofStepPath> for ExpressionInInferencePath {
        fn from(path: PropositionInProofStepPath) -> Self { Self {
            proposition_path: path,
            subexpression_path: ExpressionInExpressionPath::default(),
        }}
    }
    impl From<(ExpressionInInferencePath,ImmediateExpressionInExpressionPath)> for ExpressionInInferencePath {
        fn from(mut value: (ExpressionInInferencePath,ImmediateExpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.push(value.1);
            value.0
        }
    }
    impl From<(ExpressionInInferencePath,ExpressionInExpressionPath)> for ExpressionInInferencePath {
        fn from(mut value: (ExpressionInInferencePath,ExpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.append(&mut value.1.0);
            value.0
        }
    }
    impl From<(PropositionInProofStepPath,ExpressionInExpressionPath)> for ExpressionInInferencePath {
        fn from(value: (PropositionInProofStepPath,ExpressionInExpressionPath)) -> Self { Self {
            proposition_path: value.0,
            subexpression_path: value.1
        }}
    }
    impl From<(PropositionInProofStepPath,ImmediateExpressionInExpressionPath)> for ExpressionInInferencePath {
        fn from(value: (PropositionInProofStepPath,ImmediateExpressionInExpressionPath)) -> Self { Self {
            proposition_path: value.0,
            subexpression_path: ExpressionInExpressionPath(vec![value.1])
        }}
    }
}
