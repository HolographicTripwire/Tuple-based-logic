use std::collections::HashSet;

use crate::{
    propositions::types::{
        assigned::Proposition,
        unassigned::{
            UnassignedProposition,
            binding::bounds::{
                GetBoundsForUpropIdenticalToUprop, GetBoundsForUpropsEquivalentToUprop,
                GetBoundsForUpropsSubsumedByUprop, GetBoundsForUpropsSubsumingProp,
                InsertBoundsForUprop,
            },
        },
    },
    utils::collections::binding::binders::{Binder, InsertBinder},
};

pub trait GetBinderForUpropIdenticalToUprop<ElemUprop: UnassignedProposition>: Binder {
    type DefaultGetBoundsForPropIdenticalToProp<'elem>: GetBoundsForUpropIdenticalToUprop<'elem, ElemUprop, Self>
    where
        ElemUprop: 'elem;

    fn get_identical_to<'prop, 'binder>(
        &'binder self,
        prop: &'prop ElemUprop,
    ) -> Option<&'binder Self::Value> {
        self.get_unique_by_bounds(&Self::DefaultGetBoundsForPropIdenticalToProp::from(prop))
    }
}
pub trait GetBinderForUpropEquivalentToUprop<ElemUprop: UnassignedProposition>: Binder {
    type DefaultGetBoundsUpropsEquivalentToUprop<'elem,'bounds,'binder>: GetBoundsForUpropsEquivalentToUprop<'elem,ElemUprop,Self> where ElemUprop: 'elem;

    fn get_equivalent_to(&self, element: &ElemUprop) -> HashSet<&Self::Value> {
        self.get_by_bounds(&Self::DefaultGetBoundsUpropsEquivalentToUprop::from(
            element,
        ))
    }
}
pub trait GetBinderForUpropsSubsumedByUprop<SubsumerElemUprop: UnassignedProposition>:
    Binder
{
    type DefaultGetBoundsForUpropsSubsumedByUprop<'elem,'bounds,'binder>: GetBoundsForUpropsSubsumedByUprop<'elem,SubsumerElemUprop,Self> where SubsumerElemUprop: 'elem;

    fn get_subsumed_by(&self, element: &SubsumerElemUprop) -> HashSet<&Self::Value> {
        self.get_by_bounds(&Self::DefaultGetBoundsForUpropsSubsumedByUprop::from(
            element,
        ))
    }
}
pub trait GetBinderForUpropsSubsumingProp<SubsumedElemProp: Proposition>: Binder {
    type DefaultGetBoundsForUpropsSubsumingProp<'elem,'bounds,'binder>: GetBoundsForUpropsSubsumingProp<'elem,SubsumedElemProp,Self> where SubsumedElemProp: 'elem;

    fn get_subsumers_of(&self, element: &SubsumedElemProp) -> HashSet<&Self::Value> {
        self.get_by_bounds(&Self::DefaultGetBoundsForUpropsSubsumingProp::from(element))
    }
}
pub trait GetBinderForUpropsSubsumingUprop<SubsumedElemUprop: UnassignedProposition>:
    Binder
{
    type DefaultGetBoundsForUpropsSubsumedByUprop<'elem,'bounds,'binder>: GetBoundsForUpropsSubsumedByUprop<'elem,SubsumedElemUprop,Self> where SubsumedElemUprop: 'elem;

    fn get_subsumers_of(&self, element: &SubsumedElemUprop) -> HashSet<&Self::Value> {
        self.get_by_bounds(&Self::DefaultGetBoundsForUpropsSubsumedByUprop::from(
            element,
        ))
    }
}

pub trait InsertBinderForUprop<'elem, ElemUprop: 'elem + UnassignedProposition>:
    InsertBinder<Self::DefaultInsertionBounds>
{
    type DefaultInsertionBounds: InsertBoundsForUprop<'elem, ElemUprop, Self>;

    fn insert_uprop(&mut self, uprop: &'elem ElemUprop, value: Self::Value) {
        self.insert_by_bounds(&Self::DefaultInsertionBounds::from(uprop), value)
    }
}
