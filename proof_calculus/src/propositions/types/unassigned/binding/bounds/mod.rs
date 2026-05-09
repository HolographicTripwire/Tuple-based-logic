use std::collections::HashSet;

use crate::{
    propositions::{
        assignments::{
            PartialPropositionalAssignment, PartialPropositionalAssignmentConstructor,
            PropositionalAssignment, PropositionalAssignmentConstructor,
        },
        types::{assigned::Proposition, unassigned::UnassignedProposition},
    },
    utils::collections::binding::{
        binders::{Binder, InsertBinder},
        bounds::{GetBounds, InsertBounds, UniqueGetBounds},
    },
};

pub trait GetBoundsForUpropIdenticalToUprop<
    'elem,
    ElemUprop: 'elem + UnassignedProposition,
    B: Binder,
>: UniqueGetBounds<B> + From<&'elem ElemUprop>
{
}
pub trait GetBoundsForUpropsEquivalentToUprop<
    'elem,
    ElemUprop: 'elem + UnassignedProposition,
    B: Binder,
>: GetBounds<B> + From<&'elem ElemUprop>
{
}
pub trait GetBoundsForConstructibleUpropsEquivalentToUprop<
    'elem,
    MapUprop: UnassignedProposition,
    ElemUprop: 'elem + UnassignedProposition,
    B: Binder,
    ElemToMapAssignment: PartialPropositionalAssignment<ElemUprop, MapUprop>,
    MapToElemAssignment: PartialPropositionalAssignment<MapUprop, ElemUprop>,
>: GetBoundsForUpropsEquivalentToUprop<'elem, ElemUprop, B>
{
    type ElemToMapConstructor: PartialPropositionalAssignmentConstructor<ElemUprop, MapUprop, ElemToMapAssignment>;
    fn get_from_with_elem_to_map_constructors<'binder>(
        &self,
        binder: &'binder B,
    ) -> HashSet<(&'binder B::Value, Self::ElemToMapConstructor)>;
    type MapToElemConstructor: PartialPropositionalAssignmentConstructor<MapUprop, ElemUprop, MapToElemAssignment>;
    fn get_from_with_map_to_elem_constructors<'binder>(
        &self,
        binder: &'binder B,
    ) -> HashSet<(&'binder B::Value, Self::MapToElemConstructor)>;
}

pub trait GetBoundsForUpropsSubsumingProp<'elem, ElemProp: 'elem + Proposition, B: Binder>:
    GetBounds<B> + From<&'elem ElemProp>
{
}
pub trait GetBoundsForConstructibleUpropsSubsumingProp<
    'elem,
    MapUprop: UnassignedProposition,
    ElemProp: 'elem + Proposition,
    B: Binder,
    MapToElemAssignment: PropositionalAssignment<MapUprop, ElemProp>,
>: GetBoundsForUpropsSubsumingProp<'elem, ElemProp, B>
{
    type MapToElemConstructor: PropositionalAssignmentConstructor<MapUprop, ElemProp, MapToElemAssignment>;
    fn get_from_with_map_to_elem_constructors<'binder>(
        &self,
        binder: &'binder B,
    ) -> HashSet<(&'binder B::Value, Self::MapToElemConstructor)>;
}

pub trait GetBoundsForUpropsSubsumingUprop<
    'elem,
    ElemUprop: 'elem + UnassignedProposition,
    B: Binder,
>: GetBounds<B> + From<&'elem ElemUprop>
{
}
pub trait GetBoundsForConstructibleUpropsSubsumingByUprop<
    'elem,
    MapUprop: UnassignedProposition,
    ElemUprop: 'elem + UnassignedProposition,
    B: Binder,
    MapToElemAssignment: PartialPropositionalAssignment<MapUprop, ElemUprop>,
>: GetBoundsForUpropsSubsumingUprop<'elem, ElemUprop, B>
{
    type MapToElemConstructor: PartialPropositionalAssignmentConstructor<MapUprop, ElemUprop, MapToElemAssignment>;
    fn get_from_with_map_to_elem_constructors<'binder>(
        &self,
        binder: &'binder B,
    ) -> HashSet<(&'binder B::Value, Self::MapToElemConstructor)>;
}

pub trait GetBoundsForUpropsSubsumedByUprop<
    'elem,
    ElemUprop: 'elem + UnassignedProposition,
    B: Binder,
>: GetBounds<B> + From<&'elem ElemUprop>
{
}
pub trait GetBoundsForConstructibleUpropsSubsumedByUprop<
    'elem,
    MapUprop: UnassignedProposition,
    ElemUprop: 'elem + UnassignedProposition,
    B: Binder,
    ElemToMapAssignment: PartialPropositionalAssignment<ElemUprop, MapUprop>,
>: GetBoundsForUpropsSubsumedByUprop<'elem, ElemUprop, B>
{
    type ElemToMapConstructor: PartialPropositionalAssignmentConstructor<ElemUprop, MapUprop, ElemToMapAssignment>;
    fn get_from_with_elem_to_map_constructors<'binder>(
        &self,
        binder: &'binder B,
    ) -> HashSet<(&'binder B::Value, Self::ElemToMapConstructor)>;
}

pub trait InsertBoundsForUprop<
    'elem,
    ElemUprop: 'elem + UnassignedProposition,
    B: InsertBinder<Self>,
>: InsertBounds<B> + From<&'elem ElemUprop>
{
}
