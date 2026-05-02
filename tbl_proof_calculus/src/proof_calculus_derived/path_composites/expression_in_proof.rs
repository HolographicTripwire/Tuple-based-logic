use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::{propositions::types::assigned::paths::PropositionInSequentialProofStepPath, proofs::sequential::subproofs::SequentialProofInProofPath};

use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::assigned::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ExpressionInProofPath{
    pub step_path: SequentialProofInProofPath,
    pub proposition_path: PropositionInSequentialProofStepPath,
    pub subexpression_path: TblSubexpressionInExpressionPath
}

impl Display for ExpressionInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}::{}::{}",self.step_path,self.proposition_path,self.subexpression_path)
    }
}

pub type AtomicTblExpressionInProof<'a> = ObjAtPath<'a,AtomicTblExpression,ExpressionInProofPath>;
pub type OwnedAtomicTblExpressionInProof = OwnedObjAtPath<AtomicTblExpression,ExpressionInProofPath>;

pub type CompoundTblExpressionInProof<'a,C: CompoundTblExpression> = ObjAtPath<'a,C,ExpressionInProofPath>;
pub type OwnedTblCompoundExpressionInProof<C: CompoundTblExpression> = OwnedObjAtPath<C,ExpressionInProofPath>;

pub type TblExpressionInProof<'a,C: CompoundTblExpression> = ObjAtPath<'a,TblExpression<C>,ExpressionInProofPath>;
pub type TblOwnedExpressionInProof<C: CompoundTblExpression> = OwnedObjAtPath<TblExpression<C>,ExpressionInProofPath>;

mod from {
    use crate::{expressions::{paths::immediate::ImmediateTblSubexpressionInExpressionPath}, proof_calculus_derived::path_composites::PropositionInProofPath};

    use super::*;

    impl From<PropositionInProofPath> for ExpressionInProofPath {
        fn from(path: PropositionInProofPath) -> Self { Self {
            step_path: path.step_path,
            proposition_path: path.proposition_path,
            subexpression_path: TblSubexpressionInExpressionPath::default(),
        }}
    }
    impl From<(ExpressionInProofPath,ImmediateTblSubexpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(mut value: (ExpressionInProofPath,ImmediateTblSubexpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.push(value.1);
            value.0
        }
    }
    impl From<(ExpressionInProofPath,TblSubexpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(mut value: (ExpressionInProofPath,TblSubexpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.append(&mut value.1.0);
            value.0
        }
    }
    impl From<(PropositionInProofPath,TblSubexpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(value: (PropositionInProofPath,TblSubexpressionInExpressionPath)) -> Self { Self {
                step_path: value.0.step_path,
                proposition_path: value.0.proposition_path,
                subexpression_path: value.1
        }}
    }
    impl From<(PropositionInProofPath,ImmediateTblSubexpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(value: (PropositionInProofPath,ImmediateTblSubexpressionInExpressionPath)) -> Self { Self {
                step_path: value.0.step_path,
                proposition_path: value.0.proposition_path,
                subexpression_path: value.1.into()
        }}
    }
    impl From<(SequentialProofInProofPath,PropositionInSequentialProofStepPath)> for ExpressionInProofPath {
        fn from(value: (SequentialProofInProofPath,PropositionInSequentialProofStepPath)) -> Self { Self {
            step_path: value.0,
            proposition_path: value.1,
            subexpression_path: TblSubexpressionInExpressionPath::default(),
        }}
    }
}
