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
