use crate::propositions::{Expression, Proposition, SubexpressionPath};

#[derive(Clone)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Rule: InferenceRule> {
    pub inference_type: Rule,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}
impl <Rules: InferenceRule> Inference<Rules> {
    pub fn get_subexpression(&self, path: InferenceSubexpressionPath) -> Result<&Expression,()> {
        let vec = if path.is_conclusion { &self.conclusions } else { &self.assumptions };
        let Some(proposition) = vec.get(path.proposition_index) else { return Err(()) };
        proposition.get_subexpression(path.subexpression_path)
    }
}

pub trait InferenceRule: Clone {}

pub struct InferenceSubexpressionPath {
    is_conclusion: bool,
    proposition_index: usize,
    subexpression_path: SubexpressionPath
}
impl InferenceSubexpressionPath {
    fn new(is_conclusion: bool, expression_index: usize, subexpression_path: SubexpressionPath) -> Self
        { Self {is_conclusion, proposition_index: expression_index, subexpression_path} }
    pub fn assumption(assumption_index: usize, assumption_subpath: SubexpressionPath) -> Self { Self::new(false,assumption_index,assumption_subpath) }
    pub fn conclusion(conclusion_index: usize, conclusion_subpath: SubexpressionPath) -> Self { Self::new(true, conclusion_index, conclusion_subpath) }
}
