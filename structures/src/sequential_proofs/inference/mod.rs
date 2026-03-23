
use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};

use crate::{expressions::Proposition, sequential_proofs::{AssumptionInProofStepPath, ExplicitConclusionInProofStepPath, ParentOfAssumptions, ParentOfExplicitConclusions, ProofInProofPath, ProofStep}};

#[derive(Clone,PartialEq,Eq,Debug)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Rule:InferenceRule> {
    pub inference_type: Rule,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}

impl <Rule: InferenceRule> ParentOfAssumptions for Inference<Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionInProofStepPath>
        { (0..self.assumptions.len()).map(|n| AssumptionInProofStepPath(n)) }

    fn get_assumption(&self,path: &AssumptionInProofStepPath) -> Result<&Proposition,()>
        { self.assumptions.get(path.0).ok_or(()) }

    fn get_assumptions(&self) -> impl IntoIterator<Item = &Proposition> { &self.assumptions }
    
    fn into_located_assumptions_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,AssumptionInProofStepPath>> where Proposition:Clone,Self:Sized {
        self.assumptions.into_iter()
            .enumerate()
            .map(|(id,conclusion)| OwnedObjAtPath{obj: conclusion, path: AssumptionInProofStepPath(id)})
    }
}
impl <Rule: InferenceRule> ParentOfExplicitConclusions for Inference<Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInProofStepPath>
        { (0..self.conclusions.len()).map(|n| ExplicitConclusionInProofStepPath(n)) }

    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInProofStepPath) -> Result<&Proposition,()>
        { self.conclusions.get(path.0).ok_or(()) }

    fn get_explicit_conclusions(&self) -> impl IntoIterator<Item = &Proposition> { &self.conclusions }

    fn into_located_explicit_conclusions_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,ExplicitConclusionInProofStepPath>> where Proposition:Clone,Self:Sized {
        self.conclusions.into_iter()
            .enumerate()
            .map(|(id,conclusion)| OwnedObjAtPath{obj: conclusion, path: ExplicitConclusionInProofStepPath(id)})
    }
}
impl <Rule:InferenceRule> ProofStep<Rule> for Inference<Rule> {}

// impl <Rule: InferenceRule> HasChildren<AtomicProofInProofPath,Proof<Rule>> for Inference<Rule> {
//     fn valid_primitive_paths(&self) -> Vec<AtomicProofInProofPath> { vec![] }
//     fn get_child(&self, _: &AtomicProofInProofPath) -> Result<&Proof<Rule>,()> { Err(()) }
//     fn get_child_owned(&self, _: &AtomicProofInProofPath) -> Result<Proof<Rule>,()> { Err(()) }
    
//     fn into_located_children_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proof<Rule>,AtomicProofInProofPath>> where Proof<Rule>: Clone, Self: Sized
//         { vec![] }
// }

pub trait InferenceRule: 'static + Clone + PartialEq {}

pub type InferenceInProof<'a, Rule> = ObjAtPath<'a,Inference<Rule>,ProofInProofPath>;
pub type OwnedInferenceInProof<Rule> = OwnedObjAtPath<Inference<Rule>,ProofInProofPath>;

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
