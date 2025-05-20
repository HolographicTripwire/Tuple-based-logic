use crate::{propositions::{Proposition, Term}};


pub struct TupleOrError<E: Clone> {
    pub error: E
}

impl <E: Clone> TupleOrError<E> {
    pub fn prop_as_tuple<'a>(&self, proposition: &'a Proposition) -> Result<&'a Vec<Term>,E> {
        proposition.0.as_tuple().or(Err(self.error.clone()))
    }

    pub fn term_as_tuple<'a>(&self, term: &'a Term) -> Result<&'a Vec<Term>,E> {
        term.as_tuple().or(Err(self.error.clone()))
    }

    pub fn prop_as_slice<'a>(&self, proposition: &'a Proposition) -> Result<&'a [Term],E> {
        proposition.0.as_slice().or(Err(self.error.clone()))
    }

    pub fn term_as_slice<'a>(&self, term: &'a Term) -> Result<&'a [Term],E> {
        term.as_slice().or(Err(self.error.clone()))
    }
}
