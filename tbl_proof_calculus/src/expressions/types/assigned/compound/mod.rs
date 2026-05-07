use std::{hash::Hash, fmt::Debug};

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::expressions::types::assigned::{TblExpression, subexpressions::{ParentOfImmediateSubexpressions, ParentOfSubexpressions}};

//pub mod r#ref;
pub mod r#box;
pub mod rc;
pub mod arc;

pub trait TblExpressionCompound: Clone + PartialEq + Eq + Hash + Debug + ParentOfImmediateSubexpressions<Self> + ParentOfSubexpressions<Self> {
    fn replace(&self, to_replace: &TblExpression<Self>, replace_with: &TblExpression<Self>) -> Self;
    fn as_slice(&self) -> &[TblExpression<Self>];
    fn len(&self) -> usize;
}

pub type TblExpressionCompoundAtPath<'a,C:TblExpressionCompound,Path> = ObjAtPath<'a,C,Path>;
pub type OwnedTblExpressionCompoundAtPath<C:TblExpressionCompound,Path> = OwnedObjAtPath<C,Path>;
