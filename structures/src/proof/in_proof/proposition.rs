
use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}};

use crate::{expressions::{Proposition, subexpression::ExpressionInExpressionPath}, path_composites::ExpressionInInferencePath};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct AssumptionInProofStepPath(pub usize);
#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct ExplicitConclusionInProofStepPath(pub usize);

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub enum PropositionInProofStepPath {
    Assumption(AssumptionInProofStepPath),
    ExplicitConclusion(ExplicitConclusionInProofStepPath)
}
impl PropositionInProofStepPath {
    pub fn new(is_conclusion: bool, proposition_index: usize) -> Self { match is_conclusion {
        true => Self::assumption(proposition_index),
        false => Self::explicit_conclusion(proposition_index),
    } }
    pub fn assumption(assumption_index: usize) -> Self { Self::Assumption(AssumptionInProofStepPath(assumption_index)) }
    pub fn explicit_conclusion(conclusion_index: usize) -> Self { Self::ExplicitConclusion(ExplicitConclusionInProofStepPath(conclusion_index)) }
}

impl Display for PropositionInProofStepPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self {
        PropositionInProofStepPath::Assumption(assumption_path) => write!(f,"A{}",assumption_path.0),
        PropositionInProofStepPath::ExplicitConclusion(conclusion_path) => write!(f,"C{}",conclusion_path.0),
    }}
}

pub type AssumptionInProofStep<'a> = ObjAtPath<'a,Proposition,AssumptionInProofStepPath>;
pub type OwnedAssumptionInProofStep = OwnedObjAtPath<Proposition,AssumptionInProofStepPath>;

pub type ExplicitConclusionInProofStep<'a> = ObjAtPath<'a,Proposition,ExplicitConclusionInProofStepPath>;
pub type OwnedExplicitConclusionInProofStep = OwnedObjAtPath<Proposition,ExplicitConclusionInProofStepPath>;

pub type PropositionInProofStep<'a> = ObjAtPath<'a,Proposition,PropositionInProofStepPath>;
pub type OwnedPropositionInProofStep = OwnedObjAtPath<Proposition,PropositionInProofStepPath>;

// impl <'a> Into<ExpressionInInference<'a>> for PropositionInProofStep<'a> {
//     fn into(self) -> ExpressionInInference<'a> {
//         let (obj, path) = self.0.into_obj_and_path();
//         ExpressionInInference::from_inner(obj, ExpressionInInferencePath {
//             proposition_path: path,
//             subexpression_path: ExpressionInExpressionPath::default()
//         })
//     }
// }
