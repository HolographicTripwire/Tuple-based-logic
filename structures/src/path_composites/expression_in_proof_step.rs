use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, Path};

use crate::{expressions::{Expression, ExpressionInExpressionPath}, path_composites::proposition_in_proof::PropositionInProofPath, DisplayExt};

#[derive(Clone,PartialEq,Eq)]
pub struct ExpressionInProofStepPath {
    pub proposition_path: PropositionInProofPath,
    pub subexpression_path: ExpressionInExpressionPath,
}
impl Path for ExpressionInProofStepPath {}
impl Display for ExpressionInProofStepPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.proposition_path,self.subexpression_path.display())
    }
}

pub type ExpressionInProofStep<'a> = ObjAtPath<'a,Expression,ExpressionInProofStepPath>;
pub type OwnedExpressionInProofStep = OwnedObjAtPath<Expression,ExpressionInProofStepPath>;
