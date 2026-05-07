use std::collections::HashMap;

use itertools::Itertools;
use proof_calculus::{propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment}, utils::{collections::maps::{KeyConflictError, conflictless_hashmap::{ConflictlessHashMap}}, traits::{combinable::TryCombine, try_from_iter::TryFromIterator}}};

use crate::expressions::{types::assigned::{TblExpression, compound::TblExpressionCompound}, types::unassigned::{UnassignedTblExpression, compound::UnassignedTblExpressionCompound, variable::TblExpressionVariable}};

pub mod constructors;

#[derive(Clone,PartialEq,Eq,Debug,Default)]
pub struct SparseTblExpressionAssignment<C: TblExpressionCompound>(pub ConflictlessHashMap<TblExpressionVariable, TblExpression<C>>);
pub type SparseTblPropositionAssignment<C: TblExpressionCompound> = SparseTblExpressionAssignment<C>;

impl <C: TblExpressionCompound> From<HashMap<TblExpressionVariable,TblExpression<C>>> for SparseTblExpressionAssignment<C>
    { fn from(map: HashMap<TblExpressionVariable,TblExpression<C>>) -> Self { Self(ConflictlessHashMap::from(map)) } }
impl <C: TblExpressionCompound> Into<HashMap<TblExpressionVariable,TblExpression<C>>> for SparseTblExpressionAssignment<C>
    { fn into(self) -> HashMap<TblExpressionVariable,TblExpression<C>> { self.0.into() } }
impl <C: TblExpressionCompound> TryFromIterator<(TblExpressionVariable,TblExpression<C>)> for SparseTblExpressionAssignment<C> {
    type Error = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,TblExpression<C>)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(ConflictlessHashMap::try_from_iter(iter.into_iter())?)) }
} impl <C: TblExpressionCompound> TryCombine for SparseTblExpressionAssignment<C> {
    type CombinationError = KeyConflictError<TblExpressionVariable,TblExpression<C>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(ConflictlessHashMap::combine(assignments.into_iter().map(|v| v.0))?)) }
}

impl <C: TblExpressionCompound + FromIterator<TblExpression<C>>, UC: UnassignedTblExpressionCompound> PropositionalAssignment<UnassignedTblExpression<UC>,TblExpression<C>> for SparseTblExpressionAssignment<C> {
    fn assign_to(&self, uprop: &UnassignedTblExpression<UC>) -> Result<TblExpression<C>,()> {
        match uprop {
            UnassignedTblExpression::Atom(atom) => Ok(TblExpression::Atom(*atom)),
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

#[derive(Clone,PartialEq,Eq,Debug,Default)]
pub struct SparsePartialTblExpressionAssignment<C: UnassignedTblExpressionCompound>(pub ConflictlessHashMap<TblExpressionVariable, UnassignedTblExpression<C>>);
pub type SparsePartialTblPropositionAssignment<C: TblExpressionCompound> = SparsePartialTblExpressionAssignment<C>;

impl <UC: UnassignedTblExpressionCompound> From<HashMap<TblExpressionVariable,UnassignedTblExpression<UC>>> for SparsePartialTblExpressionAssignment<UC>
    { fn from(map: HashMap<TblExpressionVariable,UnassignedTblExpression<UC>>) -> Self { Self(ConflictlessHashMap::from(map)) } }
impl <UC: UnassignedTblExpressionCompound> Into<HashMap<TblExpressionVariable,UnassignedTblExpression<UC>>> for SparsePartialTblExpressionAssignment<UC>
    { fn into(self) -> HashMap<TblExpressionVariable,UnassignedTblExpression<UC>> { self.0.into() } }
impl <UC: UnassignedTblExpressionCompound> TryFromIterator<(TblExpressionVariable,UnassignedTblExpression<UC>)> for SparsePartialTblExpressionAssignment<UC> {
    type Error = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>;
    fn try_from_iter<T: IntoIterator<Item = (TblExpressionVariable,UnassignedTblExpression<UC>)>>(iter: T) -> Result<Self,Self::Error>
        { Ok(Self(ConflictlessHashMap::try_from_iter(iter.into_iter())?)) }
} impl <UC: UnassignedTblExpressionCompound> TryCombine for SparsePartialTblExpressionAssignment<UC> {
    type CombinationError = KeyConflictError<TblExpressionVariable,UnassignedTblExpression<UC>>;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>
        { Ok(Self(ConflictlessHashMap::combine(assignments.into_iter().map(|v| v.0))?)) }
}

impl <'slf: 'slf2, 'slf2, 'from: 'from2, 'from2, SelfUcompound: 'slf + UnassignedTblExpressionCompound, FromUcompound: 'from + UnassignedTblExpressionCompound, ToUcompound: From<&'slf2 SelfUcompound> + From<&'from2 FromUcompound> + UnassignedTblExpressionCompound>
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
