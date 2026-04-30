use proof_calculus::propositions::types::{assigned::Proposition, unassigned::UnassignedProposition};

use crate::{expressions::types::{assigned::{TblExpression, compound::CompoundTblExpression}, unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression}}, proof_calculus_derived::path_composites::OwnedTblPropositionInProof};

pub type TblProposition<C> = TblExpression<C>;
pub type TblPropositionInSequentialProof<C> = OwnedTblPropositionInProof<TblProposition<C>>;

impl <C: CompoundTblExpression> Proposition for TblExpression<C> {}
pub type UnassignedTblProposition<C> = UnassignedTblExpression<C>;

impl <C: UnassignedCompoundTblExpression> UnassignedProposition for UnassignedTblProposition<C> {
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
                c1.construct_partial_assignment(c2),
            (UnassignedTblExpression::Variable(var), other) => 
                Ok(PartialTblExpressionAssignment::from([(*var,other.clone())])),
            _ => Err(())
        }
    }
} 
