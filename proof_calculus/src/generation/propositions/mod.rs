use std::hash::Hash;

use crate::{generation::propositions::assignments::{PartialPropositionalAssignment, PropositionalAssignment}, structures::propositions::Proposition};

pub mod assignments;

pub trait UnassignedProposition: Clone + PartialEq + Eq + Hash {
    type AssignedResult: Proposition;
    type Assignment: PropositionalAssignment;
    type PartialAssignment: PartialPropositionalAssignment;

    fn assign(&self, assignment: &Self::Assignment) -> Result<Self::AssignedResult,()>;
    fn reverse_assign(&self, assigned: Self::AssignedResult) -> Result<Self::Assignment,()>;
    fn partial_assign(self, assignment: &Self::PartialAssignment) -> Self;
    fn partial_reverse_assign(&self, assigned: &Self) -> Result<Self::PartialAssignment,()>;
}
