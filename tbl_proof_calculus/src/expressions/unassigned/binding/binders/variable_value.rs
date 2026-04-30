use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::utils::collections::{binding::binders::{Binder, GetBinder}, maps::multimap::MultiMap, sets::hashset::transform_hashset};

use crate::expressions::{assigned::{binding::binders::{get_helper, insert_helper}, subexpressions::TblSubexpressionInExpressionPath}, unassigned::{binding::bounds::{UnassignedTblExpressionBoundVariableExactValue, UnassignedTblExpressionBoundVariableExistsAtLocation}, variable::TblExpressionVariable}};

pub struct TblExpressionBinderVariableExactValue<T: Hash>(HashMap<TblSubexpressionInExpressionPath,MultiMap<TblExpressionVariable,T>>);

impl <T: Eq + Hash> Binder for TblExpressionBinderVariableExactValue<T> {
    type Value = T;

    fn get_all<'binder>(&'binder self) -> HashSet<&'binder Self::Value> {
        self.0.iter()
            .map(|(_,inner)| inner.flat_values())
            .flatten()
            .collect()
    }
}
impl <T: Eq + Hash> GetBinder<UnassignedTblExpressionBoundVariableExactValue> for TblExpressionBinderVariableExactValue<T> {
    fn get<'binder>(&'binder self, bound: &UnassignedTblExpressionBoundVariableExactValue) -> HashSet<&'binder Self::Value>
        { self.get_inner(&bound.path,&bound.value) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblExpressionBoundVariableExactValue) -> HashSet<(&'binder Self::Value,())>
        { transform_hashset(self.get_inner(&bound.path,&bound.value), |v| (v,())) }
}
impl <T: Eq + Hash> GetBinder<UnassignedTblExpressionBoundVariableExistsAtLocation> for TblExpressionBinderVariableExactValue<T> {
    fn get<'binder>(&'binder self, bound: &UnassignedTblExpressionBoundVariableExistsAtLocation) -> HashSet<&'binder Self::Value> {
        match self.0.get(&bound.path) {
            Some(inner) =>  inner.flat_values().into_iter().collect(),
            None => HashSet::new(),
        }
    }
    fn get_with_extra_data<'binder>(&'binder self, bound: &UnassignedTblExpressionBoundVariableExistsAtLocation) -> HashSet<(&'binder Self::Value,TblExpressionVariable)> {
        match self.0.get(&bound.path) {
            Some(inner) => inner.pairs().into_iter().map(|(atom,value)| (value,*atom)).collect(),
            None => HashSet::new(),
        }
    }
}

impl <T: Hash + Eq> TblExpressionBinderVariableExactValue<T> {
    fn get_inner(&self, path: &TblSubexpressionInExpressionPath, value: &TblExpressionVariable) -> HashSet<&T>
        { get_helper(&self.0, path, value) }
    
    pub fn insert(&mut self, path: &TblSubexpressionInExpressionPath, atom: TblExpressionVariable, value: T) -> bool 
        { insert_helper(&mut self.0, path, atom, value) }
    pub fn insert2(&mut self, bound: UnassignedTblExpressionBoundVariableExactValue, value: T) -> bool
        { self.insert(&bound.path, bound.value, value) }
}
