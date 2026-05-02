use crate::{proofs::inferences::{Inference, InferenceRule, located::InferenceAtPath}, propositions::types::assigned::{ParentOfExplicitConclusions, Proposition, located::ExplicitConclusionInSequentialProofStep, paths::ExplicitConclusionInSequentialProofStepPath}};


// pub struct ExplicitConclusionCountCheckError<P: Proposition, Rule: InferenceRule<P>> {
//     pub expected_count: usize,
//     pub inference: Inference<P,Rule>
// }
// impl <P: Proposition, Rule: InferenceRule<P>> ExplicitConclusionCountCheckError<P,Rule> {
//     pub fn get_actual_count(&self) -> usize { self.inference.conclusions.len() }
// }

// /// Check that the provided [Inference](OwnedInferenceInProof) has expected_count explicit conclusions, returning an error otherwise
// pub fn retrieve_explicit_conclusion<'a, P: Proposition, Rule: InferenceRule<P>, Path>(inference: InferenceAtPath<'a,P,Rule,Path>) -> ExplicitConclusionInSequentialProofStep<'a,P> {
//     inference.get_located_explicit_conclusion(ExplicitConclusionInSequentialProofStepPath(0))
//         .expect("Every inference has an explicit conclusion")
// }
