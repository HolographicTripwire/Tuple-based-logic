use std::collections::HashSet;

use crate::{propositions::{assignments::PropositionalAssignment, types::{assigned::{Proposition, binding::bounds::{GetBoundsForConstructiblePropsSubsumedByUprop, GetBoundsForPropIdenticalToProp, GetBoundsForPropsSubsumedByUprop, InsertBoundsForProp}}, unassigned::UnassignedProposition}}, utils::collections::binding::binders::{Binder, InsertBinder}};

pub trait GetBinderForPropIdenticalToProp<ElemProp: Proposition>: Binder {
    type DefaultGetBoundsForPropIdenticalToProp<'elem>: GetBoundsForPropIdenticalToProp<'elem,ElemProp,Self> where ElemProp: 'elem;
    fn get_identical_to<'prop,'binder>(&'binder self, prop: &'prop ElemProp) -> Option<&'binder Self::Value>
        { self.get_unique_by_bounds(&Self::DefaultGetBoundsForPropIdenticalToProp::from(prop)) }
}
pub trait GetBinderForPropsSubsumedByUprop<SubsumerElemUprop: UnassignedProposition>: Binder {
    type DefaultGetBoundsForPropsSubsumedByUprop<'elem>: GetBoundsForPropsSubsumedByUprop<'elem,SubsumerElemUprop,Self> where SubsumerElemUprop: 'elem;
    fn get_subsumed_by<'prop,'binder>(&'binder self, element: &'prop SubsumerElemUprop) -> HashSet<&'binder Self::Value>
        { self.get_by_bounds(&Self::DefaultGetBoundsForPropsSubsumedByUprop::from(element)) }
    fn get_subsumed_by_with_elem_to_map_constructor<'uprop,'binder,SubsumedMapProp:Proposition,Assignment:PropositionalAssignment<SubsumerElemUprop,SubsumedMapProp>>(&'binder self, element: &'uprop SubsumerElemUprop) -> 
        HashSet<(&'binder Self::Value,<Self::DefaultGetBoundsForPropsSubsumedByUprop<'uprop> as GetBoundsForConstructiblePropsSubsumedByUprop<'uprop,SubsumedMapProp,SubsumerElemUprop,Assignment,Self>>::ElemToMapConstructor)>
        where Self::DefaultGetBoundsForPropsSubsumedByUprop<'uprop>: GetBoundsForConstructiblePropsSubsumedByUprop<'uprop,SubsumedMapProp,SubsumerElemUprop,Assignment,Self>
        { Self::DefaultGetBoundsForPropsSubsumedByUprop::from(element).get_from_with_elem_to_map_constructors(self) }
}

pub trait InsertBinderForProp<'elem,ElemProp: 'elem + Proposition>: InsertBinder<Self::DefaultInsertionBounds> {
    type DefaultInsertionBounds: InsertBoundsForProp<'elem,ElemProp,Self>;

    // fn insert<'a, Bs: InsertBoundsForProp<'a,PE,Self>>(&'a mut self, bounds: Bs, value: Self::Value) where PE: 'a;
    fn insert_prop(&mut self, prop: &'elem ElemProp, value: Self::Value)
        { self.insert_by_bounds(&Self::DefaultInsertionBounds::from(prop), value) }
}
