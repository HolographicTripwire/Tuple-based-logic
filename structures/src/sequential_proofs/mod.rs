pub mod inference;
pub mod composite;
pub mod at_path_enum;
mod in_proof;
pub mod error;

use path_lib_proc_macros::generate_parent_of_children_trait;
pub use in_proof::*;

use crate::{propositions::Proposition, sequential_proofs::{composite::CompositeSequentialProof, inference::{Inference, InferenceRule}}};

generate_parent_of_children_trait!{
    (Proposition), AssumptionInProofStepPath,
    "assumption", "assumptions", "Assumptions"
}
generate_parent_of_children_trait!{
    (Proposition), ExplicitConclusionInProofStepPath,
    "explicit_conclusion", "explicit_conclusions", "ExplicitConclusions"
}
pub trait ProofStep<Rule: InferenceRule>: ParentOfAssumptions + ParentOfExplicitConclusions {}

/// This struct represents a step within a larger proof
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Proof<Rule: InferenceRule> {
    Inference(Inference<Rule>), // A single inference step
    Composite(CompositeSequentialProof<Rule>) // A composite proof made of further subproofs
}

impl <Rule: InferenceRule> ParentOfAssumptions for Proof<Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionInProofStepPath>  { match self {
        Proof::Inference(inference) => inference.get_assumption_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.get_assumption_paths().into_iter().collect(),
    }}
    
    fn get_assumption(&self,path: &AssumptionInProofStepPath) -> Result< &Proposition,()>  { match self {
        Proof::Inference(inference) => inference.get_assumption(path),
        Proof::Composite(composite_proof) => composite_proof.get_assumption(path),
    }}
}
impl <Rule: InferenceRule> ParentOfExplicitConclusions for Proof<Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInProofStepPath> {match self {
        Proof::Inference(inference) => inference.get_explicit_conclusion_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.get_explicit_conclusion_paths().into_iter().collect(),
    }}
    
    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInProofStepPath) -> Result< &Proposition,()>  {match self {
        Proof::Inference(inference) => inference.get_explicit_conclusion(path),
        Proof::Composite(composite_proof) => composite_proof.get_explicit_conclusion(path),
    }}
}
impl <Rule: InferenceRule> ProofStep<Rule> for Proof<Rule> {}

#[cfg(test)]
mod tests {
    use crate::sequential_proofs::in_proof::ProofInProofPath;    

    #[test]
    fn test_getters() {
        let step = ProofInProofPath::default();
        assert_eq!(step.0, vec![])
    }
}
