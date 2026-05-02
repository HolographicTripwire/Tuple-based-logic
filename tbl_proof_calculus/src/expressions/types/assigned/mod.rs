use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::expressions::{paths::{TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath}, types::assigned::{atomic::AtomicTblExpression, compound::{CompoundTblExpression, arc::ArcCompoundTblExpression, r#box::BoxCompoundTblExpression, rc::RcCompoundTblExpression}, subexpressions::{ParentOfImmediateSubexpressions, ParentOfSubexpressions, TblSubexpressionInExpression, iterators::depth_first::counterclockwise::{CounterclockwiseDepthFirstLocatedTblSubexpressionIterator, CounterclockwiseDepthFirstTblSubexpressionIterator}}}};

pub mod atomic;
pub mod compound;
pub mod subexpressions;
pub mod at_path_enum;
pub mod binding;
pub mod collections;


#[derive(Debug,Clone,Eq,Hash)]
pub enum TblExpression<C: CompoundTblExpression> {
    Atomic(AtomicTblExpression),
    Compound(C)
}
pub type TblExpressionAtPath<'a,C: CompoundTblExpression, Path> = ObjAtPath<'a,TblExpression<C>,Path>;
pub type OwnedTblExpressionAtPath<C: CompoundTblExpression, Path> = OwnedObjAtPath<TblExpression<C>,Path>;

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
    pub fn replace(&self, to_replace: &TblExpression<C>, replace_with: &TblExpression<C>) -> Self {
        if self == to_replace { replace_with.clone() }
        else if let TblExpression::Compound(compound) = self
            { TblExpression::Compound(compound.replace(to_replace, replace_with)) }
        else { self.clone() }
    }

    pub fn is_atom(&self) -> bool { if let TblExpression::Atomic(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let TblExpression::Compound(_) = self { true } else { false } }

    pub fn get_subexpressions_helper(&self,path: &TblSubexpressionInExpressionPath, index: usize) -> Result<&TblExpression<C>,()> {
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
    pub fn as_slice(&self) -> Result<&[TblExpression<C>], ()> {
        match self {
            TblExpression::Atomic(_) => Err(()),
            TblExpression::Compound(proposition_exprs) => Ok(proposition_exprs.as_slice()),
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            TblExpression::Atomic(_) => None,
            TblExpression::Compound(exprs) => Some(exprs.len())
        }
    }

    pub fn transmute_compound<'a,C2: CompoundTblExpression + From<&'a C>>(&'a self) -> TblExpression<C2> { match self {
        TblExpression::Atomic(atomic) => TblExpression::Atomic(*atomic),
        TblExpression::Compound(compound) => TblExpression::Compound(compound.into()),
    }}
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
    fn get_immediate_subexpression_paths(&self) -> impl IntoIterator<Item = ImmediateTblSubexpressionInExpressionPath> { match self {
        TblExpression::Atomic(_) => Box::from_iter([]),
        TblExpression::Compound(compound) => compound.get_immediate_subexpression_paths().into_iter().collect(),
    }}

    fn get_immediate_subexpression(&self,path: &ImmediateTblSubexpressionInExpressionPath) -> Result< &TblExpression<C> ,()>  { match self {
        TblExpression::Atomic(_) => Err(()),
        TblExpression::Compound(c) => c.get_immediate_subexpression(path),
    }}
}
impl <C:CompoundTblExpression> ParentOfSubexpressions<C> for TblExpression<C> {
    fn get_subexpression_paths(&self) -> impl IntoIterator<Item = TblSubexpressionInExpressionPath>
        { self.get_located_subexpressions().into_iter().map(|expr| expr.path) }

    fn get_subexpression(&self,path: &TblSubexpressionInExpressionPath) -> Result< &TblExpression<C> ,()>
        { self.get_subexpressions_helper(path, 0) }
    
    fn get_subexpressions<'a>(&'a self) -> impl IntoIterator<Item =  &'a TblExpression<C> >where TblExpression<C> :'a
        { CounterclockwiseDepthFirstTblSubexpressionIterator::new(self) }
    fn get_located_subexpressions<'a>(&'a self) -> impl IntoIterator<Item = TblSubexpressionInExpression<'a,C>> where TblExpression<C> :'a
        { CounterclockwiseDepthFirstLocatedTblSubexpressionIterator::new(self) }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::expressions::types::assigned::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression};

    impl <C: CompoundTblExpression> From<AtomicTblExpression> for TblExpression<C> 
        { fn from(id: AtomicTblExpression) -> Self { Self::Atomic(id) } }
    impl <C: CompoundTblExpression> From<u16> for TblExpression<C> 
        { fn from(id: u16) -> Self { AtomicTblExpression(id).into() } }
    impl <C: CompoundTblExpression> From<C> for TblExpression<C> 
        { fn from(expr: C) -> Self { Self::Compound(expr) } }
    
    impl <const N: usize, C: CompoundTblExpression> From<[Self;N]> for TblExpression<C> where C: From<[Self;N]> 
        { fn from(exprs: [Self;N]) -> Self { C::from(exprs).into() } }
    impl <C: CompoundTblExpression> From<Box<[Self]>> for TblExpression<C> where C: From<Box<[Self]>> 
        { fn from(exprs: Box<[Self]>) -> Self { C::from(exprs).into() } }
    impl <C: CompoundTblExpression> From<Rc<[Self]>> for TblExpression<C> where C: From<Rc<[Self]>> 
        { fn from(exprs: Rc<[Self]>) -> Self { C::from(exprs).into() } }
    impl <C: CompoundTblExpression> From<Arc<[Self]>> for TblExpression<C> where C: From<Arc<[Self]>> 
        { fn from(exprs: Arc<[Self]>) -> Self { C::from(exprs).into() } }
    impl <C: CompoundTblExpression> From<Vec<Self>> for TblExpression<C> where C: From<Vec<Self>> 
        { fn from(exprs: Vec<Self>) -> Self { C::from(exprs).into() } }

    impl <C: CompoundTblExpression> FromIterator<Self> for TblExpression<C>
        { fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self { Self::Compound(C::from_iter(iter)) } }
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
