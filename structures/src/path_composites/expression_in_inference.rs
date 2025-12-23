use std::fmt::Display;

use path_lib::{obj_at_path::{ObjAtPath, OwnedObjAtPath}, Path};

use crate::{DisplayExt, expressions::{Expression, ExpressionInExpressionPath}, proof::PropositionInInferencePath};

#[derive(Clone,PartialEq,Eq)]
pub struct ExpressionInInferencePath {
    pub proposition_path: PropositionInInferencePath,
    pub subexpression_path: ExpressionInExpressionPath,
}
impl Path for ExpressionInInferencePath {}
impl Display for ExpressionInInferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}{}",self.proposition_path,self.subexpression_path.display())
    }
}



#[derive(Clone,PartialEq,Eq)]
pub struct ExpressionInInference<'a>(pub ObjAtPath<'a,Expression,ExpressionInInferencePath>);
impl <'a> ExpressionInInference<'a> {
    pub fn into_owned(self) -> OwnedExpressionInInference { OwnedExpressionInInference(self.0.into_owned()) }
}

#[derive(Clone,PartialEq,Eq)]
pub struct OwnedExpressionInInference(pub OwnedObjAtPath<Expression,ExpressionInInferencePath>);

mod from {
    use path_lib::paths::{PathPair, PathSeries};

    use crate::{expressions::AtomicExpressionInExpressionPath, proof::PropositionInInferencePath};

    use super::*;

    impl From<PropositionInInferencePath> for ExpressionInInferencePath {
        fn from(path: PropositionInInferencePath) -> Self { Self {
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
    impl From<PathPair<PropositionInInferencePath,ExpressionInExpressionPath>> for ExpressionInInferencePath {
        fn from(value: PathPair<PropositionInInferencePath,ExpressionInExpressionPath>) -> Self { Self {
                proposition_path: value.left,
                subexpression_path: value.right
        }}
    }
    impl From<PathPair<PropositionInInferencePath,AtomicExpressionInExpressionPath>> for ExpressionInInferencePath {
        fn from(value: PathPair<PropositionInInferencePath,AtomicExpressionInExpressionPath>) -> Self { Self {
                proposition_path: value.left,
                subexpression_path: PathSeries::new([value.right])
        }}
    }
}