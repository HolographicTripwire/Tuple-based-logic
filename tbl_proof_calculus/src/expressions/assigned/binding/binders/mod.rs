use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::{propositions::collections::binders::{GetBinderForPropIdenticalToProp, InsertBinderForProp}, utils::collections::{binders::{Binder, GetBinder, InsertBinder}, multimap::MultiMap}};

use crate::{expressions::assigned::{binding::{binders::{atom_value::TblExpressionTrackerBoundsAtomExactValue, compound_length::TblExpressionTrackerCompoundLengthBounds, value_duplication::TblExpressionTrackerDuplicationBounds}, bounds::{TblExpressionIdentityBound, TblExpressionSubsumptionBound, TblPropositionBoundAtomExactValue, TblPropositionBoundCompoundExactLength, TblPropositionBoundValueDuplicated, TblPropositionIdentityBound}, operation_bounds::{get_identical_to_prop::fast_construct::TblFastConstructGetBoundsForPropIdenticalToProp, insert::TblFastConstructInsertionBoundsForProp}}, compound::CompoundTblExpression, subexpressions::TblSubexpressionInExpressionPath}, proof_calculus_derived::aliases::propositions::TblProposition};

mod atom_value;
mod compound_length;
mod value_duplication;

pub struct TblExpressionBinder<T: Hash + Eq + Clone> {
    atom_value_bounds: TblExpressionTrackerBoundsAtomExactValue<T>,
    compound_length_bounds: TblExpressionTrackerCompoundLengthBounds<T>,
    duplicate_value_bounds: TblExpressionTrackerDuplicationBounds<T>,
}
impl <T: Hash + Eq + Clone> TblExpressionBinder<T> {
    fn get_unbounded_path_matches(&self, path: &TblSubexpressionInExpressionPath) -> HashSet<&T> {
        match (self.atom_value_bounds.get_no_bound(path), self.compound_length_bounds.get_no_bound(path)) {
            (None, None) => HashSet::new(),
            (None, Some(v)) => v.into_iter().collect(),
            (Some(v), None) => v.into_iter().collect(),
            (Some(v1), Some(v2)) => v1.into_iter().chain(v2.into_iter()).collect(),
        }
    }
}

pub type TblPropositionBinder<T> = TblExpressionBinder<T>;
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
impl <T: Hash + Eq + Clone> GetBinder<TblPropositionIdentityBound> for TblPropositionBinder<T> {
    #[inline]
    fn get<'binder>(&'binder self, key: &TblPropositionIdentityBound) -> HashSet<&'binder Self::Value> { match key {
        TblExpressionIdentityBound::AtomValue(atom_bound) => self.get(atom_bound),
        TblExpressionIdentityBound::CompoundLength(compound_bound) => self.get(compound_bound),
    }}
}

impl <T: Hash + Eq + Clone> InsertBinder<TblFastConstructInsertionBoundsForProp> for TblPropositionBinder<T> {
    fn insert_by_bounds(&mut self, bounds: &TblFastConstructInsertionBoundsForProp, value: Self::Value) {
        for bound in bounds.bounds() { match bound {
            TblExpressionSubsumptionBound::AtomValue(atom_bound) => 
                self.atom_value_bounds.insert(&atom_bound.path, atom_bound.value, value.clone()),
            TblExpressionSubsumptionBound::CompoundLength(compound_bound) => 
                self.compound_length_bounds.insert(&compound_bound.path, compound_bound.length, value.clone()),
            TblExpressionSubsumptionBound::ValueDuplicated(dups_bound) => 
                self.duplicate_value_bounds.insert(dups_bound.path1().clone(), dups_bound.path2().clone(), value.clone()),
        };}
    }
}


impl <C: CompoundTblExpression, T: Hash + Eq + Clone> GetBinderForPropIdenticalToProp<TblProposition<C>> for TblPropositionBinder<T> {
    type DefaultGetBoundsForPropIdenticalToProp<'prop> = TblFastConstructGetBoundsForPropIdenticalToProp where C: 'prop;
}
impl <'prop, C: 'prop + CompoundTblExpression, T: Hash + Eq + Clone> InsertBinderForProp<'prop, TblProposition<C>> for TblPropositionBinder<T> {
    type DefaultInsertionBounds = TblFastConstructInsertionBoundsForProp;
}

fn get_helper<'a,K1: Hash + Eq + Clone,K2: Hash + Eq,V: Hash + Eq>(map: &'a HashMap<K1,MultiMap<K2,V>>, key1: &K1, key2: &K2) -> HashSet<&'a V> {
    let optional_found = map.get(key1)
        .map(|inner| inner.get_refs(&key2));
    if let Some(Some(found)) = optional_found { found } else { HashSet::new() }
}

fn insert_helper<K1: Hash + Eq + Clone,K2: Hash + Eq,V: Hash + Eq>(map: &mut HashMap<K1,MultiMap<K2,V>>, key1: &K1, key2: K2, value: V) -> bool {
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
