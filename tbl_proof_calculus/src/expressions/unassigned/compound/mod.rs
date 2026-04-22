use std::{hash::Hash, fmt::Debug};

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::unassigned::{UnassignedTblExpression, assignments::{PartialTblExpressionAssignment, TblExpressionAssignment}, subexpressions::{ParentOfUnassignedSubexpressions, immediate::ParentOfImmediateUnassignedSubexpressions}}, expressions::assigned::{TblExpression, compound::CompoundTblExpression}};

//pub mod r#ref;
pub mod r#box;
pub mod rc;
pub mod arc;

pub trait UnassignedCompoundTblExpression: Clone + PartialEq + Eq + Hash + Debug + From<Self::InnerCompound> + TryInto<Self::InnerCompound> + ParentOfImmediateUnassignedSubexpressions<Self> + ParentOfUnassignedSubexpressions<Self> {
    type InnerCompound: CompoundTblExpression;

    fn replace(&self, to_replace: &UnassignedTblExpression<Self>, replace_with: &UnassignedTblExpression<Self>) -> Self;
    fn as_slice(&self) -> &[UnassignedTblExpression<Self>];
    fn len(&self) -> usize;

    fn reverse_assign(&self, assigned: &Self) -> Result<TblExpressionAssignment<Self::InnerCompound>,()>;
    fn partial_reverse_assign(&self, assigned: &Self) -> Result<PartialTblExpressionAssignment<Self>,()> {
        self.as_slice().into_iter()
    }
}

pub type UnassignedCompoundTblExpressionAtPath<'a,C:UnassignedCompoundTblExpression,Path> = ObjAtPath<'a,C,Path>;
pub type OwnedUnassignedCompoundTblExpressionAtPath<C:UnassignedCompoundTblExpression,Path> = OwnedObjAtPath<C,Path>;
