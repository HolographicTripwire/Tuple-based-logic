use std::collections::HashSet;

use crate::{propositions::{assignments::{PartialPropositionalAssignment, PartialPropositionalAssignmentConstructor}, types::{assigned::Proposition, unassigned::UnassignedProposition}}, utils::collections::binding::{binders::{Binder, InsertBinder}, bounds::{GetBounds, InsertBounds, UniqueGetBounds}}};

pub trait GetBoundsForUpropIdenticalToUprop<'elem, ElemUprop: 'elem + UnassignedProposition, B: Binder>: UniqueGetBounds<B> + From<&'elem ElemUprop> {}
pub trait GetBoundsForUpropsEquivalentToUprop<'elem, ElemUprop: 'elem + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'elem ElemUprop> {}
pub trait GetBoundsForConstructibleUpropsEquivalentToUprop<'slf,'elem,MapUprop: UnassignedProposition, ElemUprop:'elem + UnassignedProposition,B: Binder,Assignment: PartialPropositionalAssignment<'slf,'elem,MapUprop,ElemUprop>>: GetBoundsForUpropsEquivalentToUprop<'elem,ElemUprop,B> {
    type AssignmentConstructor: PartialPropositionalAssignmentConstructor<'slf,'elem,MapUprop,ElemUprop,Assignment>;
    fn get_from_with_assignment_constructors<'binder>(&'slf self, binder: &'binder B) -> HashSet<(&'binder B::Value, Self::AssignmentConstructor)>;
}

pub trait GetBoundsForUpropsSubsumingProp<'elem, ElemProp: 'elem + Proposition, B: Binder>: GetBounds<B> + From<&'elem ElemProp> {}
pub trait GetBoundsForUpropsSubsumedByUprop<'elem, ElemUprop: 'elem + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'elem ElemUprop> {}
pub trait GetBoundsForUpropsSubsumingByUprop<'elem, ElemUprop: 'elem + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'elem ElemUprop> {}

pub trait InsertBoundsForUprop<'elem,ElemUprop: 'elem + UnassignedProposition, B: InsertBinder<Self>>: InsertBounds<B> + From<&'elem ElemUprop> {}
