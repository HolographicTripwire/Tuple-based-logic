use std::{fmt::Debug, hash::Hash};

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::utils::traits::combinable::TryCombine;

use crate::expressions::{
    assignments::implementations::btree::{
        BTreeTblExpressionAssignment, BTreeTblPartialExpressionAssignment,
    },
    types::{
        assigned::{TblExpression, compound::TblExpressionCompound},
        unassigned::{
            UnassignedTblExpression,
            subexpressions::{
                ParentOfUnassignedSubexpressions,
                immediate::ParentOfImmediateUnassignedSubexpressions,
            },
        },
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

    fn construct_sparse_assignment<C: TblExpressionCompound>(
        &self,
        assigned: &C,
    ) -> Result<BTreeTblExpressionAssignment<C>, ()> {
    }
    fn construct_partial_assignment<UC: UnassignedTblExpressionCompound>(
        &self,
        assigned: &UC,
    ) -> Result<BTreeTblPartialExpressionAssignment<Self>, ()> {
        if self.len() != assigned.len() {
            return Err(());
        }
        self.as_slice().into_iter()
    }
}

pub type UnassignedTblExpressionCompoundAtPath<'a, C: UnassignedTblExpressionCompound, Path> =
    ObjAtPath<'a, C, Path>;
pub type OwnedUnassignedTblExpressionCompoundAtPath<C: UnassignedTblExpressionCompound, Path> =
    OwnedObjAtPath<C, Path>;
