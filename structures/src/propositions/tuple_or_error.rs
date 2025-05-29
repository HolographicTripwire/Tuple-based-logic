use crate::{propositions::{Proposition, Expression}};

pub const TUPLE_OR_NONE: TupleOrError<()> = TupleOrError{ error: () };

pub struct TupleOrError<E: Clone> {
    pub error: E
}

impl <E: Clone> TupleOrError<E> {
    pub fn prop_as_tuple<'a>(&self, proposition: &'a Proposition) -> Result<&'a Vec<Expression>,E> {
        proposition.0.as_tuple().or(Err(self.error.clone()))
    }

    pub fn expr_as_tuple<'a>(&self, expr: &'a Expression) -> Result<&'a Vec<Expression>,E> {
        expr.as_tuple().or(Err(self.error.clone()))
    }

    pub fn prop_as_slice<'a>(&self, proposition: &'a Proposition) -> Result<&'a [Expression],E> {
        proposition.0.as_slice().or(Err(self.error.clone()))
    }

    pub fn expr_as_slice<'a>(&self, expr: &'a Expression) -> Result<&'a [Expression],E> {
        expr.as_slice().or(Err(self.error.clone()))
    }
}
