use itertools::Itertools;
use proof_calculus::{propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment}, utils::{collections::maps::{KeyConflictError, dense_usize_map::DenseUsizeMap}, traits::{combinable::{TryCombine}, try_from_iter::TryFromIterator}}};

use crate::expressions::{types::{assigned::{TblExpression, compound::CompoundTblExpression}, unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression, variable::TblExpressionVariable}}};

pub mod constructors;

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct DenseTblExpressionAssignment<C: CompoundTblExpression>(DenseUsizeMap<TblExpressionVariable,TblExpression<C>>);
pub type DenseTblPropositionAssignment<C: CompoundTblExpression> = DenseTblExpressionAssignment<C>;
impl <C: CompoundTblExpression> DenseTblExpressionAssignment<C> {
    pub fn from_iter_unchecked<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Self 
        { Self(DenseUsizeMap::from_iter_unchecked(iter)) }
}
impl <C: CompoundTblExpression> Default for DenseTblExpressionAssignment<C> {
    fn default() -> Self { Self(Default::default()) }
}
impl <C: CompoundTblExpression> TryFromIterator<(TblExpressionVariable,TblExpression<C>)> for DenseTblExpressionAssignment<C> {
    type Error = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(DenseUsizeMap::try_from_iter(iter.into_iter())?)) }
}
impl <C: CompoundTblExpression> TryCombine for DenseTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|v| v.0))?)) }
}
impl <C: CompoundTblExpression + FromIterator<TblExpression<C>>, UC: UnassignedCompoundTblExpression> PropositionalAssignment<UnassignedTblExpression<UC>,TblExpression<C>> for DenseTblExpressionAssignment<C> {
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
pub struct DensePartialTblExpressionAssignment<UC: UnassignedCompoundTblExpression>(DenseUsizeMap<TblExpressionVariable,UnassignedTblExpression<UC>>);
pub type DensePartialTblPropositionAssignment<UC: CompoundTblExpression> = DensePartialTblExpressionAssignment<UC>;
impl <UC: UnassignedCompoundTblExpression> DensePartialTblExpressionAssignment<UC> {
    pub fn from_iter_unchecked<T: IntoIterator<Item = (TblExpressionVariable,UnassignedTblExpression<UC>)>>(iter: T) -> Self 
        { Self(DenseUsizeMap::from_iter_unchecked(iter)) }
}
impl <UC: UnassignedCompoundTblExpression> Default for DensePartialTblExpressionAssignment<UC> {
    fn default() -> Self { Self(Default::default()) }
}
impl <UC: UnassignedCompoundTblExpression> TryFromIterator<(TblExpressionVariable,UnassignedTblExpression<UC>)> for DensePartialTblExpressionAssignment<UC> {
    type Error = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,UnassignedTblExpression<UC>)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(DenseUsizeMap::try_from_iter(iter.into_iter())?)) }
}
impl <UC: UnassignedCompoundTblExpression> TryCombine for DensePartialTblExpressionAssignment<UC> {
    type CombinationError = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(DenseUsizeMap::merge_without_conflicts(assignments.into_iter().map(|v| v.0))?)) }
}
impl <'slf, 'from, SelfUcompound: 'slf + UnassignedCompoundTblExpression, FromUcompound: 'from + UnassignedCompoundTblExpression, ToUcompound: From<&'slf SelfUcompound> + From<&'from FromUcompound> + UnassignedCompoundTblExpression>
PartialPropositionalAssignment<'slf,'from,UnassignedTblExpression<FromUcompound>,UnassignedTblExpression<ToUcompound>>
for DensePartialTblExpressionAssignment<SelfUcompound> {
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
// TODO: consider performance implications of having a From<Vec> implementation for these to leverage From<Vec> of underlying DenseUsizeMap
