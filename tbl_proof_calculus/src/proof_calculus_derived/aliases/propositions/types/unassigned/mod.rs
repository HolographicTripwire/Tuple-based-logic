use proof_calculus::propositions::types::unassigned::UnassignedProposition;

use crate::expressions::types::unassigned::{
    UnassignedTblExpression, compound::UnassignedTblExpressionCompound,
};

pub mod binding;

pub type UnassignedTblProposition<C> = UnassignedTblExpression<C>;

impl<C: UnassignedTblExpressionCompound> UnassignedProposition for UnassignedTblProposition<C> {}
