use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::utils::{collections::{binding::binders::{Binder, GetBinder}, maps::multimap::MultiMap, sets::hashset::transform_hashset}, traits::fast_ord::FastOrd};

use crate::expressions::assigned::{binding::{binders::{get_helper, insert_helper}, bounds::TblExpressionBoundValueDuplicated}, subexpressions::TblSubexpressionInExpressionPath};

pub struct TblExpressionBinderValueDuplication<T: Hash + Eq>(HashMap<TblSubexpressionInExpressionPath,MultiMap<TblSubexpressionInExpressionPath,T>>);

impl <T: Eq + Hash> Binder for TblExpressionBinderValueDuplication<T> {
    type Value = T;

    fn get_all<'binder>(&'binder self) -> HashSet<&'binder Self::Value> {
        self.0.iter()
            .map(|(_,inner)| inner.flat_values())
            .flatten()
            .collect()
    }
}
impl <T: Eq + Hash> GetBinder<TblExpressionBoundValueDuplicated> for TblExpressionBinderValueDuplication<T> {
    fn get<'binder>(&'binder self, bound: &TblExpressionBoundValueDuplicated) -> HashSet<&'binder Self::Value>
        { get_helper(&self.0, bound.path1(), bound.path2()) }
    fn get_with_extra_data<'binder>(&'binder self, bound: &TblExpressionBoundValueDuplicated) -> HashSet<(&'binder Self::Value,())>
        { transform_hashset(self.get(bound), |v| (v,())) }
}

impl <T: Hash + Eq> TblExpressionBinderValueDuplication<T> {
    fn get_inner(&self, path1: &TblSubexpressionInExpressionPath, path2: &TblSubexpressionInExpressionPath) -> HashSet<&T> {
        let key = if path1.fast_cmp(&path2).is_lt() { (path1, path2) } else { (path2, path1) };
        get_helper(&self.0, key.0, key.1)
    }

    pub fn insert(&mut self, path1: TblSubexpressionInExpressionPath, path2: TblSubexpressionInExpressionPath, value: T) -> bool {
        let key = if path1.fast_cmp(&path2).is_lt() { (path1, path2) } else { (path2, path1) };
        insert_helper(&mut self.0, &key.0, key.1, value)
    }
    pub fn insert2(&mut self, bound: TblExpressionBoundValueDuplicated, value: T) -> bool {
        let key = bound.into_paths();
        insert_helper(&mut self.0, &key.0, key.1, value)
    }
}
