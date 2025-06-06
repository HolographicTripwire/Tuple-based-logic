use crate::{propositions::{Proposition, Expression}};

pub const TUPLE_OR_NONE: TupleOrError<()> = TupleOrError{ error: () };

pub struct TupleOrError<E: Clone> {
    pub error: E
}

impl <E: Clone> TupleOrError<E> {
    /// Turn the provided [Proposition] into a [Vec] of [Expression] objects if it is a tuple
    pub fn prop_as_tuple<'a>(&self, proposition: &'a Proposition) -> Result<&'a Vec<Expression>,E> {
        proposition.0.as_tuple().or(Err(self.error.clone()))
    }

    /// Turn the provided [Expression] into a [Vec] of [Expression] objects if it is a tuple
    pub fn expr_as_tuple<'a>(&self, expr: &'a Expression) -> Result<&'a Vec<Expression>,E> {
        expr.as_tuple().or(Err(self.error.clone()))
    }

    /// Turn the provided [Proposition] into a slice of [Expression] objects if it is a tuple
    pub fn prop_as_slice<'a>(&self, proposition: &'a Proposition) -> Result<&'a [Expression],E> {
        proposition.0.as_slice().or(Err(self.error.clone()))
    }

    /// Turn the provided [Expression] into a slice of [Expression] objects if it is a tuple
    pub fn expr_as_slice<'a>(&self, expr: &'a Expression) -> Result<&'a [Expression],E> {
        expr.as_slice().or(Err(self.error.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::atoms::AtomId;

    use super::*;

    fn atomic_expression_vec(nums: Vec<usize>) -> Vec<Expression> {
        nums.iter()
            .map(|num| -> Expression {
                Expression::Atomic(AtomId::try_from(*num).unwrap())
            }).collect()
    }

    #[test]
    fn test_prop_as_tuple_with_atom() {
        let expression = Expression::Atomic((AtomId::try_from(1)).unwrap());
        let proposition = Proposition(expression);
        assert_eq!(TUPLE_OR_NONE.prop_as_tuple(&proposition), Err(()));
    }

    #[test]
    fn test_prop_as_tuple_with_tuple() {
        let expressions = atomic_expression_vec(vec![0,1,2]);
        let combined_expression = Expression::Tuple(expressions.clone());
        let proposition = Proposition(combined_expression);
        assert_eq!(TUPLE_OR_NONE.prop_as_tuple(&proposition), Ok(&expressions));
    }

    #[test]
    fn test_expr_as_tuple_with_atom() {
        let expression = Expression::Atomic((AtomId::try_from(1)).unwrap());
        assert_eq!(TUPLE_OR_NONE.expr_as_tuple(&expression), Err(()));
    }

    #[test]
    fn test_expr_as_tuple_with_tuple() {
        let expressions = atomic_expression_vec(vec![0,1,2]);
        let combined_expression = Expression::Tuple(expressions.clone());
        assert_eq!(TUPLE_OR_NONE.expr_as_tuple(&combined_expression), Ok(&expressions));
    }

    #[test]
    fn test_prop_as_slice_with_atom() {
        let expression = Expression::Atomic((AtomId::try_from(1)).unwrap());
        let proposition = Proposition(expression);
        assert_eq!(TUPLE_OR_NONE.prop_as_slice(&proposition), Err(()));
    }

    #[test]
    fn test_prop_as_slice_with_tuple() {
        let expressions = atomic_expression_vec(vec![0,1,2]);
        let combined_expression = Expression::Tuple(expressions.clone());
        let proposition = Proposition(combined_expression);
        assert_eq!(TUPLE_OR_NONE.prop_as_slice(&proposition), Ok(expressions.as_slice()));
    }

    #[test]
    fn test_expr_as_slice_with_atom() {
        let expression = Expression::Atomic((AtomId::try_from(1)).unwrap());
        assert_eq!(TUPLE_OR_NONE.expr_as_slice(&expression), Err(()));
    }

    #[test]
    fn test_expr_as_slice_with_tuple() {
        let expressions = atomic_expression_vec(vec![0,1,2]);
        let combined_expression = Expression::Tuple(expressions.clone());
        assert_eq!(TUPLE_OR_NONE.expr_as_slice(&combined_expression), Ok(expressions.as_slice()));
    }
}
