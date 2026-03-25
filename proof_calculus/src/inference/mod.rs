use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{inference::{propositions::{ParentOfAntecedents, ParentOfConsequents, paths::{AntecedentInInferencePath, ConsequentInInferencePath}}, rules::InferenceRule}, sequential_proofs::{propositions::{ParentOfAssumptions, ParentOfExplicitConclusions}, subproof::ProofInProofPath}};

pub mod propositions;
pub mod rules;

#[derive(Clone,PartialEq,Eq,Debug)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Proposition,Rule:InferenceRule<Proposition>> {
    pub inference_type: Rule,
    pub assumptions: Box<[Proposition]>,
    pub conclusion: Proposition
}

impl <Proposition, Rule: InferenceRule<Proposition>> ParentOfAntecedents<Proposition> for Inference<Proposition, Rule> {
    fn get_antecedent_paths(&self) -> impl IntoIterator<Item = AntecedentInInferencePath>
        { (0..self.assumptions.len()).map(|n| AntecedentInInferencePath(n)) }

    fn get_antecedent(&self,path: &AntecedentInInferencePath) -> Result<&Proposition,()>
        { self.assumptions.get(path.0).ok_or(()) }

    fn get_antecedents(&self) -> impl IntoIterator<Item = &Proposition> { &self.assumptions }
    
    fn into_located_antecedents_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,AntecedentInInferencePath>> where Proposition:Clone,Self:Sized {
        self.assumptions.into_iter()
            .enumerate()
            .map(|(id,conclusion)| OwnedObjAtPath{obj: conclusion, path: AntecedentInInferencePath(id)})
    }
}
impl <Proposition, Rule: InferenceRule<Proposition>> ParentOfConsequents<Proposition> for Inference<Proposition, Rule> {
    fn get_consequent_paths(&self) -> impl IntoIterator<Item = ConsequentInInferencePath>
        { [ConsequentInInferencePath] }

    fn get_consequent(&self,path: &ConsequentInInferencePath) -> Result<&Proposition,()>
        { Ok(&self.conclusion) }

    fn get_consequents(&self) -> impl IntoIterator<Item = &Proposition> { [&self.conclusion] }

    fn into_located_consequents_owned(self) -> impl IntoIterator<Item = OwnedObjAtPath<Proposition,ConsequentInInferencePath>> where Proposition:Clone,Self:Sized
        { [self.conclusion] }
}

pub type InferenceInProof<'a, Proposition,Rule> = ObjAtPath<'a,Inference<Proposition,Rule>,ProofInProofPath>;
pub type OwnedInferenceInProof<Proposition,Rule> = OwnedObjAtPath<Inference<Proposition,Rule>,ProofInProofPath>;
