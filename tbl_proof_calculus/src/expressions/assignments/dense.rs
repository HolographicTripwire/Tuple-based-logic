use proof_calculus::{propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment}, utils::{collections::maps::{KeyConflictError, dense_usize_map::DenseUsizeMap}, traits::try_from_iter::TryFromIterator}};

use crate::expressions::types::{assigned::{TblExpression, compound::CompoundTblExpression}, unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression, variable::TblExpressionVariable}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DenseTblExpressionAssignment<C: CompoundTblExpression>(DenseUsizeMap<TblExpression<C>>);
pub type DenseTblPropositionAssignment<C: CompoundTblExpression> = DenseTblExpressionAssignment<C>;
impl <C: CompoundTblExpression> DenseTblExpressionAssignment<C> {
    
}
impl <C: CompoundTblExpression> Default for DenseTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <C: CompoundTblExpression> TryFromIterator<(TblExpressionVariable,TblExpression<C>)> for DenseTblExpressionAssignment<C> {
    type Error = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Result<Self,Self::Error> {
        match DenseUsizeMap::try_from_iter(iter.into_iter().map(|(var, expr)| (var.into(), expr))) {
            Ok(inner_map) => Ok(Self(inner_map)),
            Err(error) => Err(KeyConflictError::new(error.key.into(), error.value1, error.value2)),
        }
    }
}
impl <C: CompoundTblExpression, UC: UnassignedCompoundTblExpression> PropositionalAssignment<UnassignedTblExpression<UC>> for DenseTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError> {
        match DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|v| v.0)) {
            Ok(merged) => Ok(Self(merged)),
            Err(error) => Err(KeyConflictError::new(error.key.into(), error.value1, error.value2)),
        }
    }
    
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DensePartialTblExpressionAssignment<C: UnassignedCompoundTblExpression>(DenseUsizeMap<UnassignedTblExpression<C>>);
pub type DensePartialTblPropositionAssignment<C: CompoundTblExpression> = DensePartialTblExpressionAssignment<C>;
impl <C: UnassignedCompoundTblExpression> Default for DensePartialTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <UC: UnassignedCompoundTblExpression> TryFromIterator<(TblExpressionVariable,UnassignedTblExpression<UC>)> for DensePartialTblExpressionAssignment<UC> {
    type Error = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,UnassignedTblExpression<UC>)>>(iter: T) -> Result<Self,Self::Error> {
        match DenseUsizeMap::try_from_iter(iter.into_iter().map(|(var, expr)| (var.into(), expr))) {
            Ok(inner_map) => Ok(Self(inner_map)),
            Err(error) => Err(KeyConflictError::new(error.key.into(), error.value1, error.value2)),
        }
    }
}
impl <C: UnassignedCompoundTblExpression> PartialPropositionalAssignment<UnassignedTblExpression<C>> for DensePartialTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<C>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError> {
        match DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|v| v.0)) {
            Ok(merged) => Ok(Self(merged)),
            Err(error) => Err(KeyConflictError::new(error.key.into(), error.value1, error.value2)),
        }
    }
}

// TODO: consider performance implications of having a From<Vec> implementation for these to leverage From<Vec> of underlying DenseUsizeMap
