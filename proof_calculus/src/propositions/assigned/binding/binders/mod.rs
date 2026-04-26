use std::collections::HashSet;

use crate::{propositions::{assigned::{Proposition, binding::bounds::{GetBoundsForPropIdenticalToProp, GetBoundsForPropsSubsumedByUprop, InsertBoundsForProp}}, unassigned::UnassignedProposition}, utils::collections::binders::{Binder, InsertBinder}};


pub trait GetBinderForPropIdenticalToProp<PE: Proposition>: Binder {
    type DefaultGetBoundsForPropIdenticalToProp<'prop>: GetBoundsForPropIdenticalToProp<'prop,PE,Self> where PE: 'prop;

    #[inline]
    fn get_identical_to<'prop,'binder>(&'binder self, prop: &'prop PE) -> Option<&'binder Self::Value>
        { self.get_unique_by_bounds(&Self::DefaultGetBoundsForPropIdenticalToProp::from(prop)) }
}
// Feature: Generation
pub trait GetBinderForPropsSubsumedByUprop<UPE: UnassignedProposition>: Binder {
    type DefaultGetBoundsForPropsSubsumedByUprop<'prop>: GetBoundsForPropsSubsumedByUprop<'prop,UPE,Self> where UPE: 'prop;
    #[inline]
    fn get_subsumed_by<'prop,'binder>(&'binder self, element: &'prop UPE) -> HashSet<&'binder Self::Value>
        { self.get_by_bounds(&Self::DefaultGetBoundsForPropsSubsumedByUprop::from(element)) }
}

pub trait InsertBinderForProp<'prop,PE: 'prop + Proposition>: InsertBinder<Self::DefaultInsertionBounds> {
    type DefaultInsertionBounds: InsertBoundsForProp<'prop,PE,Self>;

    // fn insert<'a, Bs: InsertBoundsForProp<'a,PE,Self>>(&'a mut self, bounds: Bs, value: Self::Value) where PE: 'a;
    fn insert_prop(&mut self, prop: &'prop PE, value: Self::Value)
        { self.insert_by_bounds(&Self::DefaultInsertionBounds::from(prop), value) }
}
