use crate::{generation::propositions::UnassignedProposition, structures::propositions::Proposition};

pub trait PropositionBoundsUnassignedInsertion<'a, P1: 'a + Proposition, B>: IntoIterator<Item=B> + From<&'a P1> {}
pub trait PropositionBoundsUnassignedIdentity<'a, P1: 'a + UnassignedProposition, P2: UnassignedProposition, B>: IntoIterator<Item=B> + From<&'a P1> {}
pub trait PropositionBoundsUnassignedEquiv<'a, P: 'a + UnassignedProposition, P2: UnassignedProposition, B>: IntoIterator<Item=B> + From<&'a P> {}
pub trait PropositionBoundsUnassignedSubsumesUnassigned<'a, P1: 'a + UnassignedProposition, P2: UnassignedProposition, B>: IntoIterator<Item=B> + From<&'a P1> {}
pub trait PropositionBoundsUnassignedSubsumedByUnassigned<'a, P1: 'a + UnassignedProposition, P2: UnassignedProposition, B>: IntoIterator<Item=B> + From<&'a P1> {}
