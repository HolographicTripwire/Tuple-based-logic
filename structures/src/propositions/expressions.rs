use crate::atoms::{AtomId, BuiltInAtom};

use super::tuple_or_error::TUPLE_OR_NONE;

/// Components used in the construction of [Proposition] objects
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Expression {
    Atomic(AtomId),
    Tuple(Vec<Expression>)
}

impl Expression {
    // If this expression is an Atom, get its id. Otherwise throw an error
    pub fn as_atom(&self) -> Result<AtomId,()> {
        match &self {
            Expression::Atomic(entity_id) => Ok(*entity_id),
            Expression::Tuple(_) => Err(()),
        }
    }

    /// If this expression is a Tuple, get its expressions. Otherwise throw an error 
    pub fn as_vec(&self) -> Result<&Vec<Expression>,()> { 
        match &self {
            Expression::Atomic(_) => Err(()),
            Expression::Tuple(proposition_exprs) => Ok(proposition_exprs),
        }
    }

    /// If this expression is a Tuple, get its subexpressions. Otherwise throw an error 
    pub fn as_slice(&self) -> Result<&[Expression], ()> {
        match &self {
            Expression::Atomic(_) => Err(()),
            Expression::Tuple(proposition_exprs) => Ok(proposition_exprs.as_slice()),
        }
    }

    /// Get the subexpression within this expression at the provided index if it exists, otherwise throw an error.
    pub fn get_subexpr(&self, index: usize) -> Result<&Expression,()> {
        let exprs= self.as_vec()?;
        match exprs.get(index) {
            Some(expr) => Ok(expr),
            None => Err(()),
        }
    }

    pub fn is_negation_of(&self, other: &Expression) -> bool {
        let Ok([negation_atom, remainder]) = TUPLE_OR_NONE.expr_as_slice(self) else { return false; };
        if negation_atom != &BuiltInAtom::Negation.into() { return false; }
        else { return remainder == other }
    }

    pub fn negation_level(&self) -> usize {
        let Ok([negation_atom, remainder]) = TUPLE_OR_NONE.expr_as_slice(self) else { return 0; };
        if negation_atom != &BuiltInAtom::Negation.into() { return 0; }
        else { return remainder.negation_level() + 1; }
    }
}

impl From<AtomId> for Expression {
    fn from(id: AtomId) -> Self { Self::Atomic(id) }
}
impl From<Vec<Expression>> for Expression {
    fn from(exprs: Vec<Expression>) -> Self { Self::Tuple(exprs) }
}
impl From<BuiltInAtom> for Expression {
    fn from(atom: BuiltInAtom) -> Self { Self::from(AtomId::from(atom.into())) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_atom_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::from(AtomId(i));
            assert_eq!(atomic_expr.as_atom(), Ok(AtomId(i)));
        }
    }

    #[test]
    fn test_as_atom_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            assert_eq!(atomic_expr.as_atom(), Err(()));
        }
    }

    #[test]
    fn test_as_tuple_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::Atomic(AtomId(i));
            assert_eq!(atomic_expr.as_vec(), Err(()));
        }
    }

    #[test]
    fn test_as_tuple_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            assert_eq!(atomic_expr.as_vec(), Ok(&vec![Expression::from(AtomId(i))]));
        }
    }

    #[test]
    fn test_as_slice_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::Atomic(AtomId(i));
            assert_eq!(atomic_expr.as_slice(), Err(()));
        }
    }

    #[test]
    fn test_as_slice_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            assert_eq!(atomic_expr.as_slice(), Ok(vec![Expression::from(AtomId(i))].as_slice()));
        }
    }

    #[test]
    fn test_get_subexpr_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::from(AtomId(i));
            assert_eq!(atomic_expr.get_subexpr(0), Err(()));
        }
    }

    #[test]
    fn test_get_subexpr_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            assert_eq!(atomic_expr.get_subexpr(0), Ok(&Expression::from(AtomId(i))));
        }
    }

    #[test]
    fn test_get_subexpr_on_short_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomId(i))]);
            assert_eq!(atomic_expr.get_subexpr(1), Err(()));
        }
    }
}
