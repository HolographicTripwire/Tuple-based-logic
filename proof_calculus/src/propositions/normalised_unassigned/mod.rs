use crate::propositions::unassigned::{PropositionalAssignment, UnassignedProposition};

pub trait NormalisedUnassignedProposition: Sized {
    type Inner: UnassignedProposition;

    fn inner(&self) -> &Self::Inner;
    fn into_inner(self) -> Self::Inner;

    #[inline]
    fn assign<Assignment: PropositionalAssignment<Self::Inner>>(&self, assignment: Assignment) -> Result<<Self::Inner as UnassignedProposition>::AssignedResult,()>
        { self.inner().assign(assignment) }
    #[inline]
    fn reverse_assign(&self, assigned: <Self::Inner as UnassignedProposition>::AssignedResult) -> Result<<Self::Inner as UnassignedProposition>::DefaultAssignment,()>
        { self.inner().reverse_assign(assigned) }
    #[inline]
    fn partial_assign<PartialAssignment: PropositionalAssignment<Self::Inner>>(self, assignment: &PartialAssignment) -> Self::Inner
        { self.into_inner().partial_assign(assignment) }
    #[inline]
    fn partial_reverse_assign(&self, assigned: &Self::Inner) -> Result<<Self::Inner as UnassignedProposition>::DefaultPartialAssignment,()>
        { self.inner().partial_reverse_assign(assigned) }
}
