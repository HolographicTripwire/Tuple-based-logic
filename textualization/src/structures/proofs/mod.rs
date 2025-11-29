use tbl_structures::inference::InferenceRule;

use crate::structures::inferences::InferenceStyle;

#[derive(Clone)]
pub struct ProofStyle<'a,Rule: InferenceRule> {
    inference_style: InferenceStyle<'a,Rule>
}

impl <'a,Rule: InferenceRule> ProofStyle<'a,Rule> {
    pub fn inference_style(&self) -> &InferenceStyle<'a,Rule> { &self.inference_style }
}
