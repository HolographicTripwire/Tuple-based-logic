use crate::{generation::propositions::UnassignedProposition, structures::propositions::Proposition};

pub trait PropositionBoundsAssignedInsertion<'a, P: 'a + Proposition, B>: IntoIterator<Item=B> + From<&'a P> {}
pub trait PropositionBoundsAssignedIdentity<'a, PE: 'a + Proposition, PM: Proposition, B>: IntoIterator<Item=B> + From<&'a PE> {}
pub trait PropositionBoundsUnassignedSubsumesAssigned<'a, PE: 'a + UnassignedProposition, PM: Proposition, B>: IntoIterator<Item=B> + From<&'a PE> {}
