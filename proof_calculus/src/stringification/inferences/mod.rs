use std::marker::PhantomData;
use trait_aliases::trait_aliases;

use crate::{stringification::{Style, propositions::PropositionStyle}, structures::{Proposition, inferences::{Inference, InferenceRule}}};

trait_aliases!{
    pub trait InferenceRuleStyle<P: Proposition, Rule: InferenceRule<P>> = Style<Rule>;
}

#[derive(Clone)]
pub struct InferenceStyle<P: Proposition, PStyle: PropositionStyle<P>, Rule: InferenceRule<P>, RuleStyle: InferenceRuleStyle<P,Rule>> {
    pub expression_style: PStyle,
    pub rule_style: RuleStyle,
    phantom: PhantomData<(P,Rule)>
}

impl <P: Proposition, PStyle: PropositionStyle<P>, Rule: InferenceRule<P>, RuleStyle: InferenceRuleStyle<P,Rule>> Style<Inference<Rule>> for InferenceStyle<P,PStyle,Rule,RuleStyle> {
    fn parser<'a>(&self, params: Self::ParseParams) -> impl nom::Parser<String> {
        todo!()
    }

    fn format(&self, stylable: &Inference<P,Rule>) -> String {
        todo!()
    }
}
