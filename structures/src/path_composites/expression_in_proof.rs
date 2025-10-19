use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, Path};

use crate::{expressions::{Expression, ExpressionInExpressionPath}, proof::{InferenceInProofPath, PropositionInInferencePath}, DisplayExt};

#[derive(Clone,PartialEq,Eq)]
pub struct ExpressionInProofPath{
    pub step_path: InferenceInProofPath,
    pub proposition_path: PropositionInInferencePath,
    pub subexpression_path: ExpressionInExpressionPath
}

impl Path for ExpressionInProofPath {}
impl Display for ExpressionInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}{}",self.step_path,self.proposition_path,self.subexpression_path.display())
    }
}

pub type ExpressionInProof<'a> = ObjAtPath<'a,Expression,ExpressionInProofPath>;
pub type OwnedExpressionInProof = OwnedObjAtPath<Expression,ExpressionInProofPath>;
