use crate::propositions::{assignments::{PartialPropositionalAssignment, PropositionalAssignment}, types::{assigned::Proposition, unassigned::{UnassignedProposition, UnassignedPropositionForProp}}};

pub trait NormalisedUnassignedProposition: Sized + Into<Self::Inner> {
    type Inner: UnassignedProposition;

    fn inner(&self) -> &Self::Inner;
    fn into_inner(self) -> Self::Inner;

    #[inline]
    fn apply_assignment<Prop: Proposition, Assignment: PropositionalAssignment<Self::Inner,Prop>>(&self, assignment: Assignment) -> Result<Prop,()>
        where Self::Inner: UnassignedPropositionForProp<Prop>
        { self.inner().assign(assignment) }
    #[inline]
    fn construct_assignment<Prop: Proposition>(&self, assigned: Prop) -> Result<<Self::Inner as UnassignedPropositionForProp<Prop>>::DefaultAssignment,()>
        where Self::Inner: UnassignedPropositionForProp<Prop>
        { self.inner().reverse_assign(assigned) }
    #[inline]
    fn apply_partial_assignment<'slf,PartialAssignment: PartialPropositionalAssignment<'slf,'slf,Self::Inner,Self::Inner>>(self, assignment: &PartialAssignment) -> Self::Inner
        { self.into_inner().partial_assign(assignment) }
    #[inline]
    fn construct_partial_assignment<'slf>(&self, assigned: &Self::Inner) -> Result<<Self::Inner as UnassignedProposition>::DefaultPartialAssignment<'slf>,()>
        { self.inner().partial_reverse_assign(assigned) }
}
