use crate::{propositions::{Proposition, bounds::{GetBoundsForPropIdenticalToProp, InsertBoundsForProp}}, utils::collections::binders::{Binder, InsertBinder}};

// Feature: generation
pub mod unassigned;

pub trait GetBinderForPropIdenticalToProp<PE: Proposition>: Binder {
    type DefaultBounds<'a>: GetBoundsForPropIdenticalToProp<'a,PE,Self> where PE: 'a;
    #[inline]
    fn get_identical_to(&self, element: &PE) -> Option<&Self::Value> { self.get_unique_by_bounds(&Self::DefaultBounds::from(element)) }
}

pub trait InsertBinderForProp<'a,PE: 'a + Proposition>: InsertBinder<Self::DefaultInsertionBounds> {
    type DefaultInsertionBounds: InsertBoundsForProp<'a,PE,Self>;

    // fn insert<'a, Bs: InsertBoundsForProp<'a,PE,Self>>(&'a mut self, bounds: Bs, value: Self::Value) where PE: 'a;
    fn insert_prop(&mut self, prop: &'a PE, value: Self::Value)
        { self.insert_by_bounds(&Self::DefaultInsertionBounds::from(prop), value) }
}
