use crate::{inference::path::{InferenceSubexpressionPath, SubexpressionInInference}, propositions::{Expression, Proposition}};

pub mod path;

#[derive(Clone)]
/// A struct representing a single inference step within a proof
/// Every [Inference] must be an instance of a particular [InferenceRule]; The tbl_verification crate allows this to be validated.
pub struct Inference<Rule: InferenceRule> {
    pub inference_type: Rule,
    pub assumptions: Vec<Proposition>,
    pub conclusions: Vec<Proposition>
}
impl <Rules: InferenceRule> Inference<Rules> {
    pub fn get_subexpression(&self, path: &InferenceSubexpressionPath) -> Result<&Expression,()> {
        let vec = if path.is_conclusion { &self.conclusions } else { &self.assumptions };
        let Some(proposition) = vec.get(path.proposition_index) else { return Err(()) };
        proposition.get_subexpression(&path.subexpression_path)
    }

    pub fn get_located_assumptions(&self) -> Vec<SubexpressionInInference> {
        (0..self.assumptions.len())
        .map(|index| SubexpressionInInference::new(self,
                InferenceSubexpressionPath::assumption(index, vec![])
            ).expect(&format!("Assumption {index} not found"))
        ).collect::<Vec<SubexpressionInInference>>()
    }
    pub fn get_located_conclusions(&self) -> Vec<SubexpressionInInference> {
        (0..self.conclusions.len())
        .map(|index| SubexpressionInInference::new(self,
                InferenceSubexpressionPath::conclusion(index, vec![])
            ).expect(&format!("Conclusion {index} not found"))
        ).collect::<Vec<SubexpressionInInference>>()
    }
}

pub trait InferenceRule: Clone {}
