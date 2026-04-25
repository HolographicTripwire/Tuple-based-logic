use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::utils::collections::multimap::MultiMap;

use crate::expressions::assigned::{atomic::AtomicTblExpression, binding::{binders::{get_helper, insert_helper}, bounds::TblExpressionBoundAtomExactValue}, subexpressions::TblSubexpressionInExpressionPath};

pub struct TblExpressionBinderAtomExactValue<T: Hash>(HashMap<TblSubexpressionInExpressionPath,MultiMap<AtomicTblExpression,T>>);

impl <T: Hash + Eq> TblExpressionBinderAtomExactValue<T> {
    #[inline]
    pub fn get(&self, path: &TblSubexpressionInExpressionPath, value: &AtomicTblExpression) -> HashSet<&T>
        { get_helper(&self.0, path, value) }
    #[inline]
    pub fn get2(&self, bound: &TblExpressionBoundAtomExactValue) -> HashSet<&T> { self.get(&bound.path,&bound.value) }
    
    pub fn get_no_bound(&self, path: &TblSubexpressionInExpressionPath) -> Option<impl IntoIterator<Item = &T>> {
        self.0.get(path)
            .map(|inner| inner.flat_values())
    }

    #[inline]
    pub fn insert(&mut self, path: &TblSubexpressionInExpressionPath, atom: AtomicTblExpression, value: T) -> bool 
        { insert_helper(&mut self.0, path, atom, value) }
    #[inline]
    pub fn insert2(&mut self, bound: TblExpressionBoundAtomExactValue, value: T) -> bool
        { self.insert(&bound.path, bound.value, value) }
}
