use std::marker::PhantomData;

use path_lib::{paths::{PathPair, PathPrimitive}, HasChildren};

use crate::{inference::{Inference, InferenceRule}, propositions::{Proposition, SubexpressionPath}};

#[derive(Clone)]
pub struct InferencePropositionPath<Rule: InferenceRule> {
    is_conclusion: bool,
    proposition_index: usize,
    phantom: PhantomData<Rule>
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

#[derive(Clone)]
pub struct InferenceSubexpressionPath<Rule: InferenceRule>(InferencePropositionPath<Rule>,SubexpressionPath);
impl <Rule: InferenceRule> Into<PathPair<InferencePropositionPath<Rule>,SubexpressionPath>> for InferenceSubexpressionPath<Rule> {
    fn into(self) -> PathPair<InferencePropositionPath<Rule>,SubexpressionPath> { PathPair::new(self.0,self.1) }
}
impl <Rule: InferenceRule> From<(InferencePropositionPath<Rule>,SubexpressionPath)> for InferenceSubexpressionPath<Rule> {
    fn from(value: (InferencePropositionPath<Rule>,SubexpressionPath)) -> Self { Self(value.0,value.1) }
}
