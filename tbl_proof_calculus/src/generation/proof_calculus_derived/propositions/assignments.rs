use proof_calculus::{generation::propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment}, utils::collections::dense_usize_map::DenseUsizeMap};

use crate::{generation::expressions::{assignments::{PartialTblExpressionAssignment, TblExpressionAssignment}, compound::UnassignedCompoundTblExpression}, structures::expressions::compound::CompoundTblExpression};

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
