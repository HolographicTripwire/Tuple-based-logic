use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::structures::{propositions::paths::PropositionInSequentialProofStepPath, sequential_proofs::subproofs::SequentialProofInProofPath};

use crate::structures::expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression, subexpressions::SubexpressionInExpressionPath};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ExpressionInProofPath{
    pub step_path: SequentialProofInProofPath,
    pub proposition_path: PropositionInSequentialProofStepPath,
    pub subexpression_path: SubexpressionInExpressionPath
}

impl Display for ExpressionInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}::{}::{}",self.step_path,self.proposition_path,self.subexpression_path)
    }
}

pub type AtomicExpressionInProof<'a> = ObjAtPath<'a,AtomicTblExpression,ExpressionInProofPath>;
pub type OwnedAtomicExpressionInProof = OwnedObjAtPath<AtomicTblExpression,ExpressionInProofPath>;

pub type CompoundExpressionInProof<'a,C: CompoundTblExpression> = ObjAtPath<'a,C,ExpressionInProofPath>;
pub type OwnedCompoundExpressionInProof<C: CompoundTblExpression> = OwnedObjAtPath<C,ExpressionInProofPath>;

pub type ExpressionInProof<'a,C: CompoundTblExpression> = ObjAtPath<'a,TblExpression<C>,ExpressionInProofPath>;
pub type OwnedExpressionInProof<C: CompoundTblExpression> = OwnedObjAtPath<TblExpression<C>,ExpressionInProofPath>;

mod from {
    use crate::structures::{expressions::subexpressions::{SubexpressionInExpressionPath, immediate::ImmediateSubexpressionInExpressionPath}, proof_calculus_derived::path_composites::PropositionInProofPath};

    use super::*;

    impl From<PropositionInProofPath> for ExpressionInProofPath {
        fn from(path: PropositionInProofPath) -> Self { Self {
            step_path: path.step_path,
            proposition_path: path.proposition_path,
            subexpression_path: SubexpressionInExpressionPath::default(),
        }}
    }
    impl From<(ExpressionInProofPath,ImmediateSubexpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(mut value: (ExpressionInProofPath,ImmediateSubexpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.push(value.1);
            value.0
        }
    }
    impl From<(ExpressionInProofPath,SubexpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(mut value: (ExpressionInProofPath,SubexpressionInExpressionPath)) -> Self { 
            value.0.subexpression_path.0.append(&mut value.1.0);
            value.0
        }
    }
    impl From<(PropositionInProofPath,SubexpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(value: (PropositionInProofPath,SubexpressionInExpressionPath)) -> Self { Self {
                step_path: value.0.step_path,
                proposition_path: value.0.proposition_path,
                subexpression_path: value.1
        }}
    }
    impl From<(PropositionInProofPath,ImmediateSubexpressionInExpressionPath)> for ExpressionInProofPath {
        fn from(value: (PropositionInProofPath,ImmediateSubexpressionInExpressionPath)) -> Self { Self {
                step_path: value.0.step_path,
                proposition_path: value.0.proposition_path,
                subexpression_path: value.1.into()
        }}
    }
    impl From<(SequentialProofInProofPath,PropositionInSequentialProofStepPath)> for ExpressionInProofPath {
        fn from(value: (SequentialProofInProofPath,PropositionInSequentialProofStepPath)) -> Self { Self {
            step_path: value.0,
            proposition_path: value.1,
            subexpression_path: SubexpressionInExpressionPath::default(),
        }}
    }
}
