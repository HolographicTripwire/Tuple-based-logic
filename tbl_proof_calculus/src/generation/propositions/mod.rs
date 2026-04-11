use proof_calculus::{generation::propositions::{PartialPropositionalAssignment, PropositionalAssignment, UnassignedProposition}, utils::collections::dense_usize_map::DenseUsizeMap};

use crate::{generation::expressions::{PartialTblExpressionAssignment, TblExpressionAssignment, UnassignedTblExpression, compound::UnassignedCompoundTblExpression}, structures::expressions::{TblExpression, compound::CompoundTblExpression}};

pub type UnassignedTblProposition<C> = UnassignedTblExpression<C>;

impl <C: UnassignedCompoundTblExpression> UnassignedProposition for UnassignedTblExpression<C> {
    type AssignedResult = TblExpression<C::InnerCompound>;
    type Assignment = TblExpressionAssignment<C::InnerCompound>;
    type PartialAssignment = PartialTblExpressionAssignment<C>;
    
    fn assign(&self, assignment: &Self::Assignment) -> Result<Self::AssignedResult,()> {
        todo!()
    }
    
    fn reverse_assign(&self, assigned: Self::AssignedResult) -> Result<Self::Assignment,()> {
        todo!()
    }
    
    fn partial_assign(self, assignment: &Self::PartialAssignment) -> Self {
        match self {
            UnassignedTblExpression::Atomic(_) => self,
            UnassignedTblExpression::Compound(compound) => UnassignedTblExpression::Compound(compound.partial_assign(assignment)),
            UnassignedTblExpression::Variable(var) => {
                if let Some(a) = assignment.0.get(var) { a.clone() }
                else { self }
            },
        }
    }
    
    fn partial_reverse_assign(&self, assigned: &Self) -> Result<Self::PartialAssignment,()> {
        match (&self,assigned) {
            (UnassignedTblExpression::Atomic(a1), UnassignedTblExpression::Atomic(a2)) => 
                if a1 == a2 { Ok(PartialTblExpressionAssignment::default()) } else { Err(()) },
            (UnassignedTblExpression::Compound(c1), UnassignedTblExpression::Compound(c2)) =>
                c1.partial_reverse_assign(c2),
            (UnassignedTblExpression::Variable(var), other) => 
                Ok(PartialTblExpressionAssignment::from([(*var,other.clone())])),
            _ => Err(())
        }
    }
}

impl <C: CompoundTblExpression> PropositionalAssignment for TblExpressionAssignment<C> {
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,()> {
        match DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|x| x.0)) {
            Ok(merged) => Ok(Self(merged)),
            Err(_) => Err(()),
        }
    }
}

impl <C: UnassignedCompoundTblExpression> PartialPropositionalAssignment for PartialTblExpressionAssignment<C> {
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,()> {
        match DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|x| x.0)) {
            Ok(merged) => Ok(Self(merged)),
            Err(_) => Err(()),
        }
    }
}
