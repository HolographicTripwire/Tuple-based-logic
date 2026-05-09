use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::expressions::{
    TblExpressionLength,
    paths::{
        TblSubexpressionInExpressionPath, immediate::ImmediateTblSubexpressionInExpressionPath,
    },
    types::assigned::{
        atom::TblExpressionAtom,
        compound::{
            TblExpressionCompound, arc::ArcTblExpressionCompound, r#box::BoxTblExpressionCompound,
            rc::RcTblExpressionCompound,
        },
        subexpressions::{
            ParentOfImmediateSubexpressions, ParentOfSubexpressions, TblSubexpressionInExpression,
            iterators::depth_first::counterclockwise::{
                CounterclockwiseDepthFirstLocatedTblSubexpressionIterator,
                CounterclockwiseDepthFirstTblSubexpressionIterator,
            },
        },
    },
};

pub mod at_path_enum;
pub mod atom;
pub mod binding;
pub mod collections;
pub mod compound;
pub mod subexpressions;

#[derive(Debug, Clone, Eq, Hash)]
pub enum TblExpression<C: TblExpressionCompound> {
    Atom(TblExpressionAtom),
    Compound(C),
}
pub type TblExpressionAtPath<'a, C: TblExpressionCompound, Path> =
    ObjAtPath<'a, TblExpression<C>, Path>;
pub type OwnedTblExpressionAtPath<C: TblExpressionCompound, Path> =
    OwnedObjAtPath<TblExpression<C>, Path>;

impl<C1: TblExpressionCompound, C2: TblExpressionCompound + PartialEq<C1>>
    PartialEq<TblExpression<C1>> for TblExpression<C2>
{
    fn eq(&self, other: &TblExpression<C1>) -> bool {
        match (self, other) {
            (TblExpression::Atom(atom_left), TblExpression::Atom(atom_right)) => {
                atom_left == atom_right
            }
            (TblExpression::Compound(compound_left), TblExpression::Compound(compound_right)) => {
                compound_left == compound_right
            }
            _ => false,
        }
    }
}

//type RefTblExpression<'a> = TblExpression<RefCompoundTblExpression<'a>>;
pub type BoxTblExpression = TblExpression<BoxTblExpressionCompound>;
pub type RcTblExpression = TblExpression<RcTblExpressionCompound>;
pub type ArcTblExpression = TblExpression<ArcTblExpressionCompound>;

impl<C: TblExpressionCompound> TblExpression<C> {
    pub fn replace(&self, to_replace: &TblExpression<C>, replace_with: &TblExpression<C>) -> Self {
        if self == to_replace {
            replace_with.clone()
        } else if let TblExpression::Compound(compound) = self {
            TblExpression::Compound(compound.replace(to_replace, replace_with))
        } else {
            self.clone()
        }
    }

    pub fn is_atom(&self) -> bool {
        if let TblExpression::Atom(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_compound(&self) -> bool {
        if let TblExpression::Compound(_) = self {
            true
        } else {
            false
        }
    }

    pub fn get_subexpressions_helper(
        &self,
        path: &TblSubexpressionInExpressionPath,
        index: usize,
    ) -> Result<&TblExpression<C>, ()> {
        let immediate_path = path.0.get(index).ok_or(())?;
        let inner = self.get_immediate_subexpression(immediate_path)?;
        if index == path.0.len() {
            Ok(inner)
        } else {
            inner.get_subexpressions_helper(path, index + 1)
        }
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
            TblExpression::Atom(_) => Err(()),
            TblExpression::Compound(proposition_exprs) => Ok(proposition_exprs.as_slice()),
        }
    }

    pub fn len(&self) -> TblExpressionLength {
        match self {
            TblExpression::Atom(_) => TblExpressionLength::Unit,
            TblExpression::Compound(exprs) => TblExpressionLength::Compound(exprs.len()),
        }
    }
}
impl<C: TblExpressionCompound> TryInto<TblExpressionAtom> for TblExpression<C> {
    type Error = ();
    fn try_into(self) -> Result<TblExpressionAtom, Self::Error> {
        match self {
            TblExpression::Atom(atom) => Ok(atom),
            TblExpression::Compound(_) => Err(()),
        }
    }
}
// impl <C: CompoundTblExpression> TryInto<C> for TblExpression<C> {
//     type Error = ();
//     fn try_into(self) -> Result<C, Self::Error> { match self {
//         TblExpression::Atomic(_) => Err(()),
//         TblExpression::Compound(compound) => Ok(compound),
//     }}
// }

impl<C: TblExpressionCompound> ParentOfImmediateSubexpressions<C> for TblExpression<C> {
    fn get_immediate_subexpression_paths(
        &self,
    ) -> impl IntoIterator<Item = ImmediateTblSubexpressionInExpressionPath> {
        match self {
            TblExpression::Atom(_) => Box::from_iter([]),
            TblExpression::Compound(compound) => compound
                .get_immediate_subexpression_paths()
                .into_iter()
                .collect(),
        }
    }

    fn get_immediate_subexpression(
        &self,
        path: &ImmediateTblSubexpressionInExpressionPath,
    ) -> Result<&TblExpression<C>, ()> {
        match self {
            TblExpression::Atom(_) => Err(()),
            TblExpression::Compound(c) => c.get_immediate_subexpression(path),
        }
    }
}
impl<C: TblExpressionCompound> ParentOfSubexpressions<C> for TblExpression<C> {
    fn get_subexpression_paths(
        &self,
    ) -> impl IntoIterator<Item = TblSubexpressionInExpressionPath> {
        self.get_located_subexpressions()
            .into_iter()
            .map(|expr| expr.path)
    }
    fn get_subexpression(
        &self,
        path: &TblSubexpressionInExpressionPath,
    ) -> Result<&TblExpression<C>, ()> {
        self.get_subexpressions_helper(path, 0)
    }

    fn get_subexpressions<'a>(&'a self) -> impl IntoIterator<Item = &'a TblExpression<C>>
    where
        TblExpression<C>: 'a,
    {
        CounterclockwiseDepthFirstTblSubexpressionIterator::new(self)
    }
    fn get_located_subexpressions<'a>(
        &'a self,
    ) -> impl IntoIterator<Item = TblSubexpressionInExpression<'a, C>>
    where
        TblExpression<C>: 'a,
    {
        CounterclockwiseDepthFirstLocatedTblSubexpressionIterator::new(self)
    }
}

mod from {
    use std::{rc::Rc, sync::Arc};

    use crate::expressions::types::assigned::{
        TblExpression, atom::TblExpressionAtom, compound::TblExpressionCompound,
    };

    impl<C: TblExpressionCompound> From<TblExpressionAtom> for TblExpression<C> {
        fn from(id: TblExpressionAtom) -> Self {
            Self::Atom(id)
        }
    }
    impl<C: TblExpressionCompound> From<u16> for TblExpression<C> {
        fn from(id: u16) -> Self {
            TblExpressionAtom(id).into()
        }
    }
    impl<C: TblExpressionCompound> From<C> for TblExpression<C> {
        fn from(expr: C) -> Self {
            Self::Compound(expr)
        }
    }
    impl<C1: TblExpressionCompound, C2: TblExpressionCompound + for<'a> From<&'a C1>>
        From<&TblExpression<C1>> for TblExpression<C2>
    {
        fn from(value: &TblExpression<C1>) -> Self {
            match value {
                TblExpression::Atom(atomic) => TblExpression::Atom(*atomic),
                TblExpression::Compound(compound) => TblExpression::Compound(compound.into()),
            }
        }
    }

    impl<const N: usize, C: TblExpressionCompound> From<[Self; N]> for TblExpression<C>
    where
        C: From<[Self; N]>,
    {
        fn from(exprs: [Self; N]) -> Self {
            C::from(exprs).into()
        }
    }
    impl<C: TblExpressionCompound> From<Box<[Self]>> for TblExpression<C>
    where
        C: From<Box<[Self]>>,
    {
        fn from(exprs: Box<[Self]>) -> Self {
            C::from(exprs).into()
        }
    }
    impl<C: TblExpressionCompound> From<Rc<[Self]>> for TblExpression<C>
    where
        C: From<Rc<[Self]>>,
    {
        fn from(exprs: Rc<[Self]>) -> Self {
            C::from(exprs).into()
        }
    }
    impl<C: TblExpressionCompound> From<Arc<[Self]>> for TblExpression<C>
    where
        C: From<Arc<[Self]>>,
    {
        fn from(exprs: Arc<[Self]>) -> Self {
            C::from(exprs).into()
        }
    }
    impl<C: TblExpressionCompound> From<Vec<Self>> for TblExpression<C>
    where
        C: From<Vec<Self>>,
    {
        fn from(exprs: Vec<Self>) -> Self {
            C::from(exprs).into()
        }
    }

    impl<C: TblExpressionCompound + FromIterator<Self>> FromIterator<Self> for TblExpression<C> {
        fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
            Self::Compound(C::from_iter(iter))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AtomOrCompoundLength {
    Atom(TblExpressionAtom),
    CompoundLength(usize),
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
