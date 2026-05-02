use crate::propositions::{types::{assigned::Proposition, normalised_unassigned::NormalisedUnassignedProposition}, assignments::{PartialPropositionalAssignment, PropositionalAssignment}};
use std::hash::Hash;

pub mod binding;

pub trait UnassignedProposition: Clone + PartialEq + Eq + Hash {
    type DefaultPartialAssignment<'slf>: PartialPropositionalAssignment<'slf,'slf,Self,Self>;
    type DefaultNormalisation: NormalisedUnassignedProposition;

    fn partial_assign<'slf, PartialAssignment: PartialPropositionalAssignment<'slf,'slf,Self,Self>>(self, assignment: &PartialAssignment) -> Self;
    fn partial_reverse_assign<'slf>(&self, assigned: &Self) -> Result<Self::DefaultPartialAssignment<'slf>,()>;

    fn normalise(self) -> Self::DefaultNormalisation;
}
pub trait UnassignedPropositionForProp<Prop: Proposition>: UnassignedProposition {
    type DefaultAssignment: PropositionalAssignment<Self,Prop>;
    fn assign<Assignment: PropositionalAssignment<Self,Prop>>(&self, assignment: Assignment) -> Result<Prop,()>;
    fn reverse_assign(&self, assigned: Prop) -> Result<Self::DefaultAssignment,()>;
}
