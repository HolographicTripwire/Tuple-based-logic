use crate::{propositions::{Proposition, bounds::{GetBoundsForPropIdenticalToProp, InsertBoundsForProp}}, utils::collections::binders::{Binder, InsertBinder, UniqueGetBounds}};

// Feature: generation
pub mod unassigned;

pub trait GetBinderForPropIdenticalToProp<PE: Proposition>: Binder {
    type DefaultGetBoundsForPropIdenticalToProp<'a>: GetBoundsForPropIdenticalToProp<'a,PE,Self> where PE: 'a;
    #[inline]
    fn get_identical_to<'a>(&'a self, element: &'a PE) -> Option<&'a Self::Value> {
        let x = Self::DefaultGetBoundsForPropIdenticalToProp::from(element);
        let y: Option<&<Self as Binder>::Value> = self.get_unique_by_bounds(&x);
        None
    }
}

pub trait InsertBinderForProp<'a,PE: 'a + Proposition>: InsertBinder<Self::DefaultInsertionBounds> {
    type DefaultInsertionBounds: InsertBoundsForProp<'a,PE,Self>;

    // fn insert<'a, Bs: InsertBoundsForProp<'a,PE,Self>>(&'a mut self, bounds: Bs, value: Self::Value) where PE: 'a;
    fn insert_prop(&mut self, prop: &'a PE, value: Self::Value)
        { self.insert_by_bounds(&Self::DefaultInsertionBounds::from(prop), value) }
}
