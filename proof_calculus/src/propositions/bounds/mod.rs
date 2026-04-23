use crate::{propositions::Proposition, utils::collections::binders::{Binder, InsertBinder, InsertBounds, UniqueGetBounds}};

pub trait GetBoundsForPropIdenticalToProp<'prop, PE: 'prop + Proposition, B: Binder>: UniqueGetBounds<B> + From<&'prop PE> {}
pub trait InsertBoundsForProp<'prop, PE: 'prop + Proposition, B: InsertBinder<Self>>: InsertBounds<B> + From<&'prop PE> {}


// Feature: Generation
pub mod unassigned {
    use crate::{propositions::{Proposition, unassigned::UnassignedProposition}, utils::collections::binders::{Binder, GetBounds, InsertBinder, InsertBounds, UniqueGetBounds}};

    pub trait GetBoundsForPropsSubsumedByUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'prop UPE> {}

    pub trait GetBoundsForUpropIdenticalToUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: UniqueGetBounds<B> + From<&'prop UPE> {}
    pub trait GetBoundsForUpropsEquivalentToUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'prop UPE> {}
    pub trait GetBoundsForUpropsSubsumingProp<'prop, PE: 'prop + Proposition, B: Binder>: GetBounds<B> + From<&'prop PE> {}
    pub trait GetBoundsForUpropsSubsumedByUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'prop UPE> {}
    pub trait GetBoundsForUpropsSubsumingByUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'prop UPE> {}

    pub trait InsertBoundsForUprop<'prop,UPE: 'prop + UnassignedProposition, B: InsertBinder<Self>>: InsertBounds<B> + From<&'prop UPE> {}
}
