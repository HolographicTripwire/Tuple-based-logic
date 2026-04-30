use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::expressions::{paths::immediate::ImmediateTblSubexpressionInExpressionPath, types::{assigned::{TblExpression, at_path_enum::{OwnedTblExpressionAtPathEnum, TblExpressionAtPathEnum}}, unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression}}};

generate_parent_of_children_trait!{
    UnassignedTblExpression<C>, ImmediateTblSubexpressionInExpressionPath, (C: UnassignedCompoundTblExpression),
    "immediate_subexpression", "immediate_subexpressions", "ImmediateUnassignedSubexpressions"
}
pub type ImmediateSubexpressionInExpression<'a,C> = ObjAtPath<'a,TblExpression<C>,ImmediateTblSubexpressionInExpressionPath>;
pub type ImmediateSubexpressionInExpressionEnum<'a,C> = TblExpressionAtPathEnum<'a,C,ImmediateTblSubexpressionInExpressionPath>;

pub type OwnedImmediateUnassignedSubexpressionInExpression<C> = OwnedObjAtPath<TblExpression<C>,ImmediateTblSubexpressionInExpressionPath>;
pub type OwnedImmediateUnassignedSubexpressionInExpressionEnum<C> = OwnedTblExpressionAtPathEnum<C,ImmediateTblSubexpressionInExpressionPath>;
