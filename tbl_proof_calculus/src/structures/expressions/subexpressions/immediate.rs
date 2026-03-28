use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::structures::expressions::{TblExpression, at_path_enum::{ExpressionAtPathEnum, OwnedExpressionAtPathEnum}, compound::CompoundTblExpression};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ImmediateSubexpressionInExpressionPath(pub usize);
impl From<usize> for ImmediateSubexpressionInExpressionPath {
    fn from(value: usize) -> Self { Self(value) }
}
impl Display for ImmediateSubexpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

generate_parent_of_children_trait!{
    TblExpression<C>, ImmediateSubexpressionInExpressionPath, (C: CompoundTblExpression),
    "immediate_subexpression", "immediate_subexpressions", "ImmediateSubexpressions"
}
pub type ImmediateSubexpressionInExpression<'a,C> = ObjAtPath<'a,TblExpression<C>,ImmediateSubexpressionInExpressionPath>;
pub type ImmediateSubexpressionInExpressionEnum<'a,C> = ExpressionAtPathEnum<'a,C,ImmediateSubexpressionInExpressionPath>;

pub type OwnedImmediateSubexpressionInExpression<C> = OwnedObjAtPath<TblExpression<C>,ImmediateSubexpressionInExpressionPath>;
pub type OwnedImmediateSubexpressionInExpressionEnum<C> = OwnedExpressionAtPathEnum<C,ImmediateSubexpressionInExpressionPath>;
