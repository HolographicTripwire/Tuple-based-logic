use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::{Expression, atomic::AtomicExpression, compound::CompoundExpression, subexpression::ExpressionInExpressionPath}, sequential_proofs::{ProofInProofPath, PropositionInProofStepPath}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ExpressionInProofPath{
    pub step_path: ProofInProofPath,
    pub proposition_path: PropositionInProofStepPath,
    pub subexpression_path: ExpressionInExpressionPath
}

impl Display for ExpressionInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}::{}::{}",self.step_path,self.proposition_path,self.subexpression_path)
    }
}

pub type AtomicExpressionInProof<'a> = ObjAtPath<'a,AtomicExpression,ExpressionInProofPath>;
pub type OwnedAtomicExpressionInProof = OwnedObjAtPath<AtomicExpression,ExpressionInProofPath>;

pub type CompoundExpressionInProof<'a> = ObjAtPath<'a,CompoundExpression,ExpressionInProofPath>;
pub type OwnedCompoundExpressionInProof = OwnedObjAtPath<CompoundExpression,ExpressionInProofPath>;

pub type ExpressionInProof<'a> = ObjAtPath<'a,Expression,ExpressionInProofPath>;
pub type OwnedExpressionInProof = OwnedObjAtPath<Expression,ExpressionInProofPath>;

mod from {
    use crate::{expressions::subexpression::immediate::ImmediateExpressionInExpressionPath, path_composites::PropositionInProofPath};

    use super::*;

    impl From<PropositionInProofPath> for ExpressionInProofPath {
        fn from(path: PropositionInProofPath) -> Self { Self {
            step_path: path.step_path,
            proposition_path: path.proposition_path,
            subexpression_path: ExpressionInExpressionPath::default(),
        }}
    }
    impl From<(ExpressionInProofPath,ImmediateExpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(mut value: (ExpressionInProofPath,ImmediateExpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.push(value.1);
            value.0
        }
    }
    impl From<(ExpressionInProofPath,ExpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(mut value: (ExpressionInProofPath,ExpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.append(&mut value.1.0);
            value.0
        }
    }
    impl From<(PropositionInProofPath,ExpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(value: (PropositionInProofPath,ExpressionInExpressionPath)) -> Self { Self {
                step_path: value.0.step_path,
                proposition_path: value.0.proposition_path,
                subexpression_path: value.1
        }}
    }
    impl From<(PropositionInProofPath,ImmediateExpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(value: (PropositionInProofPath,ImmediateExpressionInExpressionPath)) -> Self { Self {
                step_path: value.0.step_path,
                proposition_path: value.0.proposition_path,
                subexpression_path: value.1.into()
        }}
    }
    impl From<(ProofInProofPath,PropositionInProofStepPath)> for ExpressionInProofPath {
        fn from(value: (ProofInProofPath,PropositionInProofStepPath)) -> Self { Self {
            step_path: value.0,
            proposition_path: value.1,
            subexpression_path: ExpressionInExpressionPath::default(),
        }}
    }
}
