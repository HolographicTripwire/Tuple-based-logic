
use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};

use crate::{expressions::TblProposition, sequential_proofs::{AssumptionInProofStepPath, ExplicitConclusionInProofStepPath, ParentOfAssumptions, ParentOfExplicitConclusions, ProofInProofPath, ProofStep}};


// impl <Rule: InferenceRule> HasChildren<AtomicProofInProofPath,Proof<Rule>> for Inference<Rule> {
//     fn valid_primitive_paths(&self) -> Vec<AtomicProofInProofPath> { vec![] }
//     fn get_child(&self, _: &AtomicProofInProofPath) -> Result<&Proof<Rule>,()> { Err(()) }
//     fn get_child_owned(&self, _: &AtomicProofInProofPath) -> Result<Proof<Rule>,()> { Err(()) }
    
//     fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicProofInProofPath>> where Proof<Rule>: Clone, Self: Sized
//         { vec![] }
// }

// impl <'a, Rule:InferenceRule> InferenceInProof<'a,Rule> {
//     fn path_replacer<'b>(&self, step: PropositionInProofStep<'b>) -> PropositionInProof<'b>
//         { step.replace_path(|p| PropositionInProofPath::new(self.path().clone(), p)).into() }
//     fn path_replacer_owned(&self, step: OwnedPropositionInProofStep) -> OwnedPropositionInProof
//         { step.replace_path(|p| PropositionInProofPath::new(self.path().clone(), p)).into() }

//     pub fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInProofStepPath>
//         { self.obj().assumption_paths() }
//     pub fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInProofStepPath>
//         { self.obj().explicit_conclusion_paths() }
    
//     /// Get references to all assumptions within this [ProofStep]
//     pub fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition>
//         { self.obj().get_assumptions() }
//     /// Get all assumptions within this [ProofStep]
//     pub fn get_assumptions_owned(&self) -> impl IntoIterator<Item = Proposition>
//         { self.obj().get_assumptions_owned() }
//     /// Get references to all assumptions within this [ProofStep], located by their [ProofPropositionPath]
//     pub fn get_located_assumptions(&'a self) -> impl IntoIterator<Item = PropositionInProof<'a>>
//         { self.obj().get_located_assumptions().into_iter().map(|x| self.path_replacer(x)) }
//     /// Get all assumptions within this [ProofStep], located by their [ProofPropositionPath]
//     pub fn get_located_assumptions_owned(&self) -> impl IntoIterator<Item = OwnedPropositionInProof>
//         { self.obj().get_located_assumptions_owned().into_iter().map(|p| self.path_replacer_owned(p)) }
    
//     /// Get all explicit conclusions within this [ProofStep]
//     pub fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition>
//         { self.obj().get_explicit_conclusions() }
//     /// Get owned versions of all explicit conclusions within this [ProofStep]
//     pub fn get_explicit_conclusions_owned(&self) -> impl IntoIterator<Item = Proposition>
//         { self.obj().get_explicit_conclusions_owned() }
//     /// Get all explicit conclusions within this [ProofStep], located by their [ProofPropositionPath]
//     pub fn get_located_explicit_conclusions(&'a self) -> impl IntoIterator<Item = PropositionInProof<'a>>
//         { self.obj().get_located_explicit_conclusions().into_iter().map(|x| self.path_replacer(x)) }
//     /// Get owned versions of all explicit conclusions within this [ProofStep], located by their [ProofPropositionPath]
//     pub fn get_located_explicit_conclusions_owned(&self) -> impl IntoIterator<Item = OwnedPropositionInProof>
//         { self.obj().get_located_explicit_conclusions_owned().into_iter().map(|p| self.path_replacer_owned(p)) }
// }
