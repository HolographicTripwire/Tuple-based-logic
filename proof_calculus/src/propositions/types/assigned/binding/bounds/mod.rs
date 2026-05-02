use std::collections::HashSet;

use crate::{propositions::{assignments::{PropositionalAssignment, PropositionalAssignmentConstructor}, types::{assigned::Proposition, unassigned::UnassignedProposition}}, utils::collections::binding::{binders::{Binder, InsertBinder},bounds::{GetBounds, InsertBounds, UniqueGetBounds}}};

pub trait GetBoundsForPropIdenticalToProp<'elem, ElemProp: 'elem + Proposition, B: Binder>: UniqueGetBounds<B> + From<&'elem ElemProp> {}
pub trait InsertBoundsForProp<'elem, ElemProp: 'elem + Proposition, B: InsertBinder<Self>>: InsertBounds<B> + From<&'elem ElemProp> {}

pub trait GetBoundsForPropsSubsumedByUprop<'elem, ElemUprop: 'elem + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'elem ElemUprop> {}
pub trait GetBoundsForConstructiblePropsSubsumedByUprop<'uprop,MapProp: Proposition,ElemUprop:'uprop + UnassignedProposition,Assignment: PropositionalAssignment<ElemUprop,MapProp>, B: Binder>: GetBoundsForPropsSubsumedByUprop<'uprop,ElemUprop,B> {
    type AssignmentConstructor: PropositionalAssignmentConstructor<ElemUprop,MapProp,Assignment>;
    fn get_from_with_assignment_constructors<'binder>(&self, binder: &'binder B) -> HashSet<(&'binder B::Value, Self::AssignmentConstructor)>;
}
