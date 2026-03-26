use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::structures::{Proposition, propositions::paths::{AssumptionIProofStepPath, ExplicitConclusionInProofStepPath}};

pub mod paths;

generate_parent_of_children_trait!{
    (P where P: Proposition), AssumptionIProofStepPath,
    "assumption", "assumptions", "Assumptions"
}
generate_parent_of_children_trait!{
    (P where P: Proposition), ExplicitConclusionInProofStepPath,
    "explicit_conclusion", "explicit_conclusions", "ExplicitConclusions"
}
