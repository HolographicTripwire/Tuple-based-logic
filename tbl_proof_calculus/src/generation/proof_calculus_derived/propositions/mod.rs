use proof_calculus::generation::propositions::UnassignedProposition;

use crate::{generation::expressions::{TblExpressionAssignment, UnassignedTblExpression, compound::UnassignedCompoundTblExpression}, structures::expressions::TblExpression};

pub type UnassignedTblProposition<C> = UnassignedTblExpression<C>;

impl <C: UnassignedCompoundTblExpression> UnassignedProposition for UnassignedTblProposition<C> {
    type AssignedResult = TblExpression<C::InnerCompound>;
    type Assignment = TblExpressionAssignment<C::InnerCompound>;

    fn assign(assignment: Self::Assignment) -> Self::AssignedResult {
        
    }
} 