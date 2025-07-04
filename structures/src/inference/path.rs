use std::marker::PhantomData;

use path_lib::{paths::{PathPair, PathPrimitive}, AtPath, HasChildren};

use crate::{inference::{Inference, InferenceRule}, propositions::{Expression, Proposition, SubexpressionPath}};

#[derive(Clone)]
pub struct InferencePropositionPath<Rule: InferenceRule> {
    is_conclusion: bool,
    proposition_index: usize,
    phantom: PhantomData<Rule>
}
impl <Rule: InferenceRule> InferencePropositionPath<Rule> {
    pub fn new(is_conclusion: bool, proposition_index: usize) -> Self { Self { is_conclusion, proposition_index, phantom: PhantomData } }
    pub fn assumption(assumption_index: usize) -> Self { Self::new(false, assumption_index) }
    pub fn conclusion(conclusion_index: usize) -> Self { Self::new(true, conclusion_index) }
}
impl <Rule: InferenceRule> PathPrimitive for InferencePropositionPath<Rule> {}

impl <'a, Rule: 'a + InferenceRule> HasChildren<'a,InferencePropositionPath<Rule>, Proposition> for Inference<Rule> {
    fn children(&'a self) -> impl IntoIterator<Item = &'a Proposition> {
        self.assumptions.iter().chain(self.conclusions.iter())
    }

    fn get_child(&'a self, path: &InferencePropositionPath<Rule>) -> Result<&'a Proposition,()> {
        let propositions = if path.is_conclusion { &self.assumptions } else { &self.conclusions };
        propositions.get(path.proposition_index).ok_or(())
    }
}

pub type PropositionInInference<'a,Rule> = AtPath<'a,InferencePropositionPath<Rule>,Proposition>;

#[derive(Clone)]
pub struct InferenceSubexpressionPath<Rule: InferenceRule>(InferencePropositionPath<Rule>,SubexpressionPath);
impl <Rule: InferenceRule> InferenceSubexpressionPath<Rule> {
    pub fn new(is_conclusion: bool, proposition_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { (InferencePropositionPath::new(is_conclusion, proposition_index), subexpression_path).into() }
    pub fn assumption(assumption_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { (InferencePropositionPath::assumption(assumption_index), subexpression_path).into() }
    pub fn conclusion(conclusion_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { (InferencePropositionPath::conclusion(conclusion_index), subexpression_path).into() }
}
impl <Rule: InferenceRule> Into<PathPair<InferencePropositionPath<Rule>,SubexpressionPath>> for InferenceSubexpressionPath<Rule> {
    fn into(self) -> PathPair<InferencePropositionPath<Rule>,SubexpressionPath> { PathPair::new(self.0,self.1) }
}
impl <Rule: InferenceRule, IL: Into<InferencePropositionPath<Rule>>, IR: Into<SubexpressionPath>> From<(IL,IR)> for InferenceSubexpressionPath<Rule> {
    fn from(value: (IL,IR)) -> Self { Self(value.0.into(),value.1.into()) }
}

pub type SubexpressionInInference<'a,Rule> = AtPath<'a,InferenceSubexpressionPath<Rule>,Expression>;
