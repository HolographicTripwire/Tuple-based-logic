use std::{collections::HashSet, hash::Hash};

use proof_calculus::{propositions::collections::binders::unassigned::{GetBinderForUpropIdenticalToUprop, InsertBinderForUprop}, utils::collections::binders::{Binder, GetBinder, InsertBinder}};

use crate::{expressions::{assigned::{binding::{binders::{atom_value::TblExpressionBinderAtomExactValue, compound_length::TblExpressionBinderCompoundExactLength, value_duplication::TblExpressionBinderValueDuplication}, bounds::{TblPropositionBoundAtomExactValue, TblPropositionBoundCompoundExactLength, TblPropositionBoundValueDuplicated}}, subexpressions::TblSubexpressionInExpressionPath}, unassigned::{binding::{binders::variable_value::TblExpressionBinderVariableExactValue, bounds::{TblPropositionBoundVariableExactValue, UnassignedTblExpressionInsertionBound, UnassignedTblPropositionIdentityBound}, operation_bounds::{get_identical_to_uprop::fast_construct::TblFastConstructGetBoundsForUpropIdenticalToUprop, insert::TblFastConstructInsertionBoundsForUprop}}, compound::UnassignedCompoundTblExpression}}, proof_calculus_derived::aliases::propositions::UnassignedTblProposition};

pub mod variable_value;

pub struct UnassignedTblExpressionBinder<T: Hash + Eq + Clone> {
    atom_value_bounds: TblExpressionBinderAtomExactValue<T>,
    variable_value_bounds: TblExpressionBinderVariableExactValue<T>,
    compound_length_bounds: TblExpressionBinderCompoundExactLength<T>,
    duplicate_value_bounds: TblExpressionBinderValueDuplication<T>,
}
impl <T: Hash + Eq + Clone> UnassignedTblExpressionBinder<T> {
    fn get_unbounded_path_matches(&self, path: &TblSubexpressionInExpressionPath) -> HashSet<&T> {
        match (self.atom_value_bounds.get_no_bound(path), self.compound_length_bounds.get_no_bound(path)) {
            (None, None) => HashSet::new(),
            (None, Some(v)) => v.into_iter().collect(),
            (Some(v), None) => v.into_iter().collect(),
            (Some(v1), Some(v2)) => v1.into_iter().chain(v2.into_iter()).collect(),
        }
    }
}

pub type TblPropositionBinder<T> = UnassignedTblExpressionBinder<T>;
impl <T: Hash + Eq + Clone> Binder for TblPropositionBinder<T> {
    type Value = T;
    
    #[inline]
    fn get_all<'binder>(&'binder self) -> HashSet<&'binder Self::Value> { self.get_unbounded_path_matches(&TblSubexpressionInExpressionPath::default()) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundAtomExactValue> for TblPropositionBinder<T> {
    #[inline]
    fn get<'binder>(&'binder self, key: &TblPropositionBoundAtomExactValue) -> HashSet<&'binder Self::Value>
        { self.atom_value_bounds.get2(&key) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundVariableExactValue> for TblPropositionBinder<T> {
    #[inline]
    fn get<'binder>(&'binder self, key: &TblPropositionBoundVariableExactValue) -> HashSet<&'binder Self::Value>
        { self.variable_value_bounds.get2(&key) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundCompoundExactLength> for TblPropositionBinder<T> {
    #[inline]
    fn get<'binder>(&'binder self, key: &TblPropositionBoundCompoundExactLength) -> HashSet<&'binder Self::Value>
        { self.compound_length_bounds.get2(&key) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundValueDuplicated> for TblPropositionBinder<T> {
    #[inline]
    fn get<'binder>(&'binder self, key: &TblPropositionBoundValueDuplicated) -> HashSet<&'binder Self::Value>
        { self.duplicate_value_bounds.get2(&key) }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionIdentityBound> for TblPropositionBinder<T> {
    #[inline]
    fn get<'binder>(&'binder self, key: &UnassignedTblPropositionIdentityBound) -> HashSet<&'binder Self::Value> { match key {
        UnassignedTblPropositionIdentityBound::AtomValue(atom_bound) => self.get(atom_bound),
        UnassignedTblPropositionIdentityBound::VariableValue(variable_bound) => self.get(variable_bound),
        UnassignedTblPropositionIdentityBound::CompoundLength(compound_bound) => self.get(compound_bound),
    }}
}

impl <T: Hash + Eq + Clone> InsertBinder<TblFastConstructInsertionBoundsForUprop> for TblPropositionBinder<T> {
    fn insert_by_bounds(&mut self, bounds: &TblFastConstructInsertionBoundsForUprop, value: Self::Value) {
        for bound in bounds.bounds() { match bound {
            UnassignedTblExpressionInsertionBound::AtomValue(atom_bound) => 
                self.atom_value_bounds.insert(&atom_bound.path, atom_bound.value, value.clone()),
            UnassignedTblExpressionInsertionBound::VariableValue(variable_bound) => 
                self.variable_value_bounds.insert(&variable_bound.path, variable_bound.value, value.clone()),
            UnassignedTblExpressionInsertionBound::CompoundLength(compound_bound) => 
                self.compound_length_bounds.insert(&compound_bound.path, compound_bound.length, value.clone()),
            UnassignedTblExpressionInsertionBound::ValueDuplicated(dups_bound) => 
                self.duplicate_value_bounds.insert(dups_bound.path1().clone(), dups_bound.path2().clone(), value.clone()),
        };}
    }
}


impl <C: UnassignedCompoundTblExpression, T: Hash + Eq + Clone> GetBinderForUpropIdenticalToUprop<UnassignedTblProposition<C>> for TblPropositionBinder<T> {
    type DefaultGetBoundsForPropIdenticalToProp<'prop> = TblFastConstructGetBoundsForUpropIdenticalToUprop where C: 'prop;
}
impl <'prop, C: 'prop + UnassignedCompoundTblExpression, T: Hash + Eq + Clone> InsertBinderForUprop<'prop, UnassignedTblProposition<C>> for TblPropositionBinder<T> {
    type DefaultInsertionBounds = TblFastConstructInsertionBoundsForUprop;
}
