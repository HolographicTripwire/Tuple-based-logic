use std::hash::Hash;
use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::{structures::propositions::paths::{AssumptionInProofStepPath, ExplicitConclusionInProofStepPath}, utils::traits::fast_ord::FastOrd};

pub mod paths;

generate_parent_of_children_trait!{
    P, AssumptionInProofStepPath, (P: Proposition),
    "assumption", "assumptions", "Assumptions"
}
generate_parent_of_children_trait!{
    P, ExplicitConclusionInProofStepPath, (P: Proposition),
    "explicit_conclusion", "explicit_conclusions", "ExplicitConclusions"
}

pub trait Proposition: Clone + PartialEq + Eq + Hash {}
