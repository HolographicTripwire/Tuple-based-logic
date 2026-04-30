use std::collections::HashSet;

use crate::utils::collections::binding::binders::{Binder, InsertBinder};

pub trait GetBound {
    type ExtraReturnData;
}

pub trait GetBounds<B: Binder>: Sized {
    fn get_from<'binder>(&self, binder: &'binder B) -> HashSet<&'binder B::Value>;
}

pub trait UniqueGetBounds<B: Binder>: GetBounds<B> {
    fn get_unique_from<'binder>(&self, binder: &'binder B) -> Option<&'binder B::Value> {
        let mut all = self.get_from(binder).into_iter();
        let first = all.next();
        debug_assert!(all.count() == 0, "<{} as UniqueGetBounds>::get_unique returned more than one value", std::any::type_name::<Self>());
        first
    }
}

pub trait InsertBounds<B: InsertBinder<Self>>: Sized
    { fn insert_into(&self, binder: &mut B, value: B::Value) { binder.insert_by_bounds(self, value); } }
impl <Bs, Br: InsertBinder<Bs>> InsertBounds<Br> for Bs {} 
