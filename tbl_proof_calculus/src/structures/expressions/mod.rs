use proof_calculus::structures::Proposition;

use crate::structures::expressions::{atomic::AtomicTblExpression, compound::{CompoundTblExpression, arc::ArcCompoundTblExpression, r#box::BoxCompoundTblExpression, rc::RcCompoundTblExpression}};

pub mod atomic;
pub mod compound;
pub mod subexpressions;
pub mod at_path_enum;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum TblExpression<C: CompoundTblExpression> {
    Atomic(AtomicTblExpression),
    Compound(C)
}
impl <C: CompoundTblExpression> Proposition for TblExpression<C> {}

//type RefTblExpression<'a> = TblExpression<RefCompoundTblExpression<'a>>;
pub type BoxTblExpression = TblExpression<BoxCompoundTblExpression>;
pub type RcTblExpression = TblExpression<RcCompoundTblExpression>;
pub type ArcTblExpression = TblExpression<ArcCompoundTblExpression>;

impl <C: CompoundTblExpression> TblExpression<C> {
    pub fn replace(&self, to_replace: &TblExpression<C>, replace_with: &TblExpression<C>) -> TblExpression<C> {
        if self == to_replace { replace_with.clone() }
        else if let TblExpression::Compound(compound) = self
            { TblExpression::Compound(compound.replace(to_replace, replace_with)) }
        else { self.clone() }
    }
    
    // /// If this expression is an Atom, get its id. Otherwise throw an error
    // pub fn as_atom(&self) -> Result<AtomicTblExpression,()> {
    //     match self {
    //         TblExpression::Atomic(entity_id) => Ok(*entity_id),
    //         TblExpression::Compound(_) => Err(()),
    //     }
    // }

    // /// If this expression is a Tuple, get its expressions. Otherwise throw an error 
    // pub fn as_vec<'a>(&'a self) -> Result<&'a C,()> { 
    //     match self {
    //         TblExpression::Atomic(_) => Err(()),
    //         TblExpression::Compound(proposition_exprs) => Ok(proposition_exprs),
    //     }
    // }

    // /// If this expression is a Tuple, get its subexpressions. Otherwise throw an error 
    // pub fn as_slice(&self) -> Result<&[TblExpression], ()> {
    //     match self {
    //         TblExpression::Atomic(_) => Err(()),
    //         TblExpression::Compound(proposition_exprs) => Ok(&proposition_exprs.0),
    //     }
    // }

    pub fn len(&self) -> Option<usize> {
        match self {
            TblExpression::Atomic(_) => None,
            TblExpression::Compound(exprs) => Some(exprs.len())
        }
    }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::structures::expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression};

    impl <C: CompoundTblExpression> From<AtomicTblExpression> for TblExpression<C> {
        fn from(id: AtomicTblExpression) -> Self
            { Self::Atomic(id) }
    }
    impl <C: CompoundTblExpression> From<u16> for TblExpression<C> {
        fn from(id: u16) -> Self
            { AtomicTblExpression(id).into() }
    }
    impl <C: CompoundTblExpression> From<C> for TblExpression<C> {
        fn from(expr: C) -> Self
            { Self::Compound(expr) }
    }
    impl <const N: usize, C: CompoundTblExpression> From<[TblExpression<C>;N]> for TblExpression<C> where C: From<[TblExpression<C>;N]> {
        fn from(exprs: [TblExpression<C>;N]) -> Self
            { C::from(exprs).into() }
    }
    impl <C: CompoundTblExpression> From<Box<[TblExpression<C>]>> for TblExpression<C> where C: From<Box<[TblExpression<C>]>> {
        fn from(exprs: Box<[TblExpression<C>]>) -> Self
            { C::from(exprs).into() }
    }
    impl <C: CompoundTblExpression> From<Rc<[TblExpression<C>]>> for TblExpression<C> where C: From<Rc<[TblExpression<C>]>> {
        fn from(exprs: Rc<[TblExpression<C>]>) -> Self
            { C::from(exprs).into() }
    }
    impl <C: CompoundTblExpression> From<Arc<[TblExpression<C>]>> for TblExpression<C> where C: From<Arc<[TblExpression<C>]>> {
        fn from(exprs: Arc<[TblExpression<C>]>) -> Self
            { C::from(exprs).into() }
    }
    impl <C: CompoundTblExpression> From<Vec<TblExpression<C>>> for TblExpression<C> where C: From<Vec<TblExpression<C>>> {
        fn from(exprs: Vec<TblExpression<C>>) -> Self
            { C::from(exprs).into() }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_as_atom_on_atom() {
//         for i in 0..10 {
//             let atomic_expr = TblExpression::from(AtomicTblExpression(i));
//             assert_eq!(atomic_expr.as_atom(), Ok(AtomicTblExpression(i)));
//         }
//     }

//     #[test]
//     fn test_as_atom_on_tuple() {
//         for i in 0..10 {
//             let atomic_expr = TblExpression::from(vec![TblExpression::from(AtomicTblExpression(i))]);
//             assert_eq!(atomic_expr.as_atom(), Err(()));
//         }
//     }

//     #[test]
//     fn test_as_tuple_on_atom() {
//         for i in 0..10 {
//             let atomic_expr = TblExpression::Atomic(AtomicTblExpression(i));
//             assert_eq!(atomic_expr.as_vec(), Err(()));
//         }
//     }

//     #[test]
//     fn test_as_tuple_on_tuple() {
//         for i in 0..10 {
//             let atomic_expr = TblExpression::from(vec![TblExpression::from(AtomicTblExpression(i))]);
//             assert_eq!(atomic_expr.as_vec(), Ok(&CompoundTblExpression::from(vec![TblExpression::from(AtomicTblExpression(i))])));
//         }
//     }

//     #[test]
//     fn test_as_slice_on_atom() {
//         for i in 0..10 {
//             let atomic_expr = TblExpression::Atomic(AtomicTblExpression(i));
//             assert_eq!(atomic_expr.as_slice(), Err(()));
//         }
//     }

//     #[test]
//     fn test_as_slice_on_tuple() {
//         for i in 0..10 {
//             let atomic_expr = TblExpression::from(vec![TblExpression::from(AtomicTblExpression(i))]);
//             assert_eq!(atomic_expr.as_slice(), Ok(vec![TblExpression::from(AtomicTblExpression(i))].as_slice()));
//         }
//     }
// }
