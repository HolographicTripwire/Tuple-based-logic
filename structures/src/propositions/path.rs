use crate::propositions::Expression;

#[derive(Clone)]
pub struct SubexpressionPath(Box<[usize]>);
impl <T: Into<Box<[usize]>>> From<T> for SubexpressionPath {
    fn from(value: T) -> Self { SubexpressionPath(value.into()) }
}
impl SubexpressionPath {
    pub fn join<'a>(&self, other: impl Into<&'a SubexpressionPath>) -> Self
        { Self([self.0.clone(),other.into().0.clone()].concat().into()) }
}

impl Expression {
    /// Get the subexpression within this expression at the provided [SubexpressionPath] if it exists, otherwise throw an error.
    pub fn get_subexpression<'a>(&self, at_postition: impl Into<&'a SubexpressionPath>) -> Result<&Expression,()> {
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

#[cfg(test)]
mod tests {
    use crate::atoms::AtomId;

    use super::*;

    #[test]
    fn test_get_subexpr_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::from(AtomId(i));
            assert_eq!(atomic_expr.get_subexpression(&vec![0].into()), Err(()));
        }
    }

    #[test]
    fn test_get_subexpr_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            assert_eq!(atomic_expr.get_subexpression(&vec![0].into()), Ok(&Expression::from(AtomId(i))));
        }
    }

    #[test]
    fn test_get_subexpr_on_short_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            assert_eq!(atomic_expr.get_subexpression(&vec![1].into()), Err(()));
        }
    }
}
