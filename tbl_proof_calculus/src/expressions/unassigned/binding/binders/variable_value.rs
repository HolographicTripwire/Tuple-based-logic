use std::{collections::{HashMap, HashSet}, hash::Hash};

use proof_calculus::utils::collections::multimap::MultiMap;

use crate::expressions::{assigned::{binding::binders::{get_helper, insert_helper}, subexpressions::TblSubexpressionInExpressionPath}, unassigned::{binding::bounds::TblExpressionBoundVariableExactValue, variable::TblExpressionVariable}};

pub struct TblExpressionBinderVariableExactValue<T: Hash>(HashMap<TblSubexpressionInExpressionPath,MultiMap<TblExpressionVariable,T>>);

impl <T: Hash + Eq> TblExpressionBinderVariableExactValue<T> {
    #[inline]
    pub fn get(&self, path: &TblSubexpressionInExpressionPath, value: &TblExpressionVariable) -> HashSet<&T>
        { get_helper(&self.0, path, value) }
    #[inline]
    pub fn get2(&self, bound: &TblExpressionBoundVariableExactValue) -> HashSet<&T> { self.get(&bound.path,&bound.value) }
    
    pub fn get_no_bound(&self, path: &TblSubexpressionInExpressionPath) -> Option<impl IntoIterator<Item = &T>> {
        self.0.get(path)
            .map(|inner| inner.flat_values())
    }

    #[inline]
    pub fn insert(&mut self, path: &TblSubexpressionInExpressionPath, atom: TblExpressionVariable, value: T) -> bool 
        { insert_helper(&mut self.0, path, atom, value) }
    #[inline]
    pub fn insert2(&mut self, bound: TblExpressionBoundVariableExactValue, value: T) -> bool
        { self.insert(&bound.path, bound.value, value) }
}
