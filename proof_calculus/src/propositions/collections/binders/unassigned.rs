use std::collections::HashSet;
use crate::{propositions::{Proposition, bounds::unassigned::{GetBoundsForPropsSubsumedByUprop, GetBoundsForUpropIdenticalToUprop, GetBoundsForUpropsEquivalentToUprop, GetBoundsForUpropsSubsumedByUprop, GetBoundsForUpropsSubsumingProp, InsertBoundsForUprop}, unassigned::UnassignedProposition}, utils::collections::binders::{Binder, InsertBinder}};

pub trait GetBinderForPropsSubsumedByUprop<UPE: UnassignedProposition>: Binder {
    type DefaultBounds<'a>: GetBoundsForPropsSubsumedByUprop<'a,UPE,Self> where UPE: 'a;
    #[inline]
    fn get_subsumed_by(&self, element: &UPE) -> HashSet<&Self::Value> { self.get_by_bounds(&Self::DefaultBounds::from(element)) }
}

pub trait GetBinderForUpropIdenticalToUprop<PE: UnassignedProposition>: Binder {
    type DefaultBounds<'a>: GetBoundsForUpropIdenticalToUprop<'a,PE,Self> where PE: 'a;
    #[inline]
    fn get_identical_to(&self, element: &PE) -> Option<&Self::Value> { self.get_unique_by_bounds(&Self::DefaultBounds::from(element)) }
}
pub trait GetBinderForUpropEquivalentToUprop<PE: UnassignedProposition>: Binder {
    type DefaultBounds<'a>: GetBoundsForUpropsEquivalentToUprop<'a,PE,Self> where PE: 'a;
    #[inline]
    fn get_equivalent_to(&self, element: &PE) -> HashSet<&Self::Value> { self.get_by_bounds(&Self::DefaultBounds::from(element)) }
}
pub trait GetBinderForUpropsSubsumedByUprop<UPE: UnassignedProposition>: Binder {
    type DefaultBounds<'a>: GetBoundsForUpropsSubsumedByUprop<'a,UPE,Self> where UPE: 'a;
    #[inline]
    fn get_subsumed_by(&self, element: &UPE) -> HashSet<&Self::Value> { self.get_by_bounds(&Self::DefaultBounds::from(element)) }
}
pub trait GetBinderForUpropsSubsumingProp<PE: Proposition>: Binder {
    type DefaultBounds<'a>: GetBoundsForUpropsSubsumingProp<'a,PE,Self> where PE: 'a;
    #[inline]
    fn get_subsumers_of(&self, element: &PE) -> HashSet<&Self::Value> { self.get_by_bounds(&Self::DefaultBounds::from(element)) }
}
pub trait GetBinderForUpropsSubsumingUprop<UPE: UnassignedProposition>: Binder {
    type DefaultBounds<'a>: GetBoundsForUpropsSubsumedByUprop<'a,UPE,Self> where UPE: 'a;
    #[inline]
    fn get_subsumers_of(&self, element: &UPE) -> HashSet<&Self::Value> { self.get_by_bounds(&Self::DefaultBounds::from(element)) }
}

pub trait InsertBinderForUprop<'a,UPE: 'a + UnassignedProposition>: InsertBinder<Self::DefaultInsertionBounds> {
    type DefaultInsertionBounds: InsertBoundsForUprop<'a,UPE,Self>;

    fn insert_uprop(&mut self, uprop: &'a UPE, value: Self::Value)
        { self.insert_by_bounds(&Self::DefaultInsertionBounds::from(uprop), value) }
}
