use std::collections::HashSet;

use crate::{propositions::{assigned::Proposition, unassigned::{UnassignedProposition, binding::bounds::{GetBoundsForUpropIdenticalToUprop, GetBoundsForUpropsEquivalentToUprop, GetBoundsForUpropsSubsumedByUprop, GetBoundsForUpropsSubsumingProp, InsertBoundsForUprop}}}, utils::collections::binding::binders::{Binder, InsertBinder}};

pub trait GetBinderForUpropIdenticalToUprop<UPE: UnassignedProposition>: Binder {
    type DefaultGetBoundsForPropIdenticalToProp<'prop>: GetBoundsForUpropIdenticalToUprop<'prop,UPE,Self> where UPE: 'prop;
    #[inline]
    fn get_identical_to<'prop,'binder>(&'binder self, prop: &'prop UPE) -> Option<&'binder Self::Value>
        { self.get_unique_by_bounds(&Self::DefaultGetBoundsForPropIdenticalToProp::from(prop)) }
}
pub trait GetBinderForUpropEquivalentToUprop<PE: UnassignedProposition>: Binder {
    type DefaultGetBoundsUpropsEquivalentToUprop<'prop,'bounds,'binder>: GetBoundsForUpropsEquivalentToUprop<'prop,PE,Self> where PE: 'prop;
    #[inline]
    fn get_equivalent_to(&self, element: &PE) -> HashSet<&Self::Value> { self.get_by_bounds(&Self::DefaultGetBoundsUpropsEquivalentToUprop::from(element)) }
}
pub trait GetBinderForUpropsSubsumedByUprop<UPE: UnassignedProposition>: Binder {
    type DefaultGetBoundsForUpropsSubsumedByUprop<'prop,'bounds,'binder>: GetBoundsForUpropsSubsumedByUprop<'prop,UPE,Self> where UPE: 'prop;
    #[inline]
    fn get_subsumed_by(&self, element: &UPE) -> HashSet<&Self::Value> { self.get_by_bounds(&Self::DefaultGetBoundsForUpropsSubsumedByUprop::from(element)) }
}
pub trait GetBinderForUpropsSubsumingProp<PE: Proposition>: Binder {
    type DefaultGetBoundsForUpropsSubsumingProp<'prop,'bounds,'binder>: GetBoundsForUpropsSubsumingProp<'prop,PE,Self> where PE: 'prop;
    #[inline]
    fn get_subsumers_of(&self, element: &PE) -> HashSet<&Self::Value> { self.get_by_bounds(&Self::DefaultGetBoundsForUpropsSubsumingProp::from(element)) }
}
pub trait GetBinderForUpropsSubsumingUprop<UPE: UnassignedProposition>: Binder {
    type DefaultGetBoundsForUpropsSubsumedByUprop<'prop,'bounds,'binder>: GetBoundsForUpropsSubsumedByUprop<'prop,UPE,Self> where UPE: 'prop;
    #[inline]
    fn get_subsumers_of(&self, element: &UPE) -> HashSet<&Self::Value> { self.get_by_bounds(&Self::DefaultGetBoundsForUpropsSubsumedByUprop::from(element)) }
}

pub trait InsertBinderForUprop<'a,UPE: 'a + UnassignedProposition>: InsertBinder<Self::DefaultInsertionBounds> {
    type DefaultInsertionBounds: InsertBoundsForUprop<'a,UPE,Self>;

    fn insert_uprop(&mut self, uprop: &'a UPE, value: Self::Value)
        { self.insert_by_bounds(&Self::DefaultInsertionBounds::from(uprop), value) }
}
