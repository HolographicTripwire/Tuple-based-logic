use std::{hash::Hash, fmt::Debug};

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};
use proof_calculus::utils::traits::{combinable::TryCombine, try_from_iter::TryFromIterator};

use crate::expressions::{assignments::implementations::sparse::{SparsePartialTblExpressionAssignment, SparseTblExpressionAssignment}, types::{assigned::{TblExpression, compound::CompoundTblExpression}, unassigned::{UnassignedTblExpression, subexpressions::{ParentOfUnassignedSubexpressions, immediate::ParentOfImmediateUnassignedSubexpressions}}}};

//pub mod r#ref;
pub mod r#box;
pub mod rc;
pub mod arc;

pub trait UnassignedCompoundTblExpression: Clone + PartialEq + Eq + Hash + Debug + From<Self::InnerCompound> + TryInto<Self::InnerCompound> + ParentOfImmediateUnassignedSubexpressions<Self> + ParentOfUnassignedSubexpressions<Self> {
    type InnerCompound: CompoundTblExpression;

    fn replace(&self, to_replace: &UnassignedTblExpression<Self>, replace_with: &UnassignedTblExpression<Self>) -> Self;
    fn as_slice(&self) -> &[UnassignedTblExpression<Self>];
    fn len(&self) -> usize;

    fn construct_assignment<C: CompoundTblExpression>(&self, assigned: &C) -> Result<SparseTblExpressionAssignment<Self::InnerCompound>,()> {
        if self.len() != assigned.len() { return Err(()) }
        let a = assigned.as_slice().iter();
        let u = self.as_slice().iter();
        let mut assignments = SparseTblExpressionAssignment::default();
        for (assigned_subexpression, unassigned_subexpression) in a.zip(u) {
            match (assigned_subexpression, unassigned_subexpression) {
                (TblExpression::Atomic(assigned_atom), UnassignedTblExpression::Atomic(unassigned_atom))
                    => { if assigned_atom != unassigned_atom { return Err(()) } },
                (TblExpression::Compound(assigned_compound), UnassignedTblExpression::Compound(unassigned_compound)) => {
                    let Ok(new_assignments) = unassigned_compound.construct_assignment(assigned_compound) else { return Err(()) };
                    match SparseTblExpressionAssignment::combine([assignments,new_assignments]) {
                        Ok(combined) => assignments = combined,
                        Err(_) => return Err(()),
                    };
                }, (expr, UnassignedTblExpression::Variable(variable)) => assignments.insert(*variable,expr.clone()),
                _ => return Err(())
            }
        }
        Ok(assignment)
    }
    fn construct_partial_assignment(&self, assigned: &Self) -> Result<SparsePartialTblExpressionAssignment<Self>,()> {
        if self.len() != assigned.len() { return Err(()) }
        self.as_slice().into_iter()
    }
}

pub type UnassignedCompoundTblExpressionAtPath<'a,C:UnassignedCompoundTblExpression,Path> = ObjAtPath<'a,C,Path>;
pub type OwnedUnassignedCompoundTblExpressionAtPath<C:UnassignedCompoundTblExpression,Path> = OwnedObjAtPath<C,Path>;
