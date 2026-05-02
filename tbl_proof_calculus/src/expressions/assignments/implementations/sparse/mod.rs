use std::collections::HashMap;

use itertools::Itertools;
use proof_calculus::{propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment}, utils::{collections::maps::{KeyConflictError, hashmap::{combine_hashmaps_without_conflicts, create_hashmap_without_conflicts}}, traits::{combinable::TryCombine, try_from_iter::TryFromIterator}}};

use crate::expressions::{types::assigned::{TblExpression, compound::CompoundTblExpression}, types::unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression, variable::TblExpressionVariable}};

pub mod constructors;

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct SparseTblExpressionAssignment<C: CompoundTblExpression>(pub HashMap<TblExpressionVariable, TblExpression<C>>);
pub type SparseTblPropositionAssignment<C: CompoundTblExpression> = SparseTblExpressionAssignment<C>;
impl <C: CompoundTblExpression> SparseTblExpressionAssignment<C> {
    fn from_iter_unchecked<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Self 
        { Self(HashMap::from_iter(iter)) }
}
impl <C: CompoundTblExpression> Default for SparseTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <C: CompoundTblExpression> TryFromIterator<(TblExpressionVariable,TblExpression<C>)> for SparseTblExpressionAssignment<C> {
    type Error = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Result<Self,Self::Error> {
        Ok(Self(create_hashmap_without_conflicts(iter)?))
    }
}
impl <C: CompoundTblExpression> TryCombine for SparseTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError> {
        let hashmaps = assignments.into_iter().map(|v| v.0);
        let combined_hashmap = combine_hashmaps_without_conflicts(hashmaps)?;
        Ok(Self(combined_hashmap))
    }
}
impl <C: CompoundTblExpression + FromIterator<TblExpression<C>>, UC: UnassignedCompoundTblExpression> PropositionalAssignment<UnassignedTblExpression<UC>,TblExpression<C>> for SparseTblExpressionAssignment<C> {
    fn assign_to(&self, uprop: &UnassignedTblExpression<UC>) -> Result<TblExpression<C>,()> {
        match uprop {
            UnassignedTblExpression::Atomic(atom) => Ok(TblExpression::Atomic(*atom)),
            UnassignedTblExpression::Variable(variable) => match self.0.get(variable) {
                Some(expr) => Ok(expr.clone()),
                None => Err(()),
            }, UnassignedTblExpression::Compound(compound) => Ok(TblExpression::Compound(
                compound.get_immediate_subexpressions().into_iter()
                    .map(|uexpr| self.assign_to(uexpr) )
                    .try_collect()?
            ))
        }
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct SparsePartialTblExpressionAssignment<C: UnassignedCompoundTblExpression>(pub HashMap<TblExpressionVariable, UnassignedTblExpression<C>>);
pub type SparsePartialTblPropositionAssignment<C: CompoundTblExpression> = SparsePartialTblExpressionAssignment<C>;
impl <C: UnassignedCompoundTblExpression> SparsePartialTblExpressionAssignment<C> {
    pub fn from_map_unchecked(map: HashMap<TblExpressionVariable,UnassignedTblExpression<C>>) -> Self 
        { Self(map) }
}
impl <C: UnassignedCompoundTblExpression> Default for SparsePartialTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <C: UnassignedCompoundTblExpression> TryFromIterator<(TblExpressionVariable,UnassignedTblExpression<C>)> for SparsePartialTblExpressionAssignment<C> {
    type Error = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<C>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,UnassignedTblExpression<C>)>>(iter: T) -> Result<Self,Self::Error> {
        Ok(Self(create_hashmap_without_conflicts(iter)?))
    }
}
impl <UC: UnassignedCompoundTblExpression> TryCombine for SparsePartialTblExpressionAssignment<UC> {
    type CombinationError = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError> {
        let hashmaps = assignments.into_iter().map(|v| v.0);
        let combined_hashmap = combine_hashmaps_without_conflicts(hashmaps)?;
        Ok(Self(combined_hashmap))
    }
}
impl <'slf: 'slf2, 'slf2, 'from: 'from2, 'from2, SelfUcompound: 'slf + UnassignedCompoundTblExpression, FromUcompound: 'from + UnassignedCompoundTblExpression, ToUcompound: From<&'slf2 SelfUcompound> + From<&'from2 FromUcompound> + UnassignedCompoundTblExpression>
PartialPropositionalAssignment<'slf,'from,UnassignedTblExpression<FromUcompound>,UnassignedTblExpression<ToUcompound>>
for SparsePartialTblExpressionAssignment<SelfUcompound> {
    fn assign_to(&'slf self, uprop: &'from UnassignedTblExpression<FromUcompound>) -> UnassignedTblExpression<ToUcompound> {
        match uprop {
            UnassignedTblExpression::Variable(variable) => match self.0.get(variable) {
                Some(uexpr) => uexpr.transmute_compound(),
                None => UnassignedTblExpression::Variable(*variable),
            },
            other => other.transmute_compound(),
        }
    }
}