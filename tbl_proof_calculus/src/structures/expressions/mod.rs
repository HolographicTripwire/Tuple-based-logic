use proof_calculus::structures::propositions::Proposition;

use crate::structures::expressions::{atomic::AtomicTblExpression, compound::{CompoundTblExpression, arc::ArcCompoundTblExpression, r#box::BoxCompoundTblExpression, rc::RcCompoundTblExpression}, subexpressions::{ParentOfSubexpressions, SubexpressionInExpressionPath, immediate::{ImmediateSubexpressionInExpressionPath, ParentOfImmediateSubexpressions}}};

pub mod atomic;
pub mod compound;
pub mod located;
pub mod subexpressions;
pub mod at_path_enum;

#[derive(Debug,Clone,Eq,Hash)]
pub enum TblExpression<C: CompoundTblExpression> {
    Atomic(AtomicTblExpression),
    Compound(C)
}
impl <C: CompoundTblExpression> Proposition for TblExpression<C> {}
impl <C1: CompoundTblExpression, C2: CompoundTblExpression + PartialEq<C1>> PartialEq<TblExpression<C1>> for TblExpression<C2> {
    fn eq(&self, other: &TblExpression<C1>) -> bool { match (self,other) {
        (TblExpression::Atomic(atom_left), TblExpression::Atomic(atom_right)) => atom_left == atom_right,
        (TblExpression::Compound(compound_left), TblExpression::Compound(compound_right)) => compound_left == compound_right,
        _ => false
    }}
}

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

    pub fn is_atom(&self) -> bool { if let TblExpression::Atomic(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let TblExpression::Compound(_) = self { true } else { false } }

    pub fn get_subexpressions_helper(&self,path: &SubexpressionInExpressionPath, index: usize) -> Result<&TblExpression<C>,()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subexpression(immediate_path)?;
        if index == path.0.len() { Ok(inner) }
        else { inner.get_subexpressions_helper(path, index+1) }
    }

    // /// If this expression is a Tuple, get its expressions. Otherwise throw an error 
    // pub fn as_vec<'a>(&'a self) -> Result<&'a C,()> { 
    //     match self {
    //         TblExpression::Atomic(_) => Err(()),
    //         TblExpression::Compound(proposition_exprs) => Ok(proposition_exprs),
    //     }
    // }

    /// If this expression is a Tuple, get its subexpressions. Otherwise throw an error 
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
impl <C: CompoundTblExpression> TryInto<AtomicTblExpression> for TblExpression<C> {
    type Error = ();
    fn try_into(self) -> Result<AtomicTblExpression, Self::Error> { match self {
        TblExpression::Atomic(atom) => Ok(atom),
        TblExpression::Compound(_) => Err(()),
    }}
}
// impl <C: CompoundTblExpression> TryInto<C> for TblExpression<C> {
//     type Error = ();
//     fn try_into(self) -> Result<C, Self::Error> { match self {
//         TblExpression::Atomic(_) => Err(()),
//         TblExpression::Compound(compound) => Ok(compound),
//     }}
// }

impl <C:CompoundTblExpression> ParentOfImmediateSubexpressions<C> for TblExpression<C> {
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateSubexpressionInExpressionPath> { match self {
        TblExpression::Atomic(_) => Box::from_iter([]),
        TblExpression::Compound(compound) => compound.get_immediate_subexpression_paths().into_iter().collect(),
    }}

    fn get_immediate_subexpression(&self,path: &ImmediateSubexpressionInExpressionPath) -> Result< &TblExpression<C> ,()>  { match self {
        TblExpression::Atomic(_) => Err(()),
        TblExpression::Compound(c) => c.get_immediate_subexpression(path),
    }}
}
impl <C:CompoundTblExpression> ParentOfSubexpressions<C> for TblExpression<C> {
    fn get_subexpression_paths(&self) -> impl IntoIterator<Item = SubexpressionInExpressionPath> {
        let immediate = self.get_immediate_subexpression_paths()
            .into_iter()
            .map(|x| x.into());
        let deferred = self.get_located_immediate_subexpressions()
            .into_iter()
            .map(|inner| inner.obj.get_subexpression_paths()
                .into_iter()
                .map(|p| (inner.path,p).into())
                .collect::<Vec<_>>()
            ).flatten();
        immediate.chain(deferred)
    }

    fn get_subexpression(&self,path: &SubexpressionInExpressionPath) -> Result< &TblExpression<C> ,()>
        { self.get_subexpressions_helper(path, 0) }
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
