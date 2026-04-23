use crate::{propositions::{Proposition, bounds::{GetBoundsForPropIdenticalToProp, InsertBoundsForProp}}, utils::collections::binders::{Binder, InsertBinder}};

// Feature: generation
pub mod unassigned;

pub trait GetBinderForPropIdenticalToProp<PE: Proposition>: Binder {
    type DefaultGetBoundsForPropIdenticalToProp<'prop>: GetBoundsForPropIdenticalToProp<'prop,PE,Self> where PE: 'prop;

    #[inline]
    fn get_identical_to<'prop,'binder>(&'binder self, prop: &'prop PE) -> Option<&'binder Self::Value>
        { self.get_unique_by_bounds(&Self::DefaultGetBoundsForPropIdenticalToProp::from(prop)) }
}

pub trait InsertBinderForProp<'prop,PE: 'prop + Proposition>: InsertBinder<Self::DefaultInsertionBounds> {
    type DefaultInsertionBounds: InsertBoundsForProp<'prop,PE,Self>;

    // fn insert<'a, Bs: InsertBoundsForProp<'a,PE,Self>>(&'a mut self, bounds: Bs, value: Self::Value) where PE: 'a;
    fn insert_prop(&mut self, prop: &'prop PE, value: Self::Value)
        { self.insert_by_bounds(&Self::DefaultInsertionBounds::from(prop), value) }
}
