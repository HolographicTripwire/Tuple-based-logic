use proof_calculus::utils::collections::maps::KeyConflictError;

use crate::expressions::types::{assigned::atom::TblExpressionAtom, unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound, variable::TblExpressionVariable}};

pub enum TblPartialReverseAssignmentError<UC: UnassignedTblExpressionCompound> {
    AtomConflict(TblExpressionAtom, TblExpressionAtom),
    VariableConflict(KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>)
}
