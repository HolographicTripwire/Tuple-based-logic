use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::expressions::{Expression, at_path_enum::{ExpressionAtPathEnum, OwnedExpressionAtPathEnum}};

/// The atomic object that makes up [SubexpressionPaths](SubexpressionPath)
/// For example, within the [Expression] (a,(b,c),d), the [AtomicSubexpressionPath] 1 would lead to the [Expression] (b,c)
#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ImmediateExpressionInExpressionPath(pub usize);
impl From<usize> for ImmediateExpressionInExpressionPath {
    fn from(value: usize) -> Self { Self(value) }
}
impl Display for ImmediateExpressionInExpressionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        { write!(f, "{}", self.0) }
}

// A reference to an [Expression], located within another [Expression] by a [SubexpressionPath]
// An [Expression], located within another [Expression] by a [SubexpressionPath]
generate_parent_of_children_trait!{
    (Expression), ImmediateExpressionInExpressionPath,
    "immediate_subexpression", "immediate_subexpressions", "ImmediateSubexpressions"
}
pub type ImmediateSubexpressionInExpression<'a> = ObjAtPath<'a,Expression,ImmediateExpressionInExpressionPath>;
pub type ImmediateSubexpressionInExpressionEnum<'a> = ExpressionAtPathEnum<'a,ImmediateExpressionInExpressionPath>;

pub type OwnedImmediateSubexpressionInExpression = OwnedObjAtPath<Expression,ImmediateExpressionInExpressionPath>;
pub type OwnedImmediateSubexpressionInExpressionEnum = OwnedExpressionAtPathEnum<ImmediateExpressionInExpressionPath>;
