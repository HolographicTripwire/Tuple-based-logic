pub mod atomic;
pub mod compound;
pub mod at_path_enum;
pub mod subexpression;
pub mod signatures;
// pub mod tuple_or_error;

pub use signatures::*;
pub use crate::propositions::{Proposition,PropositionSet};

use crate::expressions::{atomic::AtomicExpression, compound::CompoundExpression};

/// A compound unit in Tuple-Based Logic, which are used to build up [Propositions](Proposition)
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Expression {
    Atomic(AtomicExpression),
    Compound(CompoundExpression)
}

impl Expression {
    /// If this expression is an Atom, get its id. Otherwise throw an error
    pub fn as_atom(&self) -> Result<AtomicExpression,()> {
        match self {
            Expression::Atomic(entity_id) => Ok(*entity_id),
            Expression::Compound(_) => Err(()),
        }
    }

    /// If this expression is a Tuple, get its expressions. Otherwise throw an error 
    pub fn as_vec<'a>(&'a self) -> Result<&'a CompoundExpression,()> { 
        match self {
            Expression::Atomic(_) => Err(()),
            Expression::Compound(proposition_exprs) => Ok(proposition_exprs),
        }
    }

    /// If this expression is a Tuple, get its subexpressions. Otherwise throw an error 
    pub fn as_slice(&self) -> Result<&[Expression], ()> {
        match self {
            Expression::Atomic(_) => Err(()),
            Expression::Compound(proposition_exprs) => Ok(proposition_exprs.0.as_slice()),
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            Expression::Atomic(_) => None,
            Expression::Compound(exprs) => Some(exprs.0.len())
        }
    }
}

mod from {
    use crate::expressions::{CompoundExpression, Expression, atomic::AtomicExpression};

    impl From<AtomicExpression> for Expression {
        fn from(id: AtomicExpression) -> Self
            { Self::Atomic(id) }
    }
    impl From<u16> for Expression {
        fn from(id: u16) -> Self
            { AtomicExpression(id).into() }
    }
    impl From<CompoundExpression> for Expression {
        fn from(expr: CompoundExpression) -> Self
            { Self::Compound(expr) }
    }
    impl From<Vec<Expression>> for Expression {
        fn from(exprs: Vec<Expression>) -> Self
            { CompoundExpression(exprs).into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_atom_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::from(AtomicExpression(i));
            assert_eq!(atomic_expr.as_atom(), Ok(AtomicExpression(i)));
        }
    }

    #[test]
    fn test_as_atom_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomicExpression(i))]);
            assert_eq!(atomic_expr.as_atom(), Err(()));
        }
    }

    #[test]
    fn test_as_tuple_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::Atomic(AtomicExpression(i));
            assert_eq!(atomic_expr.as_vec(), Err(()));
        }
    }

    #[test]
    fn test_as_tuple_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomicExpression(i))]);
            assert_eq!(atomic_expr.as_vec(), Ok(&CompoundExpression(vec![Expression::from(AtomicExpression(i))])));
        }
    }

    #[test]
    fn test_as_slice_on_atom() {
        for i in 0..10 {
            let atomic_expr = Expression::Atomic(AtomicExpression(i));
            assert_eq!(atomic_expr.as_slice(), Err(()));
        }
    }

    #[test]
    fn test_as_slice_on_tuple() {
        for i in 0..10 {
            let atomic_expr = Expression::from(vec![Expression::from(AtomicExpression(i))]);
            assert_eq!(atomic_expr.as_slice(), Ok(vec![Expression::from(AtomicExpression(i))].as_slice()));
        }
    }
}
