use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, paths::{PathPair, PathSeries}, Path};

use crate::{expressions::{Expression, SubexpressionPath}, proof::ProofPropositionPath};

#[derive(Clone)]
pub struct ProofSubexpressionPath{
    pub proposition: ProofPropositionPath,
    pub subexpression: SubexpressionPath
}
impl Path for ProofSubexpressionPath {}
impl ProofSubexpressionPath {
    pub fn new(is_conclusion: bool, proposition_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { (ProofPropositionPath::new(is_conclusion, proposition_index), subexpression_path).into() }
    pub fn assumption(assumption_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { (ProofPropositionPath::assumption(assumption_index), subexpression_path).into() }
    pub fn conclusion(conclusion_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { (ProofPropositionPath::conclusion(conclusion_index), subexpression_path).into() }
}

pub type SubexpressionInProof<'a> = ObjAtPath<'a,Expression,ProofSubexpressionPath>;
pub type OwnedSubexpressionInProof = OwnedObjAtPath<Expression,ProofSubexpressionPath>;

mod from {
    use super::*;

    impl From<PathPair<ProofSubexpressionPath,SubexpressionPath>> for ProofSubexpressionPath {
        fn from(mut pair: PathPair<ProofSubexpressionPath,SubexpressionPath>) -> Self {
            pair.left.subexpression = PathSeries::new([pair.left.subexpression.into_paths(),pair.right.into_paths()].concat());
            pair.left
        }
    }
}
mod into {
    use super::*;

    impl Into<PathPair<ProofPropositionPath,SubexpressionPath>> for ProofSubexpressionPath {
        fn into(self) -> PathPair<ProofPropositionPath,SubexpressionPath> { PathPair::new(self.proposition,self.subexpression) }
    }
    impl <IL: Into<ProofPropositionPath>, IR: Into<SubexpressionPath>> From<(IL,IR)> for ProofSubexpressionPath {
        fn from(value: (IL,IR)) -> Self { Self{ proposition:value.0.into(), subexpression: value.1.into() } }
    }
}
