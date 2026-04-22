use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::utils::{collections::multimap::MultiMap, traits::fast_ord::FastOrd};

use crate::expressions::assigned::{bounds::TblExpressionBoundValueDuplicated, collections::tracker::bounds::{get_helper, insert_helper}, subexpressions::TblSubexpressionInExpressionPath};

pub struct TblExpressionTrackerDuplicationBounds<T: Hash + Eq>(HashMap<TblSubexpressionInExpressionPath,MultiMap<TblSubexpressionInExpressionPath,T>>);

impl <T: Hash + Eq> TblExpressionTrackerDuplicationBounds<T> {
    pub fn get(&self, path1: &TblSubexpressionInExpressionPath, path2: &TblSubexpressionInExpressionPath) -> HashSet<&T> {
        let key = if path1.fast_cmp(&path2).is_lt() { (path1, path2) } else { (path2, path1) };
        get_helper(&self.0, key.0, key.1)
    }
    pub fn get2(&self, bound: &TblExpressionBoundValueDuplicated) -> HashSet<&T> { get_helper(&self.0, bound.path1(), bound.path2()) }

    pub fn insert(&mut self, path1: TblSubexpressionInExpressionPath, path2: TblSubexpressionInExpressionPath, value: T) -> bool {
        let key = if path1.fast_cmp(&path2).is_lt() { (path1, path2) } else { (path2, path1) };
        insert_helper(&mut self.0, &key.0, key.1, value)
    }
    pub fn insert2(&mut self, bound: TblExpressionBoundValueDuplicated, value: T) -> bool {
        let key = bound.into_paths();
        insert_helper(&mut self.0, &key.0, key.1, value)
    }
}
