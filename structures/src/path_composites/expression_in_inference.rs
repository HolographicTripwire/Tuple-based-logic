use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, Path};

use crate::{expressions::{Expression, ExpressionInExpressionPath}, path_composites::proposition_in_proof::PropositionInProofPath, DisplayExt};

#[derive(Clone,PartialEq,Eq)]
pub struct ExpressionInInferencePath {
    pub proposition_path: PropositionInProofPath,
    pub subexpression_path: ExpressionInExpressionPath,
}
impl Path for ExpressionInInferencePath {}
impl Display for ExpressionInInferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.proposition_path,self.subexpression_path.display())
    }
}

#[derive(Clone,PartialEq,Eq)]
pub struct ExpressionInInference<'a>(pub ObjAtPath<'a,Expression,ExpressionInInferencePath>);
#[derive(Clone,PartialEq,Eq)]
pub struct OwnedExpressionInInference(pub OwnedObjAtPath<Expression,ExpressionInInferencePath>);
