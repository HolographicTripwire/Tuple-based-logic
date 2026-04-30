use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::{propositions::assigned::binding::binders::{GetBinderForPropIdenticalToProp, InsertBinderForProp}, utils::collections::{binding::binders::{Binder, GetBinder, InsertBinder}, maps::multimap::MultiMap, sets::hashset::transform_hashset}};
use ref_cast::RefCast;

use crate::{expressions::assigned::{atomic::AtomicTblExpression, binding::{binders::{atom_value::TblExpressionBinderAtomExactValue, compound_length::TblExpressionBinderCompoundExactLength, value_duplication::TblExpressionBinderValueDuplication}, bounds::{AtomOrCompoundLength, TblExpressionBoundAtomExistsAtLocation, TblExpressionBoundCompoundExistsAtLocation, TblExpressionBoundExpressionExistsAtLocation, TblExpressionIdentityBound, TblExpressionInsertionBound, TblPropositionBoundAtomExactValue, TblPropositionBoundAtomExistsAtLocation, TblPropositionBoundCompoundExactLength, TblPropositionBoundCompoundExistsAtLocation, TblPropositionBoundExpressionExistsAtLocation, TblPropositionBoundValueDuplicated, TblPropositionIdentityBound}, operation_bounds::{get_identical_to_prop::fast_construct::TblFastConstructGetBoundsForPropIdenticalToProp, insert::TblFastConstructInsertionBoundsForProp}}, compound::CompoundTblExpression}, proof_calculus_derived::aliases::propositions::TblProposition};

pub mod atom_value;
pub mod compound_length;
pub mod value_duplication;

pub struct TblExpressionBinder<T: Hash + Eq + Clone> {
    atom_value_bounds: TblExpressionBinderAtomExactValue<T>,
    compound_length_bounds: TblExpressionBinderCompoundExactLength<T>,
    duplicate_value_bounds: TblExpressionBinderValueDuplication<T>,
}

pub type TblPropositionBinder<T> = TblExpressionBinder<T>;
impl <T: Hash + Eq + Clone> Binder for TblPropositionBinder<T> {
    type Value = T;
    
    fn get_all<'binder>(&'binder self) -> HashSet<&'binder Self::Value> { self.get(&TblExpressionBoundExpressionExistsAtLocation::default()) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundAtomExistsAtLocation> for TblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &TblPropositionBoundAtomExistsAtLocation) -> HashSet<&'binder Self::Value>
        { self.atom_value_bounds.get(bound) }    
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblPropositionBoundAtomExistsAtLocation) -> HashSet<(&'binder Self::Value,AtomicTblExpression)>
        { self.atom_value_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundAtomExactValue> for TblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &TblPropositionBoundAtomExactValue) -> HashSet<&'binder Self::Value>
        { self.atom_value_bounds.get(bound) }    
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblPropositionBoundAtomExactValue) -> HashSet<(&'binder Self::Value,())>
        { self.atom_value_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundCompoundExistsAtLocation> for TblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &TblPropositionBoundCompoundExistsAtLocation) -> HashSet<&'binder Self::Value>
        { self.compound_length_bounds.get(bound) }    
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblPropositionBoundCompoundExistsAtLocation) -> HashSet<(&'binder Self::Value,usize)>
        { self.compound_length_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundCompoundExactLength> for TblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &TblPropositionBoundCompoundExactLength) -> HashSet<&'binder Self::Value>
        { self.compound_length_bounds.get(bound) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblPropositionBoundCompoundExactLength) -> HashSet<(&'binder Self::Value,())>
        { self.compound_length_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundExpressionExistsAtLocation> for TblPropositionBinder<T> {
    fn get<'binder>(&'binder self, bound: &TblPropositionBoundExpressionExistsAtLocation) -> HashSet<&'binder Self::Value> {
        let h1 = self.get(TblExpressionBoundAtomExistsAtLocation::ref_cast(&bound.path))
            .into_iter();
        let h2 = self.get(TblExpressionBoundCompoundExistsAtLocation::ref_cast(&bound.path))
            .into_iter();
        h1.chain(h2).collect()
    }    
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblPropositionBoundExpressionExistsAtLocation) -> HashSet<(&'binder Self::Value,AtomOrCompoundLength)> {
        let h1 = self.get_with_extra_data(TblExpressionBoundAtomExistsAtLocation::ref_cast(&bound.path))
            .into_iter()
            .map(|(t,atom)| (t,AtomOrCompoundLength::Atom(atom)));
        let h2 = self.get_with_extra_data(TblExpressionBoundCompoundExistsAtLocation::ref_cast(&bound.path))
            .into_iter()
            .map(|(t,length)| (t,AtomOrCompoundLength::CompoundLength(length)));
        h1.chain(h2).collect()
    }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionBoundValueDuplicated> for TblPropositionBinder<T> {
    fn get<'binder>(&'binder self, key: &TblPropositionBoundValueDuplicated) -> HashSet<&'binder Self::Value>
        { self.duplicate_value_bounds.get(&key) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblPropositionBoundValueDuplicated) -> HashSet<(&'binder Self::Value,<TblPropositionBoundValueDuplicated as proof_calculus::utils::collections::binding::bounds::GetBound>::ExtraReturnData)>
        { self.duplicate_value_bounds.get_with_extra_data(bound) }
}
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionIdentityBound> for TblPropositionBinder<T> {
    fn get<'binder>(&'binder self, key: &TblPropositionIdentityBound) -> HashSet<&'binder Self::Value> { match key {
        TblExpressionIdentityBound::AtomValue(atom_bound) => self.get(atom_bound),
        TblExpressionIdentityBound::CompoundLength(compound_bound) => self.get(compound_bound),
    }}
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblPropositionIdentityBound) -> HashSet<(&'binder Self::Value,<TblPropositionIdentityBound as proof_calculus::utils::collections::binding::bounds::GetBound>::ExtraReturnData)>
        { transform_hashset(self.get(bound), |v|(v,())) }
}

impl <T: Hash + Eq + Clone> InsertBinder<TblFastConstructInsertionBoundsForProp> for TblPropositionBinder<T> {
    fn insert_by_bounds(&mut self, bounds: &TblFastConstructInsertionBoundsForProp, value: Self::Value) {
        for bound in bounds.bounds() { match bound {
            TblExpressionInsertionBound::AtomValue(atom_bound) => 
                self.atom_value_bounds.insert(&atom_bound.path, atom_bound.value, value.clone()),
            TblExpressionInsertionBound::CompoundLength(compound_bound) => 
                self.compound_length_bounds.insert(&compound_bound.path, compound_bound.length, value.clone()),
            TblExpressionInsertionBound::ValueDuplicated(dups_bound) => 
                self.duplicate_value_bounds.insert(dups_bound.path1().clone(), dups_bound.path2().clone(), value.clone()),
        };}
    }
}

impl <C: CompoundTblExpression, T: Hash + Eq + Clone> GetBinderForPropIdenticalToProp<TblProposition<C>> for TblPropositionBinder<T>
    { type DefaultGetBoundsForPropIdenticalToProp<'prop> = TblFastConstructGetBoundsForPropIdenticalToProp where C: 'prop; }
impl <'prop, C: 'prop + CompoundTblExpression, T: Hash + Eq + Clone> InsertBinderForProp<'prop, TblProposition<C>> for TblPropositionBinder<T>
    { type DefaultInsertionBounds = TblFastConstructInsertionBoundsForProp; }

pub (crate) fn get_helper<'a,K1: Hash + Eq + Clone,K2: Hash + Eq,V: Hash + Eq>(map: &'a HashMap<K1,MultiMap<K2,V>>, key1: &K1, key2: &K2) -> HashSet<&'a V> {
    let optional_found = map.get(key1)
        .map(|inner| inner.get_refs(&key2));
    if let Some(Some(found)) = optional_found { found } else { HashSet::new() }
}

pub (crate) fn insert_helper<K1: Hash + Eq + Clone,K2: Hash + Eq,V: Hash + Eq>(map: &mut HashMap<K1,MultiMap<K2,V>>, key1: &K1, key2: K2, value: V) -> bool {
    match map.get_mut(&key1) {
        Some(inner) => inner.insert(key2, value),
        None => {
            let mut inner = MultiMap::new();
            inner.insert(key2, value);
            map.insert(key1.clone(), inner);
            true
        },
    }
}
