use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::{generation::expressions::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression}, structures::expressions::{TblExpression, at_path_enum::{OwnedTblExpressionAtPathEnum, TblExpressionAtPathEnum}, subexpressions::immediate::ImmediateSubexpressionInExpressionPath}};

generate_parent_of_children_trait!{
    UnassignedTblExpression<C>, ImmediateSubexpressionInExpressionPath, (C: UnassignedCompoundTblExpression),
    "immediate_subexpression", "immediate_subexpressions", "ImmediateUnassignedSubexpressions"
}
pub type ImmediateSubexpressionInExpression<'a,C> = ObjAtPath<'a,TblExpression<C>,ImmediateSubexpressionInExpressionPath>;
pub type ImmediateSubexpressionInExpressionEnum<'a,C> = TblExpressionAtPathEnum<'a,C,ImmediateSubexpressionInExpressionPath>;

pub type OwnedImmediateUnassignedSubexpressionInExpression<C> = OwnedObjAtPath<TblExpression<C>,ImmediateSubexpressionInExpressionPath>;
pub type OwnedImmediateUnassignedSubexpressionInExpressionEnum<C> = OwnedTblExpressionAtPathEnum<C,ImmediateSubexpressionInExpressionPath>;
