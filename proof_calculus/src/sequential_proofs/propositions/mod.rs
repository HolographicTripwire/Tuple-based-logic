use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::sequential_proofs::propositions::paths::{AssumptionInProofStepPath, ConclusionInSequentialProofStepPath};

pub mod paths;

generate_parent_of_children_trait!{
    (Proposition where Proposition: Clone), AssumptionInProofStepPath,
    "assumption", "assumptions", "Assumptions"
}
generate_parent_of_children_trait!{
    (Proposition where Proposition: Clone), ConclusionInSequentialProofStepPath,
    "explicit_conclusion", "explicit_conclusions", "ExplicitConclusions"
}
