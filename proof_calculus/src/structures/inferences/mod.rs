use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::{propositions::Proposition, propositions::{ParentOfAssumptions, ParentOfExplicitConclusions, paths::{AssumptionInProofStepPath, ExplicitConclusionInProofStepPath}}, sequential_proofs::subproofs::ProofInProofPath};

pub trait InferenceRule<P: Proposition>: 'static + Clone + PartialEq {}

#[derive(Clone,PartialEq,Eq,Debug)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<P: Proposition,Rule:InferenceRule<P>> {
    pub inference_type: Rule,
    pub assumptions: Box<[P]>,
    pub conclusion: P
}

impl <P: Proposition, Rule: InferenceRule<P>> ParentOfAssumptions<P> for Inference<P, Rule> {
    fn get_assumption_paths(&self) -> impl IntoIterator<Item = AssumptionInProofStepPath>
        { (0..self.assumptions.len()).map(|n| AssumptionInProofStepPath::from(n)) }

    fn get_assumption(&self,path: &AssumptionInProofStepPath) -> Result<&P,()>
        { self.assumptions.get(path.0).ok_or(()) }

    fn get_assumptions<'a>(&'a self) -> impl IntoIterator<Item = &'a P> where P: 'a { &self.assumptions }
    
    fn into_located_assumptions_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<P,AssumptionInProofStepPath>> where Self:Sized {
        self.assumptions.into_iter()
            .enumerate()
            .map(|(id,conclusion)| OwnedObjAtPath{obj: conclusion, path: AssumptionInProofStepPath::from(id)})
    }
}

impl <P: Proposition, Rule: InferenceRule<P>> ParentOfExplicitConclusions<P> for Inference<P, Rule> {
    fn get_explicit_conclusion_paths(&self) -> impl IntoIterator<Item = ExplicitConclusionInProofStepPath>
        { [ExplicitConclusionInProofStepPath(0)] }

    fn get_explicit_conclusion(&self,path: &ExplicitConclusionInProofStepPath) -> Result<&P,()>
        { if path.0 == 0 { Ok(&self.conclusion) } else { Err(()) } }

    fn get_explicit_conclusions<'a>(&'a self) -> impl IntoIterator<Item = &'a P> where P: 'a { [&self.conclusion] }

    fn into_located_explicit_conclusions_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<P,ExplicitConclusionInProofStepPath>> where Self:Sized
        { [OwnedObjAtPath{obj: self.conclusion, path: ExplicitConclusionInProofStepPath(0)}] }
}

pub type InferenceInProof<'a, Proposition,Rule> = ObjAtPath<'a,Inference<Proposition,Rule>,ProofInProofPath>;
pub type OwnedInferenceInProof<Proposition,Rule> = OwnedObjAtPath<Inference<Proposition,Rule>,ProofInProofPath>;
