use crate::{inference::{Inference, InferenceRule}, propositions::{Expression, SubexpressionPath}};

#[derive(Clone)]
pub struct InferenceSubexpressionPath {
    pub is_conclusion: bool,
    pub proposition_index: usize,
    pub subexpression_path: SubexpressionPath
}
impl InferenceSubexpressionPath {
    pub fn new(is_conclusion: bool, expression_index: usize, subexpression_path: impl Into<SubexpressionPath>) -> Self
        { Self {is_conclusion, proposition_index: expression_index, subexpression_path: subexpression_path.into()} }
    
    pub fn assumption(assumption_index: usize, assumption_subpath: impl Into<SubexpressionPath>) -> Self
        { Self::new(false,assumption_index,assumption_subpath.into()) }
    pub fn conclusion(conclusion_index: usize, conclusion_subpath: impl Into<SubexpressionPath>) -> Self
        { Self::new(true, conclusion_index, conclusion_subpath.into()) }

    pub fn join<'a>(&self, subpath: impl Into<&'a SubexpressionPath>) -> Self {
        Self {
            is_conclusion: self.is_conclusion,
            proposition_index: self.proposition_index,
            subexpression_path: self.subexpression_path.join(subpath)
        }
    }
}

#[derive(Clone)]
pub struct SubexpressionInInference<'a> {
    expression: &'a Expression,
    path: InferenceSubexpressionPath
}
impl <'a> SubexpressionInInference<'a> {
    pub fn new<Rule: InferenceRule>(inference: &'a Inference<Rule>, path: InferenceSubexpressionPath) -> Result<Self,()> {
        let expression = inference.get_subexpression(&path)?;
        Ok(Self { expression, path })
    }

    pub fn expression(&self) -> &'a Expression { self.expression }
    pub fn path(&self) -> &InferenceSubexpressionPath { &self.path }

    pub fn join<'b>(&self, subpath: impl Into<&'b SubexpressionPath>) -> Result<Self,()> {
        let subpath = subpath.into();
        Ok(Self {
            expression: self.expression.get_subexpression(subpath)?,
            path: self.path.join(subpath)
        })
    }

    pub fn subexpressions(&self) -> Result<Vec<SubexpressionInInference>,()> {
        let vec = self.expression.as_vec()?;
        Ok((0..vec.len())
            .map(|index| self.join(&vec![index].into()).expect(&format!("Conclusion {index} not found")))
            .collect::<Vec<SubexpressionInInference>>())
    }
}
