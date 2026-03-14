use std::fmt::Display;

use path_lib::Path;
use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{DisplayExt, expressions::{Expression, ExpressionInExpressionPath}, proof::PropositionInProofStepPath};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct ExpressionInInferencePath {
    pub proposition_path: PropositionInProofStepPath,
    pub subexpression_path: ExpressionInExpressionPath,
}
impl Path for ExpressionInInferencePath {}
impl Display for ExpressionInInferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.proposition_path,self.subexpression_path.display())
    }
}

generate_obj_at_path_wrappers!{
    (Expression), ExpressionInInferencePath,
    "ExpressionInInference", [Clone, PartialEq, Eq, Debug],
    "OwnedExpressionInInference", [Clone, PartialEq, Eq, Debug]
}

mod from {
    use path_lib::paths::{PathPair, PathSeries};

    use crate::expressions::AtomicExpressionInExpressionPath;

    use super::*;

    impl From<PropositionInProofStepPath> for ExpressionInInferencePath {
        fn from(path: PropositionInProofStepPath) -> Self { Self {
            proposition_path: path,
            subexpression_path: ExpressionInExpressionPath::empty(),
        }}
    }
    impl From<PathPair<ExpressionInInferencePath,AtomicExpressionInExpressionPath>> for ExpressionInInferencePath {
        fn from(mut value: PathPair<ExpressionInInferencePath,AtomicExpressionInExpressionPath>) -> Self { 
            value.left.subexpression_path.append(value.right);
            value.left
        }
    }
    impl From<PathPair<ExpressionInInferencePath,ExpressionInExpressionPath>> for ExpressionInInferencePath {
        fn from(mut value: PathPair<ExpressionInInferencePath,ExpressionInExpressionPath>) -> Self { 
            value.left.subexpression_path.append_all(value.right.into_paths());
            value.left
        }
    }
    impl From<PathPair<PropositionInProofStepPath,ExpressionInExpressionPath>> for ExpressionInInferencePath {
        fn from(value: PathPair<PropositionInProofStepPath,ExpressionInExpressionPath>) -> Self { Self {
                proposition_path: value.left,
                subexpression_path: value.right
        }}
    }
    impl From<PathPair<PropositionInProofStepPath,AtomicExpressionInExpressionPath>> for ExpressionInInferencePath {
        fn from(value: PathPair<PropositionInProofStepPath,AtomicExpressionInExpressionPath>) -> Self { Self {
                proposition_path: value.left,
                subexpression_path: PathSeries::new([value.right])
        }}
    }
}