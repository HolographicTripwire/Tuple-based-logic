use std::collections::{HashMap, HashSet};

use crate::expressions::{assigned::{RcTblExpression, collections::tracker::TblExpressionTracker}, unassigned::{UnassignedTblExpression, compound::UnassignedCompoundTblExpression}};

mod atomic;

pub use atomic::AtomicTblExpressionMap;

pub struct TblExpressionMap<V> {
    values: HashMap<RcTblExpression,V>,
    tracker: TblExpressionTracker<RcTblExpression>
}

impl <V> TblExpressionMap<V> {
    // fn get_identical<C: CompoundTblExpression>(expr: TblExpression<C>) -> Option<RcCompoundTblExpression> {

    // }

    fn insert(&mut self, key: RcTblExpression, value: V) {
        self.tracker.insert(&key, &key);
        self.values.insert(key, value);
    }

    fn get_subsumed_by<C: UnassignedCompoundTblExpression>(expr: UnassignedTblExpression<C>) -> HashSet<> {

    }
}
