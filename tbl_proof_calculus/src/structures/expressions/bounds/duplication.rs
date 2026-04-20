use proof_calculus::utils::traits::fast_ord::FastOrd;

use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath};

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct TblExpressionBoundValueDuplicated{
    path1: TblSubexpressionInExpressionPath,
    path2: TblSubexpressionInExpressionPath
}
impl TblExpressionBoundValueDuplicated {
    pub fn new(path1: TblSubexpressionInExpressionPath, path2: TblSubexpressionInExpressionPath) -> (Self,bool) {
        if path1.fast_cmp(&path2).is_lt() { (Self { path1, path2 },false) }
        else { (Self { path1: path2, path2: path1 },true) }
    }
    pub fn new_unchecked(path1: TblSubexpressionInExpressionPath, path2: TblSubexpressionInExpressionPath) -> Self {
        debug_assert!(path1.fast_cmp(&path2).is_lt());
        Self { path1, path2 }
    }

    pub fn path1(&self) -> &TblSubexpressionInExpressionPath { &self.path1 }
    pub fn path2(&self) -> &TblSubexpressionInExpressionPath { &self.path2 }
    pub fn into_paths(self) -> (TblSubexpressionInExpressionPath,TblSubexpressionInExpressionPath) { (self.path1,self.path2) }
}

pub type TblPropositionBoundValueDuplicated = TblExpressionBoundValueDuplicated;
