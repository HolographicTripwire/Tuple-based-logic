use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::expressions::assigned::{OwnedTblExpressionAtPath, TblExpression, atomic::{AtomicTblExpressionAtPath, OwnedAtomicTblExpressionAtPath}, compound::{CompoundTblExpression, CompoundTblExpressionAtPath, OwnedCompoundTblExpressionAtPath}};

mod with_path_specified;
pub use with_path_specified::*;

pub enum TblExpressionAtPathEnum<'a,C: CompoundTblExpression, Path> {
    Atomic(AtomicTblExpressionAtPath<'a,Path>),
    Compound(CompoundTblExpressionAtPath<'a,C,Path>)
}

impl <'a,C:CompoundTblExpression,Path> TblExpressionAtPathEnum<'a,C,Path> {
    pub fn is_atom(&self) -> bool { if let TblExpressionAtPathEnum::Atomic(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let TblExpressionAtPathEnum::Atomic(_) = self { true } else { false } }
}
impl <'a,C:CompoundTblExpression,Path> TryInto<AtomicTblExpressionAtPath<'a,Path>> for TblExpressionAtPathEnum<'a,C,Path> {
    type Error = ();
    fn try_into(self) -> Result<AtomicTblExpressionAtPath<'a,Path>, Self::Error> { match self {
        TblExpressionAtPathEnum::Atomic(atom) => Ok(atom),
        TblExpressionAtPathEnum::Compound(_) => Err(())
    }}
}
impl <'a,C:CompoundTblExpression,Path> TryInto<CompoundTblExpressionAtPath<'a,C,Path>> for TblExpressionAtPathEnum<'a,C,Path> {
    type Error = ();
    fn try_into(self) -> Result<CompoundTblExpressionAtPath<'a,C,Path>, Self::Error> { match self {
        TblExpressionAtPathEnum::Atomic(_) => Err(()),
        TblExpressionAtPathEnum::Compound(compound) => Ok(compound)
    }}
}

impl <'a,C: CompoundTblExpression, Path> From<ObjAtPath<'a,TblExpression<C>,Path>> for TblExpressionAtPathEnum<'a,C,Path> {
    fn from(value: ObjAtPath<'a,TblExpression<C>,Path>) -> Self { match value.obj {
        TblExpression::Atomic(atom) => Self::Atomic(ObjAtPath { obj: atom, path: value.path }),
        TblExpression::Compound(compound) => Self::Compound(ObjAtPath { obj: compound, path: value.path }),
    }}
}
// impl <'a,Path> Into<OwnedObjAtPath<Expression,Path>> for ExpressionAtPathEnum<'a,Path> {
//     fn into(self) -> ObjAtPath<Expression, Path> { match self {
//         Self::Atomic(inner) => ObjAtPath { obj: Expression::Atomic(inner.obj), path: inner.path },
//         Self::Compound(inner) => ObjAtPath { obj: Expression::Compound(inner.obj), path: inner.path },
//     }}
// }


pub enum OwnedTblExpressionAtPathEnum<C: CompoundTblExpression, Path> {
    Atomic(OwnedAtomicTblExpressionAtPath<Path>),
    Compound(OwnedCompoundTblExpressionAtPath<C,Path>)
}
impl <C:CompoundTblExpression,Path> OwnedTblExpressionAtPathEnum<C,Path> {
    pub fn is_atom(&self) -> bool { if let OwnedTblExpressionAtPathEnum::Atomic(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let OwnedTblExpressionAtPathEnum::Atomic(_) = self { true } else { false } }
}
impl <C:CompoundTblExpression,Path> TryInto<OwnedAtomicTblExpressionAtPath<Path>> for OwnedTblExpressionAtPathEnum<C,Path> {
    type Error = ();
    fn try_into(self) -> Result<OwnedAtomicTblExpressionAtPath<Path>, Self::Error> { match self {
        OwnedTblExpressionAtPathEnum::Atomic(atom) => Ok(atom),
        OwnedTblExpressionAtPathEnum::Compound(_) => Err(())
    }}
}
impl <C:CompoundTblExpression,Path> TryInto<OwnedCompoundTblExpressionAtPath<C,Path>> for OwnedTblExpressionAtPathEnum<C,Path> {
    type Error = ();
    fn try_into(self) -> Result<OwnedCompoundTblExpressionAtPath<C,Path>, Self::Error> { match self {
        OwnedTblExpressionAtPathEnum::Atomic(_) => Err(()),
        OwnedTblExpressionAtPathEnum::Compound(compound) => Ok(compound)
    }}
}

impl <C: CompoundTblExpression, Path> From<OwnedTblExpressionAtPath<C,Path>> for OwnedTblExpressionAtPathEnum<C,Path> {
    fn from(value: OwnedObjAtPath<TblExpression<C>,Path>) -> Self { match value.obj {
        TblExpression::Atomic(atom) => Self::Atomic(OwnedObjAtPath { obj: atom, path: value.path }),
        TblExpression::Compound(compound) => Self::Compound(OwnedObjAtPath { obj: compound, path: value.path }),
    }}
}
impl <C: CompoundTblExpression,Path> Into<OwnedTblExpressionAtPath<C,Path>> for OwnedTblExpressionAtPathEnum<C,Path> {
    fn into(self) -> OwnedObjAtPath<TblExpression<C>, Path> { match self {
        Self::Atomic(inner) => OwnedObjAtPath { obj: TblExpression::Atomic(inner.obj), path: inner.path },
        Self::Compound(inner) => OwnedObjAtPath { obj: TblExpression::Compound(inner.obj), path: inner.path },
    }}
}
