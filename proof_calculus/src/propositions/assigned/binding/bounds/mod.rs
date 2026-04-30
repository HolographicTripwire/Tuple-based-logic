use crate::{propositions::{assigned::Proposition, unassigned::UnassignedProposition}, utils::collections::binding::{binders::{Binder, InsertBinder},bounds::{GetBounds, InsertBounds, UniqueGetBounds}}};

pub trait GetBoundsForPropIdenticalToProp<'prop, PE: 'prop + Proposition, B: Binder>: UniqueGetBounds<B> + From<&'prop PE> {}
pub trait InsertBoundsForProp<'prop, PE: 'prop + Proposition, B: InsertBinder<Self>>: InsertBounds<B> + From<&'prop PE> {}

pub trait GetBoundsForPropsSubsumedByUprop<'prop, UPE: 'prop + UnassignedProposition, B: Binder>: GetBounds<B> + From<&'prop UPE> {}
