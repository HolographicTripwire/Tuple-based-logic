// Feature: Generation
use crate::propositions::assigned::Proposition;
use std::hash::Hash;

pub mod binding;

pub trait UnassignedProposition: Clone + PartialEq + Eq + Hash {
    type AssignedResult: Proposition;
    type Assignment: PropositionalAssignment;
    type PartialAssignment: PartialPropositionalAssignment;

    fn assign(&self, assignment: &Self::Assignment) -> Result<Self::AssignedResult,()>;
    fn reverse_assign(&self, assigned: Self::AssignedResult) -> Result<Self::Assignment,()>;
    fn partial_assign(self, assignment: &Self::PartialAssignment) -> Self;
    fn partial_reverse_assign(&self, assigned: &Self) -> Result<Self::PartialAssignment,()>;
}

pub trait PropositionalAssignment: Sized {
    
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,()>;
}
pub trait PartialPropositionalAssignment: Sized {
    fn combine<I: IntoIterator<Item = Self>>(assignments: I) -> Result<Self,()>;
}
