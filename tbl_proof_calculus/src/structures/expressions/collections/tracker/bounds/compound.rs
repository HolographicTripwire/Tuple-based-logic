use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::utils::collections::multimap::MultiMap;

use crate::structures::expressions::{bounds::TblExpressionBoundCompoundExactLength, collections::tracker::bounds::{get_helper, insert_helper}, subexpressions::TblSubexpressionInExpressionPath};

pub struct TblExpressionTrackerCompoundLengthBounds<T: Hash + Eq>(HashMap<TblSubexpressionInExpressionPath,MultiMap<usize,T>>);

impl <T: Hash + Eq> TblExpressionTrackerCompoundLengthBounds<T> {
    #[inline]
    pub fn get(&self, path: &TblSubexpressionInExpressionPath, length: usize) -> HashSet<&T>
        { get_helper(&self.0, path, &length) }
    #[inline]
    pub fn get2(&self, bound: &TblExpressionBoundCompoundExactLength) -> HashSet<&T> { self.get(&bound.path, bound.length)}

    pub fn get_no_bound(&self, path: &TblSubexpressionInExpressionPath) -> Option<impl IntoIterator<Item = &T>> {
        self.0.get(path)
            .map(|inner| inner.flat_values())
    }

    #[inline]
    pub fn insert(&mut self, path: &TblSubexpressionInExpressionPath, length: usize, value: T) -> bool 
        { insert_helper(&mut self.0, path, length, value) }
    #[inline]
    pub fn insert2(&mut self, bound: TblExpressionBoundCompoundExactLength, value: T) -> bool
        { self.insert(&bound.path, bound.length, value) }
}
