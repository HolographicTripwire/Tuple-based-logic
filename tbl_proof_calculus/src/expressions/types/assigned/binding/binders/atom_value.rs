use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::utils::collections::{binding::binders::{Binder, GetBinder}, maps::multimap::MultiMap, sets::hashset::transform_hashset};

use crate::expressions::{paths::TblSubexpressionInExpressionPath, types::assigned::{atomic::AtomicTblExpression, binding::{binders::{get_helper, insert_helper}, bounds::{TblExpressionBoundAtomExactValue, TblExpressionBoundAtomExistsAtLocation}}}};

pub struct TblExpressionBinderAtomExactValue<T: Hash + Eq>(HashMap<TblSubexpressionInExpressionPath,MultiMap<AtomicTblExpression,T>>);

impl <T: Eq + Hash> Binder for TblExpressionBinderAtomExactValue<T> {
    type Value = T;

    fn get_all<'binder>(&'binder self) -> HashSet<&'binder Self::Value> {
        self.0.iter()
            .map(|(_,inner)| inner.flat_values())
            .flatten()
            .collect()
    }
}
impl <T: Eq + Hash> GetBinder<TblExpressionBoundAtomExactValue> for TblExpressionBinderAtomExactValue<T> {
    fn get<'binder>(&'binder self, bound: &TblExpressionBoundAtomExactValue) -> HashSet<&'binder Self::Value>
        { self.get_inner(&bound.path,&bound.value) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblExpressionBoundAtomExactValue) -> HashSet<(&'binder Self::Value,())>
        { transform_hashset(self.get_inner(&bound.path,&bound.value), |v| (v,())) }
}
impl <T: Eq + Hash> GetBinder<TblExpressionBoundAtomExistsAtLocation> for TblExpressionBinderAtomExactValue<T> {
    fn get<'binder>(&'binder self, bound: &TblExpressionBoundAtomExistsAtLocation) -> HashSet<&'binder Self::Value> {
        match self.0.get(&bound.path) {
            Some(inner) =>  inner.flat_values().into_iter().collect(),
            None => HashSet::new(),
        }
    }
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblExpressionBoundAtomExistsAtLocation) -> HashSet<(&'binder Self::Value,AtomicTblExpression)> {
        match self.0.get(&bound.path) {
            Some(inner) => inner.pairs().into_iter().map(|(atom,value)| (value,*atom)).collect(),
            None => HashSet::new(),
        }
    }
}

impl <T: Hash + Eq> TblExpressionBinderAtomExactValue<T> {
    fn get_inner(&self, path: &TblSubexpressionInExpressionPath, value: &AtomicTblExpression) -> HashSet<&T>
        { get_helper(&self.0, path, value) }
    
    pub fn insert(&mut self, path: &TblSubexpressionInExpressionPath, atom: AtomicTblExpression, value: T) -> bool 
        { insert_helper(&mut self.0, path, atom, value) }
    pub fn insert2(&mut self, bound: TblExpressionBoundAtomExactValue, value: T) -> bool
        { self.insert(&bound.path, bound.value, value) }
}
