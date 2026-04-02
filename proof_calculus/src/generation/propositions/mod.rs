use std::hash::Hash;

use crate::structures::propositions::Proposition;

pub trait UnassignedProposition: Clone + PartialEq + Eq + Hash {
    type AssignedResult: Proposition;
    type Assignment;

    fn assign(assignment: Self::Assignment) -> Self::AssignedResult;
}
