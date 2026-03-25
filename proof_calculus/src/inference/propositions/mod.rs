use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::inference::propositions::paths::{AntecedentInInferencePath, ConsequentInInferencePath};

pub mod paths;

generate_parent_of_children_trait!{
    (Proposition where Proposition: Clone), AntecedentInInferencePath,
    "antecedent", "antecedents", "Antecedents"
}
generate_parent_of_children_trait!{
    (Proposition where Proposition: Clone), ConsequentInInferencePath,
    "consequent", "consequents", "Consequents"
}
