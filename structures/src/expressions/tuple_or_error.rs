use crate::expressions::{Expression, compound::CompoundExpression};

pub const TUPLE_OR_UNIT: TupleOrError<()> = TupleOrError{ error: () };

// Converts expressions to a given type of they are a tuple. Otherwise throws the stored error
pub struct TupleOrError<E: Clone> {
    pub error: E
}

impl <E: Clone> TupleOrError<E> {
    /// Turn the provided [Expression] into a [Vec] of [Expression] objects if it is a tuple
    pub fn as_tuple<'a>(&self, expr: &'a Expression) -> Result<&'a CompoundExpression,E>
        { expr.as_vec().or(Err(self.error.clone())) }

    /// Turn the provided [Expression] into a slice of [Expression] objects if it is a tuple
    pub fn as_slice<'a>(&self, expr: &'a Expression) -> Result<&'a [Expression],E>
        { expr.as_slice().or(Err(self.error.clone())) }
}

#[cfg(test)]
mod tests {
    use crate::expressions::atomic::AtomicExpression;

    use super::*;

    fn atomic_expression_vec(nums: Vec<usize>) -> Vec<Expression> {
        nums.iter()
            .map(|num| -> Expression {
                Expression::Atomic(AtomicExpression::try_from(*num).unwrap())
            }).collect()
    }

    #[test]
    fn test_expr_as_tuple_with_atom() {
        let expression = Expression::Atomic((AtomicExpression::try_from(1)).unwrap());
        assert_eq!(TUPLE_OR_UNIT.as_tuple(&expression), Err(()));
    }

    #[test]
    fn test_expr_as_tuple_with_tuple() {
        let expressions = atomic_expression_vec(vec![0,1,2]);
        let combined_expression = expressions.clone().into();
        assert_eq!(TUPLE_OR_UNIT.as_tuple(&combined_expression).map(|x| &x.0), Ok(&expressions));
    }

    #[test]
    fn test_expr_as_slice_with_atom() {
        let expression = Expression::Atomic((AtomicExpression::try_from(1)).unwrap());
        assert_eq!(TUPLE_OR_UNIT.as_slice(&expression), Err(()));
    }

    #[test]
    fn test_expr_as_slice_with_tuple() {
        let expressions = atomic_expression_vec(vec![0,1,2]);
        let combined_expression = expressions.clone().into();
        assert_eq!(TUPLE_OR_UNIT.as_slice(&combined_expression), Ok(expressions.as_slice()));
    }
}
