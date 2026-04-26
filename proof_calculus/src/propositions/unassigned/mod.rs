// Feature: Generation
use crate::propositions::{assigned::Proposition, normalised_unassigned::NormalisedUnassignedProposition};
use std::hash::Hash;

pub mod binding;

pub trait UnassignedProposition: Clone + PartialEq + Eq + Hash {
    type AssignedResult: Proposition;
    type DefaultAssignment: PropositionalAssignment<Self>;
    type DefaultPartialAssignment: PropositionalAssignment<Self>;
    type DefaultNormalisation: NormalisedUnassignedProposition;

    fn assign<Assignment: PropositionalAssignment<Self>>(&self, assignment: Assignment) -> Result<Self::AssignedResult,()>;
    fn reverse_assign(&self, assigned: Self::AssignedResult) -> Result<Self::DefaultAssignment,()>;
    fn partial_assign<PartialAssignment: PropositionalAssignment<Self>>(self, assignment: &PartialAssignment) -> Self;
    fn partial_reverse_assign(&self, assigned: &Self) -> Result<Self::DefaultPartialAssignment,()>;

    fn normalise(self) -> Self::DefaultNormalisation;
}

pub trait PropositionalAssignment<UP: UnassignedProposition>: Sized {
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,()>;
}

