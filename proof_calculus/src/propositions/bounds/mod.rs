use crate::{propositions::{Proposition, unassigned::UnassignedProposition}};

pub trait PropositionBoundsAssignedInsertion<'a, P: 'a + Proposition, B>: IntoIterator<Item=B> + From<&'a P> {}
pub trait PropositionBoundsAssignedIdentity<'a, PE: 'a + Proposition, PM: Proposition, B>: IntoIterator<Item=B> + From<&'a PE> {}
pub trait PropositionBoundsUnassignedSubsumesAssigned<'a, PE: 'a + UnassignedProposition, PM: Proposition, B>: IntoIterator<Item=B> + From<&'a PE> {}

// Feature: Generation
pub mod unassigned {
    use crate::propositions::{Proposition, unassigned::UnassignedProposition};

    pub trait PropositionBoundsUnassignedInsertion<'a, P1: 'a + Proposition, B>: IntoIterator<Item=B> + From<&'a P1> {}
    pub trait PropositionBoundsUnassignedIdentity<'a, P1: 'a + UnassignedProposition, P2: UnassignedProposition, B>: IntoIterator<Item=B> + From<&'a P1> {}
    pub trait PropositionBoundsUnassignedEquiv<'a, P: 'a + UnassignedProposition, P2: UnassignedProposition, B>: IntoIterator<Item=B> + From<&'a P> {}
    pub trait PropositionBoundsUnassignedSubsumesUnassigned<'a, P1: 'a + UnassignedProposition, P2: UnassignedProposition, B>: IntoIterator<Item=B> + From<&'a P1> {}
    pub trait PropositionBoundsUnassignedSubsumedByUnassigned<'a, P1: 'a + UnassignedProposition, P2: UnassignedProposition, B>: IntoIterator<Item=B> + From<&'a P1> {}
}
