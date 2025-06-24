use crate::propositions::Expression;

pub struct SubexpressionPath(Box<[usize]>);
impl <T: Into<Box<[usize]>>> From<T> for SubexpressionPath {
    fn from(value: T) -> Self { SubexpressionPath(value.into()) }
}

impl Expression {
    pub fn get_subexpression(&self, at_postition: impl Into<SubexpressionPath>) -> Result<&Expression,()> {
        self.get_subexpression_inner(&at_postition.into().0)
    }
    
    fn get_subexpression_inner(&self, at_position: &[usize]) -> Result<&Expression,()> {
        if at_position.len() == 0 { return Ok(self) }
        let Ok(vec) = self.as_vec() else { return Err(()) };
        let Some(first_index) = at_position.get(0) else { return Err(()) };
        let Some(subexpression) = vec.get(*first_index) else { return Err(()) };
        Ok(subexpression.get_subexpression_inner(&at_position[1..])?)
    }
}
