use crate::propositions::types::unassigned::UnassignedProposition;

pub trait PropositionalAssignment<UP: UnassignedProposition>: Sized {
    type CombinationError;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>;
}
pub trait PropositionalAssignmentConstructor<UP: UnassignedProposition, PA: PropositionalAssignment<UP>>: Sized {
    fn construct(prop: UP) -> PA;
}

pub trait PartialPropositionalAssignment<UP: UnassignedProposition>: Sized {
    type CombinationError;
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,Self::CombinationError>;
}
pub trait UnassignedPropositionalAssignmentConstructor<UP: UnassignedProposition, PA: PartialPropositionalAssignment<UP>>: Sized {
    fn construct(prop: UP) -> PA;
}
