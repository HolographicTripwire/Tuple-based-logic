use std::fmt::Display;

use path_lib::{paths::PathPair, Path};
use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{DisplayExt, expressions::{AtomicExpressionInExpressionPath, Expression, ExpressionInExpressionPath}, proof::{ProofInProofPath, inference::PropositionInInferencePath}};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ExpressionInProofPath{
    pub step_path: ProofInProofPath,
    pub proposition_path: PropositionInInferencePath,
    pub subexpression_path: ExpressionInExpressionPath
}

impl Path for ExpressionInProofPath {}
impl Display for ExpressionInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}{}",self.step_path.display(),self.proposition_path,self.subexpression_path.display())
    }
}

generate_obj_at_path_wrappers!{
    (Expression), ExpressionInExpressionPath,
    "ExpressionInProof", [Clone, PartialEq, Eq, Debug],
    "OwnedExpressionInProof", [Clone, PartialEq, Eq, Debug]
}

mod from {
    use path_lib::paths::PathSeries;

    use crate::path_composites::PropositionInProofPath;

    use super::*;

    impl From<PropositionInProofPath> for ExpressionInProofPath {
        fn from(path: PropositionInProofPath) -> Self { Self {
            step_path: path.step_path,
            proposition_path: path.proposition_path,
            subexpression_path: ExpressionInExpressionPath::empty(),
        }}
    }
    impl From<PathPair<ExpressionInProofPath,AtomicExpressionInExpressionPath>> for ExpressionInProofPath {
        fn from(mut value: PathPair<ExpressionInProofPath,AtomicExpressionInExpressionPath>) -> Self { 
            value.left.subexpression_path.append(value.right);
            value.left
        }
    }
    impl From<PathPair<ExpressionInProofPath,ExpressionInExpressionPath>> for ExpressionInProofPath {
        fn from(mut value: PathPair<ExpressionInProofPath,ExpressionInExpressionPath>) -> Self { 
            value.left.subexpression_path.append_all(value.right.into_paths());
            value.left
        }
    }
    impl From<PathPair<PropositionInProofPath,ExpressionInExpressionPath>> for ExpressionInProofPath {
        fn from(value: PathPair<PropositionInProofPath,ExpressionInExpressionPath>) -> Self { Self {
                step_path: value.left.step_path,
                proposition_path: value.left.proposition_path,
                subexpression_path: value.right
        }}
    }
    impl From<PathPair<PropositionInProofPath,AtomicExpressionInExpressionPath>> for ExpressionInProofPath {
        fn from(value: PathPair<PropositionInProofPath,AtomicExpressionInExpressionPath>) -> Self { Self {
                step_path: value.left.step_path,
                proposition_path: value.left.proposition_path,
                subexpression_path: PathSeries::new([value.right])
        }}
    }
    impl From<PathPair<ProofInProofPath,PropositionInInferencePath>> for ExpressionInProofPath {
        fn from(value: PathPair<ProofInProofPath,PropositionInInferencePath>) -> Self { Self {
            step_path: value.left,
            proposition_path: value.right,
            subexpression_path: ExpressionInExpressionPath::empty(),
        }}
    }
}