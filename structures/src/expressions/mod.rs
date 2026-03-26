pub mod atomic;
pub mod compound;
pub mod at_path_enum;
pub mod subexpression;
pub mod signatures;
// pub mod tuple_or_error;

pub use signatures::*;
pub use crate::propositions::{TblProposition,TblPropSet};

use crate::expressions::{atomic::AtomicTblExpression, compound::CompoundTblExpression};

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum TblExpression {
    Atomic(AtomicTblExpression),
    Compound(CompoundTblExpression)
}

impl TblExpression {
    pub fn replace(&self, to_replace: &TblExpression, replace_with: &TblExpression) -> TblExpression {
        if self == to_replace { replace_with.clone() }
        else if let TblExpression::Compound(compound) = self
            { TblExpression::Compound(compound.replace(to_replace, replace_with)) }
        else { self.clone() }
    }
    
    /// If this expression is an Atom, get its id. Otherwise throw an error
    pub fn as_atom(&self) -> Result<AtomicTblExpression,()> {
        match self {
            TblExpression::Atomic(entity_id) => Ok(*entity_id),
            TblExpression::Compound(_) => Err(()),
        }
    }

    /// If this expression is a Tuple, get its expressions. Otherwise throw an error 
    pub fn as_vec<'a>(&'a self) -> Result<&'a CompoundTblExpression,()> { 
        match self {
            TblExpression::Atomic(_) => Err(()),
            TblExpression::Compound(proposition_exprs) => Ok(proposition_exprs),
        }
    }

    /// If this expression is a Tuple, get its subexpressions. Otherwise throw an error 
    pub fn as_slice(&self) -> Result<&[TblExpression], ()> {
        match self {
            TblExpression::Atomic(_) => Err(()),
            TblExpression::Compound(proposition_exprs) => Ok(&proposition_exprs.0),
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            TblExpression::Atomic(_) => None,
            TblExpression::Compound(exprs) => Some(exprs.0.len())
        }
    }
}

mod from {
    use std::sync::Arc;

    use crate::expressions::{CompoundTblExpression, TblExpression, atomic::AtomicTblExpression};

    impl From<AtomicTblExpression> for TblExpression {
        fn from(id: AtomicTblExpression) -> Self
            { Self::Atomic(id) }
    }
    impl From<u16> for TblExpression {
        fn from(id: u16) -> Self
            { AtomicTblExpression(id).into() }
    }
    impl From<CompoundTblExpression> for TblExpression {
        fn from(expr: CompoundTblExpression) -> Self
            { Self::Compound(expr) }
    }
    impl <const N: usize> From<[TblExpression;N]> for TblExpression {
        fn from(exprs: [TblExpression;N]) -> Self
            { CompoundTblExpression::from(exprs).into() }
    }
    impl From<Box<[TblExpression]>> for TblExpression {
        fn from(exprs: Box<[TblExpression]>) -> Self
            { CompoundTblExpression::from(exprs).into() }
    }
    impl From<Arc<[TblExpression]>> for TblExpression {
        fn from(exprs: Arc<[TblExpression]>) -> Self
            { CompoundTblExpression::from(exprs).into() }
    }
    impl From<Vec<TblExpression>> for TblExpression {
        fn from(exprs: Vec<TblExpression>) -> Self
            { CompoundTblExpression::from(exprs).into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_atom_on_atom() {
        for i in 0..10 {
            let atomic_expr = TblExpression::from(AtomicTblExpression(i));
            assert_eq!(atomic_expr.as_atom(), Ok(AtomicTblExpression(i)));
        }
    }

    #[test]
    fn test_as_atom_on_tuple() {
        for i in 0..10 {
            let atomic_expr = TblExpression::from(vec![TblExpression::from(AtomicTblExpression(i))]);
            assert_eq!(atomic_expr.as_atom(), Err(()));
        }
    }

    #[test]
    fn test_as_tuple_on_atom() {
        for i in 0..10 {
            let atomic_expr = TblExpression::Atomic(AtomicTblExpression(i));
            assert_eq!(atomic_expr.as_vec(), Err(()));
        }
    }

    #[test]
    fn test_as_tuple_on_tuple() {
        for i in 0..10 {
            let atomic_expr = TblExpression::from(vec![TblExpression::from(AtomicTblExpression(i))]);
            assert_eq!(atomic_expr.as_vec(), Ok(&CompoundTblExpression::from(vec![TblExpression::from(AtomicTblExpression(i))])));
        }
    }

    #[test]
    fn test_as_slice_on_atom() {
        for i in 0..10 {
            let atomic_expr = TblExpression::Atomic(AtomicTblExpression(i));
            assert_eq!(atomic_expr.as_slice(), Err(()));
        }
    }

    #[test]
    fn test_as_slice_on_tuple() {
        for i in 0..10 {
            let atomic_expr = TblExpression::from(vec![TblExpression::from(AtomicTblExpression(i))]);
            assert_eq!(atomic_expr.as_slice(), Ok(vec![TblExpression::from(AtomicTblExpression(i))].as_slice()));
        }
    }
}
