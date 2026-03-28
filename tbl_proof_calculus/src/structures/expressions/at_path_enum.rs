use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::structures::expressions::{TblExpression, atomic::AtomicTblExpression, compound::CompoundTblExpression};

pub enum ExpressionAtPathEnum<'a,C: CompoundTblExpression, Path> {
    Atomic(ObjAtPath<'a,AtomicTblExpression,Path>),
    Compound(ObjAtPath<'a,C,Path>)
}
impl <'a,C: CompoundTblExpression, Path> From<ObjAtPath<'a,TblExpression<C>,Path>> for ExpressionAtPathEnum<'a,C,Path> {
    fn from(value: ObjAtPath<'a,TblExpression<C>,Path>) -> Self { match value.obj {
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


pub enum OwnedExpressionAtPathEnum<C: CompoundTblExpression, Path> {
    Atomic(OwnedObjAtPath<AtomicTblExpression,Path>),
    Compound(OwnedObjAtPath<C,Path>)
}
impl <C: CompoundTblExpression, Path> From<OwnedObjAtPath<TblExpression<C>,Path>> for OwnedExpressionAtPathEnum<C,Path> {
    fn from(value: OwnedObjAtPath<TblExpression<C>,Path>) -> Self { match value.obj {
        TblExpression::Atomic(atom) => Self::Atomic(OwnedObjAtPath { obj: atom, path: value.path }),
        TblExpression::Compound(compound) => Self::Compound(OwnedObjAtPath { obj: compound, path: value.path }),
    }}
}
impl <C: CompoundTblExpression,Path> Into<OwnedObjAtPath<TblExpression<C>,Path>> for OwnedExpressionAtPathEnum<C,Path> {
    fn into(self) -> OwnedObjAtPath<TblExpression<C>, Path> { match self {
        Self::Atomic(inner) => OwnedObjAtPath { obj: TblExpression::Atomic(inner.obj), path: inner.path },
        Self::Compound(inner) => OwnedObjAtPath { obj: TblExpression::Compound(inner.obj), path: inner.path },
    }}
}
