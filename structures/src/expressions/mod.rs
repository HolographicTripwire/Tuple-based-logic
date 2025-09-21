mod propositions;
pub mod tuple_or_error;
mod subexpression_path;

pub use propositions::{Proposition,PropositionSet,get_contradictions};
pub use subexpression_path::*;

use crate::atoms::{AtomId, BuiltInAtom};

use tuple_or_error::TUPLE_OR_UNIT;

/// A compound unit in Tuple-Based Logic, which are used to build up [Propositions](Proposition)
#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Expression {
    Atomic(AtomId),
    Tuple(Vec<Expression>)
}

impl Expression {
    /// If this expression is an Atom, get its id. Otherwise throw an error
    pub fn as_atom(&self) -> Result<AtomId,()> {
        match self {
            Expression::Atomic(entity_id) => Ok(*entity_id),
            Expression::Tuple(_) => Err(()),
        }
    }

    /// If this expression is a Tuple, get its expressions. Otherwise throw an error 
    pub fn as_vec<'a>(&'a self) -> Result<&'a Vec<Expression>,()> { 
        match self {
            Expression::Atomic(_) => Err(()),
            Expression::Tuple(proposition_exprs) => Ok(proposition_exprs),
        }
    }

    /// If this expression is a Tuple, get its subexpressions. Otherwise throw an error 
    pub fn as_slice(&self) -> Result<&[Expression], ()> {
        match self {
            Expression::Atomic(_) => Err(()),
            Expression::Tuple(proposition_exprs) => Ok(proposition_exprs.as_slice()),
        }
    }

    /// Get the expression which is negated by this expression
    /// Returns Some(&negated_expression) if this expression is the negation of some negated_expression, otherwise returns None
    /// For example, get_negated((¬,(¬,P))) = (¬,P)
    pub fn get_negated(&self) -> Option<&Expression> {
        // Try splitting this atom into two components. On failure, this expression is not a well-formed negation, so return None
        let Ok([negation_atom, negated_expression]) = TUPLE_OR_UNIT.as_slice(self) else { return None; };
        // If the head of this expression is not a negation atom, this expression is not a well-formed negation, so return false
        if negation_atom != &BuiltInAtom::Negation.into() { return None; }
        // We now know that the expression is a well-formed negation, so we return the expression being negated
        Some(negated_expression)
    }

    /// Check if this expression is the negation of another
    pub fn is_negation_of(&self, other: &Expression) -> bool { self.get_negated() == Some(other) }

    /// Get the number of negations that this proposition begins with
    /// Note that a negation level is only counted if that level contains two terms - where one is the negation.
    /// So, (¬,(¬,P)) counts as two, but (¬,(¬,P,Q)) and (¬,(¬)) only count as one
    pub fn negation_level(&self) -> usize {
        // Inductive case; if this expression negates something, then its negation level is the negated expressoin's negation level plus one
        if let Some(negated_expression) = self.get_negated()
            { negated_expression.negation_level() + 1 }
        // Base case; if this expression doesn't negate anything than the negation level is zero
        else { 0 }
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
}
