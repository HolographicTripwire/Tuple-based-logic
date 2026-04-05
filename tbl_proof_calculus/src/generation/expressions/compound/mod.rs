use std::{hash::Hash, fmt::Debug};

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression};

//pub mod r#ref;
pub mod r#box;
pub mod rc;
pub mod arc;

pub trait UnassignedCompoundTblExpression: Clone + PartialEq + Eq + Hash + Debug + /*ParentOfImmediateSubexpressions<Self> + ParentOfSubexpressions<Self>*/ {
    
    // fn replace(&self, to_replace: &TblExpression<Self::InnerCompound>, replace_with: &TblExpression<Self::InnerCompound>) -> Self;
    // fn as_slice(&self) -> &[TblExpression<Self::InnerCompound>];
    fn len(&self) -> usize;
}

pub type UnassignedCompoundTblExpressionAtPath<'a,C:UnassignedCompoundTblExpression,Path> = ObjAtPath<'a,C,Path>;
pub type OwnedUnassignedCompoundTblExpressionAtPath<C:UnassignedCompoundTblExpression,Path> = OwnedObjAtPath<C,Path>;
