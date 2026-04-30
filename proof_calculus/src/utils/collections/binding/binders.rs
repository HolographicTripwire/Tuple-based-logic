use std::hash::Hash;
use std::collections::HashSet;

use crate::utils::collections::binding::bounds::{GetBound, GetBounds, UniqueGetBounds};

pub trait Binder: Sized {
    type Value: Eq + Hash;

    fn get_all<'binder>(&'binder self) -> HashSet<&'binder Self::Value>;
    #[inline]
    fn get_by_bounds<'binder,B: GetBounds<Self>>(&'binder self, bounds: &B) -> HashSet<&'binder Self::Value>
        { bounds.get_from(self) }
    #[inline]
    fn get_unique_by_bounds<'binder,B: UniqueGetBounds<Self>>(&'binder self, bounds: &B) -> Option<&'binder Self::Value>
        { bounds.get_unique_from(self) }
}

pub trait GetBinder<B: GetBound>: Binder {
    fn get<'binder>(&'binder self, bound: &B) -> HashSet<&'binder Self::Value>;
    fn get_with_extra_data<'binder>(&'binder self, bound: &B) -> HashSet<(&'binder Self::Value,B::ExtraReturnData)>;

    fn get_intersection<'binder, 'bounds, I: IntoIterator<Item=&'bounds B>>(&'binder self, bounds: I) -> HashSet<&'binder Self::Value> where B: 'bounds {
        let mut iter = bounds.into_iter();
        if let Some(value) = iter.next() {
            let mut results = self.get(value);
            for bound in iter {
                if results.len() == 0 { break; }
                results = results.intersection(&self.get(bound)).map(|v| *v).collect();
            }
            results
        } else { self.get_all() }
    }
}


pub trait InsertBinder<B>: Binder {
    fn insert_by_bounds(&mut self, bounds: &B, value: Self::Value);
}

