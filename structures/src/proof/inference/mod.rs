
use path_lib::{HasChildren, obj_at_path::{OwnedObjAtPath}};

use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{expressions::Proposition, path_composites::{OwnedPropositionInProof, PropositionInProof, PropositionInProofPath}, proof::{AtomicProofInProofPath, OwnedPropositionInProofStep, Proof, ProofInProofPath, ProofStep, PropositionInProofStep, PropositionInProofStepPath, get_child_inner, valid_primitive_paths_inner}};

#[derive(Clone,PartialEq,Eq,Debug)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Rule:InferenceRule> {
    pub inference_type: Rule,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}

impl <Rule:InferenceRule> ProofStep<Rule> for Inference<Rule> {
    fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInProofStepPath>
        { (0..self.assumptions.len()).map(|n| PropositionInProofStepPath::assumption(n)) }
    fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInProofStepPath>
        { (0..self.conclusions.len()).map(|n| PropositionInProofStepPath::conclusion(n)) }

    // Faster versions of default members
    fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition> { &self.assumptions }
    fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition> { &self.conclusions }
}
impl <Rule:InferenceRule> HasChildren<PropositionInProofStepPath,Proposition> for Inference<Rule> {
    fn valid_primitive_paths(&self) -> Vec<PropositionInProofStepPath> { valid_primitive_paths_inner(self, self.conclusions.len()) }
    fn get_child(&self, path: &PropositionInProofStepPath) -> Result<&Proposition,()> { get_child_inner(self,path) }

    fn get_child_owned(&self, path: &PropositionInProofStepPath) -> Result<Proposition,()> where Proposition: Clone
        { get_child_inner(self,path).cloned() }

    fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,PropositionInProofStepPath>> where Proposition: Clone, Self: Sized {
        let assumptions = self.assumptions.into_iter()
            .enumerate()
            .map(|(id,conclusion)| OwnedObjAtPath::from_inner(conclusion, PropositionInProofStepPath::assumption(id)));
        let conclusions = self.conclusions.into_iter()
            .enumerate()
            .map(|(id,conclusion)| OwnedObjAtPath::from_inner(conclusion, PropositionInProofStepPath::conclusion(id)));
        assumptions.chain(conclusions)
    }
}
impl <Rule: InferenceRule> HasChildren<AtomicProofInProofPath,Proof<Rule>> for Inference<Rule> {
    fn valid_primitive_paths(&self) -> Vec<AtomicProofInProofPath> { vec![] }
    fn get_child(&self, _: &AtomicProofInProofPath) -> Result<&Proof<Rule>,()> { Err(()) }
    fn get_child_owned(&self, _: &AtomicProofInProofPath) -> Result<Proof<Rule>,()> { Err(()) }
    
    fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicProofInProofPath>> where Proof<Rule>: Clone, Self: Sized
        { vec![] }
}

pub trait InferenceRule: 'static + Clone + PartialEq {}

generate_obj_at_path_wrappers!{
    (Inference<Rule> where Rule: InferenceRule),ProofInProofPath,
    "InferenceInProof", [Clone, PartialEq, Eq, Debug],
    "OwnedInferenceInProof", [Clone, PartialEq, Eq, Debug]
}

impl <'a, Rule:InferenceRule> InferenceInProof<'a,Rule> {
    fn path_replacer<'b>(&self, step: PropositionInProofStep<'b>) -> PropositionInProof<'b>
        { step.replace_path(|p| PropositionInProofPath::new(self.path().clone(), p)).into() }
    fn path_replacer_owned(&self, step: OwnedPropositionInProofStep) -> OwnedPropositionInProof
        { step.replace_path(|p| PropositionInProofPath::new(self.path().clone(), p)).into() }

    pub fn assumption_paths(&self) -> impl IntoIterator<Item = PropositionInProofStepPath>
        { self.obj().assumption_paths() }
    pub fn explicit_conclusion_paths(&self) -> impl IntoIterator<Item = PropositionInProofStepPath>
        { self.obj().explicit_conclusion_paths() }
    
    /// Get references to all assumptions within this [ProofStep]
    pub fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition>
        { self.obj().get_assumptions() }
    /// Get all assumptions within this [ProofStep]
    pub fn get_assumptions_owned(&self) -> impl IntoIterator<Item = Proposition>
        { self.obj().get_assumptions_owned() }
    /// Get references to all assumptions within this [ProofStep], located by their [ProofPropositionPath]
    pub fn get_located_assumptions(&'a self) -> impl IntoIterator<Item = PropositionInProof<'a>>
        { self.obj().get_located_assumptions().into_iter().map(|x| self.path_replacer(x)) }
    /// Get all assumptions within this [ProofStep], located by their [ProofPropositionPath]
    pub fn get_located_assumptions_owned(&self) -> impl IntoIterator<Item = OwnedPropositionInProof>
        { self.obj().get_located_assumptions_owned().into_iter().map(|p| self.path_replacer_owned(p)) }
    
    /// Get all explicit conclusions within this [ProofStep]
    pub fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition>
        { self.obj().get_explicit_conclusions() }
    /// Get owned versions of all explicit conclusions within this [ProofStep]
    pub fn get_explicit_conclusions_owned(&self) -> impl IntoIterator<Item = Proposition>
        { self.obj().get_explicit_conclusions_owned() }
    /// Get all explicit conclusions within this [ProofStep], located by their [ProofPropositionPath]
    pub fn get_located_explicit_conclusions(&'a self) -> impl IntoIterator<Item = PropositionInProof<'a>>
        { self.obj().get_located_explicit_conclusions().into_iter().map(|x| self.path_replacer(x)) }
    /// Get owned versions of all explicit conclusions within this [ProofStep], located by their [ProofPropositionPath]
    pub fn get_located_explicit_conclusions_owned(&self) -> impl IntoIterator<Item = OwnedPropositionInProof>
        { self.obj().get_located_explicit_conclusions_owned().into_iter().map(|p| self.path_replacer_owned(p)) }
}
