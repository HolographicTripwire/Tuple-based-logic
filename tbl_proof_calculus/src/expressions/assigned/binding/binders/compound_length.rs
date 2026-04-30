use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::utils::collections::{binding::binders::{Binder, GetBinder}, maps::multimap::MultiMap, sets::hashset::transform_hashset};

use crate::expressions::assigned::{binding::{binders::{get_helper, insert_helper}, bounds::{TblExpressionBoundCompoundExactLength, TblExpressionBoundCompoundExistsAtLocation}}, subexpressions::TblSubexpressionInExpressionPath};

pub struct TblExpressionBinderCompoundExactLength<T: Hash + Eq>(HashMap<TblSubexpressionInExpressionPath,MultiMap<usize,T>>);

impl <T: Eq + Hash> Binder for TblExpressionBinderCompoundExactLength<T> {
    type Value = T;

    fn get_all<'binder>(&'binder self) -> HashSet<&'binder Self::Value> {
        self.0.iter()
            .map(|(_,inner)| inner.flat_values())
            .flatten()
            .collect()
    }
}
impl <T: Eq + Hash> GetBinder<TblExpressionBoundCompoundExactLength> for TblExpressionBinderCompoundExactLength<T> {
    fn get<'binder>(&'binder self, bound: &TblExpressionBoundCompoundExactLength) -> HashSet<&'binder Self::Value>
        { self.get_inner(&bound.path,bound.length) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblExpressionBoundCompoundExactLength) -> HashSet<(&'binder Self::Value,())>
        { transform_hashset(self.get(bound), |v| (v,())) }
}
impl <T: Eq + Hash> GetBinder<TblExpressionBoundCompoundExistsAtLocation> for TblExpressionBinderCompoundExactLength<T> {
    fn get<'binder>(&'binder self, bound: &TblExpressionBoundCompoundExistsAtLocation) -> HashSet<&'binder Self::Value> {
        match self.0.get(&bound.path) {
            Some(inner) =>  inner.flat_values().into_iter().collect(),
            None => HashSet::new(),
        }
    }
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblExpressionBoundCompoundExistsAtLocation) -> HashSet<(&'binder Self::Value,usize)> {
        match self.0.get(&bound.path) {
            Some(inner) => inner.pairs().into_iter().map(|(length,value)| (value,*length)).collect(),
            None => HashSet::new(),
        }
    }
}

impl <T: Hash + Eq> TblExpressionBinderCompoundExactLength<T> {
    fn get_inner(&self, path: &TblSubexpressionInExpressionPath, length: usize) -> HashSet<&T>
        { get_helper(&self.0, path, &length) }

    pub fn insert(&mut self, path: &TblSubexpressionInExpressionPath, length: usize, value: T) -> bool 
        { insert_helper(&mut self.0, path, length, value) }
    pub fn insert2(&mut self, bound: TblExpressionBoundCompoundExactLength, value: T) -> bool
        { self.insert(&bound.path, bound.length, value) }
}
