use std::fmt::Display;

use path_lib::paths::{PathPrimitive, PathSeries};
use path_lib_proc_macros::generate_obj_at_path_wrappers;

use crate::{expressions::Proposition, path_composites::{ExpressionInInference, ExpressionInInferencePath}};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct PropositionInInferencePath {
    pub is_conclusion: bool,
    pub proposition_index: usize
}
impl PropositionInInferencePath {
    pub fn new(is_conclusion: bool, proposition_index: usize) -> Self { Self { is_conclusion, proposition_index } }
    pub fn assumption(assumption_index: usize) -> Self { Self::new(false, assumption_index) }
    pub fn conclusion(conclusion_index: usize) -> Self { Self::new(true, conclusion_index) }
}
impl PathPrimitive for PropositionInInferencePath {}

impl Display for PropositionInInferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_conclusion { write!(f,"C{}",self.proposition_index) }
        else { write!(f,"A{}",self.proposition_index) }
    }
}

generate_obj_at_path_wrappers!{
    (Proposition), PropositionInInferencePath,
    "PropositionInInference", [Clone, PartialEq, Eq, Debug],
    "OwnedPropositionInInference", [Clone, PartialEq, Eq, Debug]
}
impl <'a> Into<ExpressionInInference<'a>> for PropositionInInference<'a> {
    fn into(self) -> ExpressionInInference<'a> {
        let (obj, path) = self.0.into_obj_and_path();
        ExpressionInInference::from_inner(obj, ExpressionInInferencePath {
            proposition_path: path,
            subexpression_path: PathSeries::empty()
        })
    }
}
