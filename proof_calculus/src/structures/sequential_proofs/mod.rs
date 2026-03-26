pub mod composite;
pub mod subproof;
pub mod at_path_enum;
pub mod errors;

use crate::structures::{Proposition, inference::{Inference, InferenceRule}, propositions::{ParentOfAssumptions, ParentOfExplicitConclusions, paths::{AssumptionIProofStepPath, ExplicitConclusionInProofStepPath}}, sequential_proofs::composite::CompositeSequentialProof};


/// This struct represents a step within a larger proof
#[derive(Clone,PartialEq,Eq,Debug)]
pub enum SequentialProof<P: Proposition, Rule: InferenceRule<P>> {
    Inference(Inference<P, Rule>), // A single inference step
    Composite(CompositeSequentialProof<P, Rule>) // A composite proof made of further subproofs
}

impl <P: Proposition, Rule: InferenceRule<P>> ParentOfAssumptions<P> for SequentialProof<P, Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionIProofStepPath>  { match self {
        SequentialProof::Inference(inference) => inference.get_assumption_paths().into_iter().collect::<Vec<_>>(),
        SequentialProof::Composite(composite) => composite.get_assumption_paths().into_iter().collect(),
    }}
    
    fn get_assumption(&self,path: &AssumptionIProofStepPath) -> Result< &P,()>  { match self {
        SequentialProof::Inference(inference) => inference.get_assumption(path),
        SequentialProof::Composite(composite_proof) => composite_proof.get_assumption(path),
    }}
}
impl <P:Proposition, Rule: InferenceRule<P>> ParentOfExplicitConclusions<P> for SequentialProof<P,Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInProofStepPath> {match self {
        SequentialProof::Inference(inference) => inference.get_explicit_conclusion_paths().into_iter().collect::<Vec<_>>(),
        SequentialProof::Composite(composite) => composite.get_explicit_conclusion_paths().into_iter().collect(),
    }}
    
    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInProofStepPath) -> Result< &P,()>  {match self {
        SequentialProof::Inference(inference) => inference.get_explicit_conclusion(path),
        SequentialProof::Composite(composite_proof) => composite_proof.get_explicit_conclusion(path),
    }}
}
//impl <P: Proposition, Rule: InferenceRule<P>> ProofStep<P, Rule> for SequentialProof<P, Rule> {}
