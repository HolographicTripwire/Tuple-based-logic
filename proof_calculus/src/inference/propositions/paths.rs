use std::fmt::Display;

use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::sequential_proofs::propositions::paths::AssumptionInProofStepPath;

pub type AntecedentInInferencePath = AssumptionInProofStepPath;
#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ConsequentInInferencePath;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub enum PropositionInInferencePath {
    Antecedent(AntecedentInInferencePath),
    Consequent(ConsequentInInferencePath)
}
impl PropositionInInferencePath {
    pub fn antecedent(assumption_index: usize) -> Self { Self::Antecedent(AssumptionInProofStepPath(assumption_index)) }
    pub fn consequent() -> Self { Self::Consequent(ConsequentInInferencePath) }
}

impl Display for PropositionInInferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self {
        PropositionInInferencePath::Antecedent(assumption_path) => write!(f,"A{}",assumption_path.0),
        PropositionInInferencePath::Consequent(conclusion_path) => write!(f,"C{}",conclusion_path.0),
    }}
}

pub type AssumptionInProofStep<'a,Proposition> = ObjAtPath<'a,Proposition,AntecedentInInferencePath>;
pub type OwnedAssumptionInProofStep<Proposition> = OwnedObjAtPath<Proposition,AntecedentInInferencePath>;

pub type ExplicitConclusionInProofStep<'a,Proposition> = ObjAtPath<'a,Proposition,ConsequentInInferencePath>;
pub type OwnedExplicitConclusionInProofStep<Proposition> = OwnedObjAtPath<Proposition,ConsequentInInferencePath>;

pub type PropositionInProofStep<'a,Proposition> = ObjAtPath<'a,Proposition,PropositionInInferencePath>;
pub type OwnedPropositionInProofStep<Proposition> = OwnedObjAtPath<Proposition,PropositionInInferencePath>;
