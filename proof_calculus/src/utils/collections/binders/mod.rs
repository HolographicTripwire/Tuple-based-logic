use std::{collections::HashSet, hash::Hash};

pub trait Binder: Sized {
    type Value: Eq + Hash;

    fn get_all<'binder>(&'binder self) -> HashSet<&'binder Self::Value>;
    #[inline]
    fn get_by_bounds<'bounds,'binder,B: GetBounds<Self>>(&'binder self, bounds: &'binder B) -> HashSet<&'binder Self::Value>
        { bounds.get_from(self) }
    #[inline]
    fn get_unique_by_bounds<'binder,B: UniqueGetBounds<Self>>(&'binder self, bounds: &B) -> Option<&'binder Self::Value>
        { bounds.get_unique_from(self) }
}

pub trait GetBinder<B>: Binder {
    fn get<'binder>(&'binder self, key: &B) -> HashSet<&'binder Self::Value>;

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

pub trait InsertBinder<B>: Binder {
    fn insert_by_bounds(&mut self, bounds: &B, value: Self::Value);
}
pub trait InsertBounds<B: InsertBinder<Self>>: Sized
    { fn insert_into(&self, binder: &mut B, value: B::Value) { binder.insert_by_bounds(self, value); } }
impl <Bs, Br: InsertBinder<Bs>> InsertBounds<Br> for Bs {} 
