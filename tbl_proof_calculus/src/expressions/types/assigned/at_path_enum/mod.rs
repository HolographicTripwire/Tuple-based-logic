use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::expressions::types::assigned::{
    OwnedTblExpressionAtPath, TblExpression,
    atom::{OwnedTblExpressionAtomAtPath, TblExpressionAtomAtPath},
    compound::{
        OwnedTblExpressionCompoundAtPath, TblExpressionCompound, TblExpressionCompoundAtPath,
    },
};

mod with_path_specified;
pub use with_path_specified::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum TblExpressionAtPathEnum<'a, C: TblExpressionCompound, Path> {
    Atom(TblExpressionAtomAtPath<'a, Path>),
    Compound(TblExpressionCompoundAtPath<'a, C, Path>),
}

impl<'a, C: TblExpressionCompound, Path> TblExpressionAtPathEnum<'a, C, Path> {
    pub fn is_atom(&self) -> bool {
        if let TblExpressionAtPathEnum::Atom(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_compound(&self) -> bool {
        if let TblExpressionAtPathEnum::Atom(_) = self {
            true
        } else {
            false
        }
    }
}
impl<'a, C: TblExpressionCompound, Path> TryInto<TblExpressionAtomAtPath<'a, Path>>
    for TblExpressionAtPathEnum<'a, C, Path>
{
    type Error = ();
    fn try_into(self) -> Result<TblExpressionAtomAtPath<'a, Path>, Self::Error> {
        match self {
            TblExpressionAtPathEnum::Atom(atom) => Ok(atom),
            TblExpressionAtPathEnum::Compound(_) => Err(()),
        }
    }
}
impl<'a, C: TblExpressionCompound, Path> TryInto<TblExpressionCompoundAtPath<'a, C, Path>>
    for TblExpressionAtPathEnum<'a, C, Path>
{
    type Error = ();
    fn try_into(self) -> Result<TblExpressionCompoundAtPath<'a, C, Path>, Self::Error> {
        match self {
            TblExpressionAtPathEnum::Atom(_) => Err(()),
            TblExpressionAtPathEnum::Compound(compound) => Ok(compound),
        }
    }
}

impl<'a, C: TblExpressionCompound, Path> From<ObjAtPath<'a, TblExpression<C>, Path>>
    for TblExpressionAtPathEnum<'a, C, Path>
{
    fn from(value: ObjAtPath<'a, TblExpression<C>, Path>) -> Self {
        match value.obj {
            TblExpression::Atom(atom) => Self::Atom(ObjAtPath {
                obj: atom,
                path: value.path,
            }),
            TblExpression::Compound(compound) => Self::Compound(ObjAtPath {
                obj: compound,
                path: value.path,
            }),
        }
    }
}
// impl <'a,Path> Into<OwnedObjAtPath<Expression,Path>> for ExpressionAtPathEnum<'a,Path> {
//     fn into(self) -> ObjAtPath<Expression, Path> { match self {
//         Self::Atomic(inner) => ObjAtPath { obj: Expression::Atomic(inner.obj), path: inner.path },
//         Self::Compound(inner) => ObjAtPath { obj: Expression::Compound(inner.obj), path: inner.path },
//     }}
// }

pub enum OwnedTblExpressionAtPathEnum<C: TblExpressionCompound, Path> {
    Atomic(OwnedTblExpressionAtomAtPath<Path>),
    Compound(OwnedTblExpressionCompoundAtPath<C, Path>),
}
impl<C: TblExpressionCompound, Path> OwnedTblExpressionAtPathEnum<C, Path> {
    pub fn is_atom(&self) -> bool {
        if let OwnedTblExpressionAtPathEnum::Atomic(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_compound(&self) -> bool {
        if let OwnedTblExpressionAtPathEnum::Atomic(_) = self {
            true
        } else {
            false
        }
    }
}
impl<C: TblExpressionCompound, Path> TryInto<OwnedTblExpressionAtomAtPath<Path>>
    for OwnedTblExpressionAtPathEnum<C, Path>
{
    type Error = ();
    fn try_into(self) -> Result<OwnedTblExpressionAtomAtPath<Path>, Self::Error> {
        match self {
            OwnedTblExpressionAtPathEnum::Atomic(atom) => Ok(atom),
            OwnedTblExpressionAtPathEnum::Compound(_) => Err(()),
        }
    }
}
impl<C: TblExpressionCompound, Path> TryInto<OwnedTblExpressionCompoundAtPath<C, Path>>
    for OwnedTblExpressionAtPathEnum<C, Path>
{
    type Error = ();
    fn try_into(self) -> Result<OwnedTblExpressionCompoundAtPath<C, Path>, Self::Error> {
        match self {
            OwnedTblExpressionAtPathEnum::Atomic(_) => Err(()),
            OwnedTblExpressionAtPathEnum::Compound(compound) => Ok(compound),
        }
    }
}

impl<C: TblExpressionCompound, Path> From<OwnedTblExpressionAtPath<C, Path>>
    for OwnedTblExpressionAtPathEnum<C, Path>
{
    fn from(value: OwnedObjAtPath<TblExpression<C>, Path>) -> Self {
        match value.obj {
            TblExpression::Atom(atom) => Self::Atomic(OwnedObjAtPath {
                obj: atom,
                path: value.path,
            }),
            TblExpression::Compound(compound) => Self::Compound(OwnedObjAtPath {
                obj: compound,
                path: value.path,
            }),
        }
    }
}
impl<C: TblExpressionCompound, Path> Into<OwnedTblExpressionAtPath<C, Path>>
    for OwnedTblExpressionAtPathEnum<C, Path>
{
    fn into(self) -> OwnedObjAtPath<TblExpression<C>, Path> {
        match self {
            Self::Atomic(inner) => OwnedObjAtPath {
                obj: TblExpression::Atom(inner.obj),
                path: inner.path,
            },
            Self::Compound(inner) => OwnedObjAtPath {
                obj: TblExpression::Compound(inner.obj),
                path: inner.path,
            },
        }
    }
}
