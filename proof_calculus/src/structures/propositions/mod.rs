use std::hash::Hash;
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::structures::propositions::paths::{AssumptionInSequentialProofStepPath, ExplicitConclusionInSequentialProofStepPath};

pub mod paths;
pub mod located;
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
