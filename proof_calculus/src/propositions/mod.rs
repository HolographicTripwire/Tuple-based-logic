use std::hash::Hash;
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::propositions::paths::{AssumptionInSequentialProofStepPath, ExplicitConclusionInSequentialProofStepPath};

pub mod paths;
pub mod located;
pub mod bounds;
pub mod collections;

generate_parent_of_children_trait!{
    P, AssumptionInSequentialProofStepPath, (P: Proposition),
    "assumption", "assumptions", "Assumptions"
}
generate_parent_of_children_trait!{
    P, ExplicitConclusionInSequentialProofStepPath, (P: Proposition),
    "explicit_conclusion", "explicit_conclusions", "ExplicitConclusions"
}

pub trait Proposition: Clone + PartialEq + Eq + Hash {}

// Feature: Generation
pub mod unassigned {
    use crate::propositions::Proposition;
    use std::hash::Hash;

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
}