use std::hash::Hash;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::expressions::{TblExpression, subexpressions::{ParentOfSubexpressions, immediate::ParentOfImmediateSubexpressions}};

//pub mod r#ref;
pub mod r#box;
pub mod rc;
pub mod arc;

pub trait CompoundTblExpression: Clone + PartialEq + Eq + Hash + ParentOfImmediateSubexpressions<Self> + ParentOfSubexpressions<Self> {
    fn replace(&self, to_replace: &TblExpression<Self>, replace_with: &TblExpression<Self>) -> Self;

    fn len(&self) -> usize;
}

pub type CompoundTblExpressionAtPath<'a,C:CompoundTblExpression,Path> = ObjAtPath<'a,C,Path>;
pub type OwnedCompoundTblExpressionAtPath<C:CompoundTblExpression,Path> = OwnedObjAtPath<C,Path>;
