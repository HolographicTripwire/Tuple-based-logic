use crate::propositions::{types::{assigned::Proposition, normalised_unassigned::NormalisedUnassignedProposition}, assignments::{PartialPropositionalAssignment, PropositionalAssignment}};
use std::hash::Hash;

pub mod binding;

pub trait UnassignedProposition: Clone + PartialEq + Eq + Hash {
    type AssignedResult: Proposition;
    type DefaultAssignment: PropositionalAssignment<Self>;
    type DefaultPartialAssignment: PartialPropositionalAssignment<Self>;
    type DefaultNormalisation: NormalisedUnassignedProposition;

    fn assign<Assignment: PropositionalAssignment<Self>>(&self, assignment: Assignment) -> Result<Self::AssignedResult,()>;
    fn reverse_assign(&self, assigned: Self::AssignedResult) -> Result<Self::DefaultAssignment,()>;
    fn partial_assign<PartialAssignment: PropositionalAssignment<Self>>(self, assignment: &PartialAssignment) -> Self;
    fn partial_reverse_assign(&self, assigned: &Self) -> Result<Self::DefaultPartialAssignment,()>;

    fn normalise(self) -> Self::DefaultNormalisation;
}
