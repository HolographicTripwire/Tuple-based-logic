use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;
use proof_calculus::utils::traits::fast_ord::FastOrd;

use crate::structures::expressions::{TblExpression, at_path_enum::{TblExpressionAtPathEnum, OwnedTblExpressionAtPathEnum}, compound::CompoundTblExpression};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug,PartialOrd,Ord)]
pub struct ImmediateTblSubexpressionInExpressionPath(pub usize);
impl From<usize> for ImmediateTblSubexpressionInExpressionPath {
    fn from(value: usize) -> Self { Self(value) }
}
impl FastOrd for ImmediateTblSubexpressionInExpressionPath {
    #[inline] fn fast_cmp(&self, other: &Self) -> std::cmp::Ordering { self.cmp(other) }
}

impl Display for ImmediateTblSubexpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

generate_parent_of_children_trait!{
    TblExpression<C>, ImmediateTblSubexpressionInExpressionPath, (C: CompoundTblExpression),
    "immediate_subexpression", "immediate_subexpressions", "ImmediateSubexpressions"
}
pub type ImmediateSubexpressionInExpression<'a,C> = ObjAtPath<'a,TblExpression<C>,ImmediateTblSubexpressionInExpressionPath>;
pub type ImmediateSubexpressionInExpressionEnum<'a,C> = TblExpressionAtPathEnum<'a,C,ImmediateTblSubexpressionInExpressionPath>;

pub type OwnedImmediateSubexpressionInExpression<C> = OwnedObjAtPath<TblExpression<C>,ImmediateTblSubexpressionInExpressionPath>;
pub type OwnedImmediateSubexpressionInExpressionEnum<C> = OwnedTblExpressionAtPathEnum<C,ImmediateTblSubexpressionInExpressionPath>;
