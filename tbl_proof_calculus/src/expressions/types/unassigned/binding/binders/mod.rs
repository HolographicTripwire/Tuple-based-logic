use std::{collections::HashSet, hash::Hash};

use proof_calculus::{propositions::unassigned::binding::binders::{GetBinderForUpropIdenticalToUprop, InsertBinderForUprop}, utils::collections::{binding::binders::{Binder, GetBinder, InsertBinder}, sets::hashset::transform_hashset}};
use ref_cast::RefCast;

use crate::{expressions::types::{assigned::{atomic::AtomicTblExpression, binding::{binders::{atom_value::TblExpressionBinderAtomExactValue, compound_length::TblExpressionBinderCompoundExactLength, value_duplication::TblExpressionBinderValueDuplication}, bounds::{TblPropositionBoundAtomExactValue, TblPropositionBoundCompoundExactLength}}}, unassigned::{binding::{binders::variable_value::TblExpressionBinderVariableExactValue, bounds::{AtomOrVariableOrCompoundLength, UnassignedTblExpressionBoundAtomExistsAtLocation, UnassignedTblExpressionBoundCompoundExistsAtLocation, UnassignedTblExpressionBoundVariableExistsAtLocation, UnassignedTblExpressionInsertionBound, UnassignedTblPropositionBoundAtomExactValue, UnassignedTblPropositionBoundAtomExistsAtLocation, UnassignedTblPropositionBoundCompoundExactLength, UnassignedTblPropositionBoundCompoundExistsAtLocation, UnassignedTblPropositionBoundExpressionExistsAtLocation, UnassignedTblPropositionBoundValueDuplicated, UnassignedTblPropositionBoundVariableExactValue, UnassignedTblPropositionBoundVariableExistsAtLocation, UnassignedTblPropositionIdentityBound}, operation_bounds::{get_identical_to_uprop::fast_construct::TblFastConstructGetBoundsForUpropIdenticalToUprop, insert::TblFastConstructInsertionBoundsForUprop}}, compound::UnassignedCompoundTblExpression, variable::TblExpressionVariable}}, proof_calculus_derived::aliases::propositions::UnassignedTblProposition};

pub mod variable_value;

pub struct UnassignedTblExpressionBinder<T: Hash + Eq + Clone> {
    atom_value_bounds: TblExpressionBinderAtomExactValue<T>,
    variable_value_bounds: TblExpressionBinderVariableExactValue<T>,
    compound_length_bounds: TblExpressionBinderCompoundExactLength<T>,
    duplicate_value_bounds: TblExpressionBinderValueDuplication<T>,
}

pub type UnassignedTblPropositionBinder<T> = UnassignedTblExpressionBinder<T>;
impl <T: Hash + Eq + Clone> Binder for UnassignedTblPropositionBinder<T> {
    type Value = T;
    fn get_all<'binder>(&'binder self) -> HashSet<&'binder Self::Value> { self.get(&UnassignedTblPropositionBoundExpressionExistsAtLocation::default()) }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionBoundAtomExistsAtLocation> for UnassignedTblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundAtomExistsAtLocation) -> HashSet<&'binder Self::Value>
        { self.atom_value_bounds.get(bound) }    
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundAtomExistsAtLocation) -> HashSet<(&'binder Self::Value,AtomicTblExpression)>
        { self.atom_value_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionBoundAtomExactValue> for UnassignedTblPropositionBinder<T> {
    fn get<'binder>(&'binder self, key: &TblPropositionBoundAtomExactValue) -> HashSet<&'binder Self::Value>
        { self.atom_value_bounds.get(key) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundAtomExactValue) -> HashSet<(&'binder Self::Value,())>
        { self.atom_value_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionBoundVariableExistsAtLocation> for UnassignedTblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundVariableExistsAtLocation) -> HashSet<&'binder Self::Value>
        { self.variable_value_bounds.get(bound) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundVariableExistsAtLocation) -> HashSet<(&'binder Self::Value,TblExpressionVariable)>
        { self.variable_value_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionBoundVariableExactValue> for UnassignedTblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundVariableExactValue) -> HashSet<&'binder Self::Value>
        { self.variable_value_bounds.get(bound) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundVariableExactValue) -> HashSet<(&'binder Self::Value,())>
        { self.variable_value_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionBoundCompoundExistsAtLocation> for UnassignedTblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundCompoundExistsAtLocation) -> HashSet<&'binder Self::Value>
        { self.compound_length_bounds.get(bound) }    
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundCompoundExistsAtLocation) -> HashSet<(&'binder Self::Value,usize)>
        { self.compound_length_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionBoundCompoundExactLength> for UnassignedTblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &TblPropositionBoundCompoundExactLength) -> HashSet<&'binder Self::Value>
        { self.compound_length_bounds.get(bound) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundCompoundExactLength) -> HashSet<(&'binder Self::Value,())>
        { self.compound_length_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionBoundExpressionExistsAtLocation> for UnassignedTblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundExpressionExistsAtLocation) -> HashSet<&'binder Self::Value> {
        let h1 = self.get(UnassignedTblExpressionBoundAtomExistsAtLocation::ref_cast(&bound.path))
            .into_iter();
        let h2 = self.get(UnassignedTblExpressionBoundVariableExistsAtLocation::ref_cast(&bound.path))
            .into_iter();
        let h3 = self.get(UnassignedTblExpressionBoundCompoundExistsAtLocation::ref_cast(&bound.path))
            .into_iter();
        h1.chain(h2).chain(h3).collect()
    }    
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundExpressionExistsAtLocation) -> HashSet<(&'binder Self::Value,AtomOrVariableOrCompoundLength)> {
        let h1 = self.get_with_extra_data(UnassignedTblExpressionBoundAtomExistsAtLocation::ref_cast(&bound.path))
            .into_iter()
            .map(|(t,atom)| (t,AtomOrVariableOrCompoundLength::Atom(atom)));
        let h2 = self.get_with_extra_data(UnassignedTblExpressionBoundVariableExistsAtLocation::ref_cast(&bound.path))
            .into_iter()
            .map(|(t,variable)| (t,AtomOrVariableOrCompoundLength::Variable(variable)));
        let h3 = self.get_with_extra_data(UnassignedTblExpressionBoundCompoundExistsAtLocation::ref_cast(&bound.path))
            .into_iter()
            .map(|(t,length)| (t,AtomOrVariableOrCompoundLength::CompoundLength(length)));
        h1.chain(h2).chain(h3).collect()
    }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionBoundValueDuplicated> for UnassignedTblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundValueDuplicated) -> HashSet<&'binder Self::Value>
        { self.duplicate_value_bounds.get(bound) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblPropositionBoundValueDuplicated) -> HashSet<(&'binder Self::Value,())>
        { self.duplicate_value_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<UnassignedTblPropositionIdentityBound> for UnassignedTblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &UnassignedTblPropositionIdentityBound) -> HashSet<&'binder Self::Value> { match bound {
        UnassignedTblPropositionIdentityBound::AtomValue(atom_bound) => self.get(atom_bound),
        UnassignedTblPropositionIdentityBound::VariableValue(variable_bound) => self.get(variable_bound),
        UnassignedTblPropositionIdentityBound::CompoundLength(compound_bound) => self.get(compound_bound),
    }}
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblPropositionIdentityBound) -> HashSet<(&'binder Self::Value,())>
        { transform_hashset(self.get(bound), |v| (v,())) }
}

impl <T: Hash + Eq + Clone> InsertBinder<TblFastConstructInsertionBoundsForUprop> for UnassignedTblPropositionBinder<T> {
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

impl <C: UnassignedCompoundTblExpression, T: Hash + Eq + Clone> GetBinderForUpropIdenticalToUprop<UnassignedTblProposition<C>> for UnassignedTblPropositionBinder<T>
    { type DefaultGetBoundsForPropIdenticalToProp<'prop> = TblFastConstructGetBoundsForUpropIdenticalToUprop where C: 'prop; }
impl <'prop, C: 'prop + UnassignedCompoundTblExpression, T: Hash + Eq + Clone> InsertBinderForUprop<'prop, UnassignedTblProposition<C>> for UnassignedTblPropositionBinder<T>
    { type DefaultInsertionBounds = TblFastConstructInsertionBoundsForUprop; }
