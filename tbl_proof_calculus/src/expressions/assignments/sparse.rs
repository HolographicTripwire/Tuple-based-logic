use std::collections::HashMap;

use proof_calculus::{propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment}, utils::collections::maps::{KeyConflictError, hashmap::combine_hashmaps_without_conflicts}};

use crate::expressions::{assigned::{TblExpression, compound::CompoundTblExpression}, unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression, variable::TblExpressionVariable}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct SparseTblExpressionAssignment<C: CompoundTblExpression>(pub HashMap<TblExpressionVariable, TblExpression<C>>);
impl <C: CompoundTblExpression> Default for SparseTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <C: CompoundTblExpression> FromIterator<(TblExpressionVariable,TblExpression<C>)> for SparseTblExpressionAssignment<C> {
    fn from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter.into_iter()))
    }
}
impl <C: CompoundTblExpression, UC: UnassignedCompoundTblExpression> PropositionalAssignment<UnassignedTblExpression<UC>> for SparseTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError> {
        let hashmaps = assignments.into_iter().map(|v| v.0);
        let combined_hashmap = combine_hashmaps_without_conflicts(hashmaps)?;
        Ok(Self(combined_hashmap))
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct SparsePartialTblExpressionAssignment<C: UnassignedCompoundTblExpression>(pub HashMap<TblExpressionVariable, UnassignedTblExpression<C>>);
impl <C: UnassignedCompoundTblExpression> Default for SparsePartialTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <C: UnassignedCompoundTblExpression> FromIterator<(TblExpressionVariable,UnassignedTblExpression<C>)> for SparsePartialTblExpressionAssignment<C> {
    fn from_iter<T: IntoIterator<Item = (TblExpressionVariable,UnassignedTblExpression<C>)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter.into_iter()))
    }
}
impl <UC: UnassignedCompoundTblExpression> PartialPropositionalAssignment<UnassignedTblExpression<UC>> for SparsePartialTblExpressionAssignment<UC> {
    type CombinationError = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError> {
        let hashmaps = assignments.into_iter().map(|v| v.0);
        let combined_hashmap = combine_hashmaps_without_conflicts(hashmaps)?;
        Ok(Self(combined_hashmap))
    }
}
