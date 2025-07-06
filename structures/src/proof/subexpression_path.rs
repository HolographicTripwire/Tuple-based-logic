use path_lib::{obj_at_path::ObjAtPath, paths::PathPair, Path};

use crate::{proof::ProofPropositionPath, expressions::{Expression, SubexpressionPath}};

#[derive(Clone)]
pub struct ProofSubexpressionPath(ProofPropositionPath,SubexpressionPath);
impl Path for ProofSubexpressionPath {}
impl ProofSubexpressionPath {
    pub fn new(is_conclusion: bool, proposition_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { (ProofPropositionPath::new(is_conclusion, proposition_index), subexpression_path).into() }
    pub fn assumption(assumption_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { (ProofPropositionPath::assumption(assumption_index), subexpression_path).into() }
    pub fn conclusion(conclusion_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { (ProofPropositionPath::conclusion(conclusion_index), subexpression_path).into() }
}
impl Into<PathPair<ProofPropositionPath,SubexpressionPath>> for ProofSubexpressionPath {
    fn into(self) -> PathPair<ProofPropositionPath,SubexpressionPath> { PathPair::new(self.0,self.1) }
}
impl <IL: Into<ProofPropositionPath>, IR: Into<SubexpressionPath>> From<(IL,IR)> for ProofSubexpressionPath {
    fn from(value: (IL,IR)) -> Self { Self(value.0.into(),value.1.into()) }
}

pub type SubexpressionInProof<'a> = ObjAtPath<'a,Expression,ProofSubexpressionPath>;
