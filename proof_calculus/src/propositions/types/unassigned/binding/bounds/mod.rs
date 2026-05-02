use crate::{propositions::types::{assigned::Proposition, unassigned::UnassignedProposition}, utils::collections::binding::{binders::{Binder, InsertBinder}, bounds::{GetBounds, InsertBounds, UniqueGetBounds}}};

pub trait GetBoundsForUpropIdenticalToUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: UniqueGetBounds<B> + From<&'prop UPE> {}
pub trait GetBoundsForUpropsEquivalentToUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'prop UPE> {}
pub trait GetBoundsForUpropsSubsumingProp<'prop, PE: 'prop + Proposition, B: Binder>: GetBounds<B> + From<&'prop PE> {}
pub trait GetBoundsForUpropsSubsumedByUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'prop UPE> {}
pub trait GetBoundsForUpropsSubsumingByUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'prop UPE> {}

pub trait InsertBoundsForUprop<'prop,UPE: 'prop + UnassignedProposition, B: InsertBinder<Self>>: InsertBounds<B> + From<&'prop UPE> {}
