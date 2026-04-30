use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::expressions::{paths::{TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath}, types::assigned::{TblExpression, at_path_enum::{OwnedTblExpressionAtPathEnum, TblExpressionAtPathEnum}, compound::CompoundTblExpression}};

pub mod iterators;

generate_parent_of_children_trait!{
    TblExpression<C>, ImmediateTblSubexpressionInExpressionPath, (C: CompoundTblExpression),
    "immediate_subexpression", "immediate_subexpressions", "ImmediateSubexpressions"
}
pub type ImmediateSubexpressionInExpression<'a,C> = ObjAtPath<'a,TblExpression<C>,ImmediateTblSubexpressionInExpressionPath>;
pub type ImmediateSubexpressionInExpressionEnum<'a,C> = TblExpressionAtPathEnum<'a,C,ImmediateTblSubexpressionInExpressionPath>;

pub type OwnedImmediateSubexpressionInExpression<C> = OwnedObjAtPath<TblExpression<C>,ImmediateTblSubexpressionInExpressionPath>;
pub type OwnedImmediateSubexpressionInExpressionEnum<C> = OwnedTblExpressionAtPathEnum<C,ImmediateTblSubexpressionInExpressionPath>;


generate_parent_of_children_trait!{
    TblExpression<C>, TblSubexpressionInExpressionPath, (C: CompoundTblExpression),
    "subexpression", "subexpressions", "Subexpressions"
}

pub type TblSubexpressionInExpression<'a,C> = ObjAtPath<'a,TblExpression<C>,TblSubexpressionInExpressionPath>;
pub type OwnedTblSubexpressionInExpression<C> = OwnedObjAtPath<TblExpression<C>,TblSubexpressionInExpressionPath>;
