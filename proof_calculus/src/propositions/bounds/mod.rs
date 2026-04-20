use trait_aliases::trait_aliases;

use crate::{propositions::Proposition, utils::collections::binders::{Binder, InsertBinder, InsertBounds, UniqueGetBounds}};

trait_aliases!{
    pub trait GetBoundsForPropIdenticalToProp<'a, PE: 'a + Proposition, B: Binder> = UniqueGetBounds<B> + From<&'a PE>;
    pub trait InsertBoundsForProp<'a,PE: 'a + Proposition, B: InsertBinder<Self>> = InsertBounds<B> + From<&'a PE>;
}

// Feature: Generation
pub mod unassigned {
    use crate::{propositions::{Proposition, unassigned::UnassignedProposition}, utils::collections::binders::{Binder, GetBounds, InsertBinder, InsertBounds, UniqueGetBounds}};
    use trait_aliases::trait_aliases;

    trait_aliases!{
        pub trait GetBoundsForPropsSubsumedByUprop<'a, UPE: 'a + UnassignedProposition, B: Binder> = GetBounds<B> + From<&'a UPE>;
    }
    trait_aliases!{
        pub trait GetBoundsForUpropIdenticalToUprop<'a, UPE: 'a + UnassignedProposition, B: Binder> = UniqueGetBounds<B> + From<&'a UPE>;
        pub trait GetBoundsForUpropsEquivalentToUprop<'a, UPE: 'a + UnassignedProposition, B: Binder> = GetBounds<B> + From<&'a UPE>;
        pub trait GetBoundsForUpropsSubsumingProp<'a, PE: 'a + Proposition, B: Binder> = GetBounds<B> + From<&'a PE>;
        pub trait GetBoundsForUpropsSubsumedByUprop<'a, UPE: 'a + UnassignedProposition, B: Binder> = GetBounds<B> + From<&'a UPE>;
        pub trait GetBoundsForUpropsSubsumingByUprop<'a, UPE: 'a + UnassignedProposition, B: Binder> = GetBounds<B> + From<&'a UPE>;

        pub trait InsertBoundsForUprop<'a,UPE: 'a + UnassignedProposition, B: InsertBinder<Self>> = InsertBounds<B> + From<&'a UPE>;

    }
}
