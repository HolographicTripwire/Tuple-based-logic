use std::hash::Hash;

use crate::structures::expressions::{TblExpression, subexpressions::immediate::ParentOfImmediateSubexpressions};

//pub mod r#ref;
pub mod r#box;
pub mod rc;
pub mod arc;

pub trait CompoundTblExpression: Clone + PartialEq + Eq + Hash + ParentOfImmediateSubexpressions<Self> {
    fn replace(&self, to_replace: &TblExpression<Self>, replace_with: &TblExpression<Self>) -> Self;

    fn len(&self) -> usize;
}
