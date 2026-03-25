pub mod propositions;
pub mod composite;
pub mod subproof;
pub mod at_path_enum;
pub mod errors;

use path_lib_proc_macros::generate_parent_of_children_trait;

use crate::{inference::{Inference, rules::InferenceRule}, propositions::{ParentOfAssumptions, ParentOfExplicitConclusions, path::AssumptionInProofStepPath}, sequential_proofs::composite::CompositeSequentialProof};

/// This struct represents a step within a larger proof
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Proof<Proposition, Rule: InferenceRule<Proposition>> {
    Inference(Inference<Proposition, Rule>), // A single inference step
    Composite(CompositeSequentialProof<Proposition, Rule>) // A composite proof made of further subproofs
}

impl <Proposition, Rule: InferenceRule<Proposition>> ParentOfAssumptions<Proposition> for Proof<Proposition, Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionInProofStepPath>  { match self {
        Proof::Inference(inference) => inference.get_assumption_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.get_assumption_paths().into_iter().collect(),
    }}
    
    fn get_assumption(&self,path: &AssumptionInProofStepPath) -> Result< &Proposition,()>  { match self {
        Proof::Inference(inference) => inference.get_assumption(path),
        Proof::Composite(composite_proof) => composite_proof.get_assumption(path),
    }}
}
impl <Proposition, Rule: InferenceRule<Proposition>> ParentOfExplicitConclusions<Proposition> for Proof<Proposition,Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInProofStepPath> {match self {
        Proof::Inference(inference) => inference.get_explicit_conclusion_paths().into_iter().collect::<Vec<_>>(),
        Proof::Composite(composite) => composite.get_explicit_conclusion_paths().into_iter().collect(),
    }}
    
    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInProofStepPath) -> Result< &Proposition,()>  {match self {
        Proof::Inference(inference) => inference.get_explicit_conclusion(path),
        Proof::Composite(composite_proof) => composite_proof.get_explicit_conclusion(path),
    }}
}
impl <Proposition, Rule: InferenceRule<Proposition>> ProofStep<Proposition, Rule> for Proof<Proposition, Rule> {}
