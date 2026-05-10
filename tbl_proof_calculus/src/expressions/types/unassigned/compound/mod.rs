use std::{fmt::Debug, hash::Hash};

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::expressions::types::unassigned::{
    UnassignedTblExpression,
    subexpressions::{
        ParentOfUnassignedSubexpressions, immediate::ParentOfImmediateUnassignedSubexpressions,
    },
};

//pub mod r#ref;
pub mod arc;
pub mod r#box;
pub mod rc;

pub trait UnassignedTblExpressionCompound:
    Clone
    + PartialEq
    + Eq
    + Hash
    + Debug
    + ParentOfImmediateUnassignedSubexpressions<Self>
    + ParentOfUnassignedSubexpressions<Self>
{
    fn replace(
        &self,
        to_replace: &UnassignedTblExpression<Self>,
        replace_with: &UnassignedTblExpression<Self>,
    ) -> Self;
    fn as_slice(&self) -> &[UnassignedTblExpression<Self>];
    fn len(&self) -> usize;
}

pub type UnassignedTblExpressionCompoundAtPath<'a, C: UnassignedTblExpressionCompound, Path> =
    ObjAtPath<'a, C, Path>;
pub type OwnedUnassignedTblExpressionCompoundAtPath<C: UnassignedTblExpressionCompound, Path> =
    OwnedObjAtPath<C, Path>;
