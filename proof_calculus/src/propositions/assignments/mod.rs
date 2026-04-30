use crate::propositions::unassigned::UnassignedProposition;

pub trait PropositionalAssignment<UP: UnassignedProposition>: Sized {
    type CombinationError;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>;
}
pub trait PartialPropositionalAssignment<UP: UnassignedProposition>: Sized {
    type CombinationError;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>;
}
