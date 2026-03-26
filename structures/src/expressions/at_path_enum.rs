use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression};

pub enum ExpressionAtPathEnum<'a,Path> {
    Atomic(ObjAtPath<'a,AtomicTblExpression,Path>),
    Compound(ObjAtPath<'a,CompoundTblExpression,Path>)
}
impl <'a,Path> From<ObjAtPath<'a,TblExpression,Path>> for ExpressionAtPathEnum<'a,Path> {
    fn from(value: ObjAtPath<'a,TblExpression,Path>) -> Self { match value.obj {
        TblExpression::Atomic(atom) => Self::Atomic(ObjAtPath { obj: atom, path: value.path }),
        TblExpression::Compound(compound) => Self::Compound(ObjAtPath { obj: &compound, path: value.path }),
    }}
}
// impl <'a,Path> Into<OwnedObjAtPath<Expression,Path>> for ExpressionAtPathEnum<'a,Path> {
//     fn into(self) -> ObjAtPath<Expression, Path> { match self {
//         Self::Atomic(inner) => ObjAtPath { obj: Expression::Atomic(inner.obj), path: inner.path },
//         Self::Compound(inner) => ObjAtPath { obj: Expression::Compound(inner.obj), path: inner.path },
//     }}
// }


pub enum OwnedExpressionAtPathEnum<Path> {
    Atomic(OwnedObjAtPath<AtomicTblExpression,Path>),
    Compound(OwnedObjAtPath<CompoundTblExpression,Path>)
}
impl <Path> From<OwnedObjAtPath<TblExpression,Path>> for OwnedExpressionAtPathEnum<Path> {
    fn from(value: OwnedObjAtPath<TblExpression,Path>) -> Self { match value.obj {
        TblExpression::Atomic(atom) => Self::Atomic(OwnedObjAtPath { obj: atom, path: value.path }),
        TblExpression::Compound(compound) => Self::Compound(OwnedObjAtPath { obj: compound, path: value.path }),
    }}
}
impl <Path> Into<OwnedObjAtPath<TblExpression,Path>> for OwnedExpressionAtPathEnum<Path> {
    fn into(self) -> OwnedObjAtPath<TblExpression, Path> { match self {
        Self::Atomic(inner) => OwnedObjAtPath { obj: TblExpression::Atomic(inner.obj), path: inner.path },
        Self::Compound(inner) => OwnedObjAtPath { obj: TblExpression::Compound(inner.obj), path: inner.path },
    }}
}
