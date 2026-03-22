use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::expressions::{Expression, atomic::AtomicExpression, compound::CompoundExpression};

pub enum ExpressionAtPathEnum<'a,Path> {
    Atomic(ObjAtPath<'a,AtomicExpression,Path>),
    Compound(ObjAtPath<'a,CompoundExpression,Path>)
}
impl <'a,Path> From<ObjAtPath<'a,Expression,Path>> for ExpressionAtPathEnum<'a,Path> {
    fn from(value: ObjAtPath<'a,Expression,Path>) -> Self { match value.obj {
        Expression::Atomic(atom) => Self::Atomic(ObjAtPath { obj: atom, path: value.path }),
        Expression::Compound(compound) => Self::Compound(ObjAtPath { obj: &compound, path: value.path }),
    }}
}
// impl <'a,Path> Into<OwnedObjAtPath<Expression,Path>> for ExpressionAtPathEnum<'a,Path> {
//     fn into(self) -> ObjAtPath<Expression, Path> { match self {
//         Self::Atomic(inner) => ObjAtPath { obj: Expression::Atomic(inner.obj), path: inner.path },
//         Self::Compound(inner) => ObjAtPath { obj: Expression::Compound(inner.obj), path: inner.path },
//     }}
// }


pub enum OwnedExpressionAtPathEnum<Path> {
    Atomic(OwnedObjAtPath<AtomicExpression,Path>),
    Compound(OwnedObjAtPath<CompoundExpression,Path>)
}
impl <Path> From<OwnedObjAtPath<Expression,Path>> for OwnedExpressionAtPathEnum<Path> {
    fn from(value: OwnedObjAtPath<Expression,Path>) -> Self { match value.obj {
        Expression::Atomic(atom) => Self::Atomic(OwnedObjAtPath { obj: atom, path: value.path }),
        Expression::Compound(compound) => Self::Compound(OwnedObjAtPath { obj: compound, path: value.path }),
    }}
}
impl <Path> Into<OwnedObjAtPath<Expression,Path>> for OwnedExpressionAtPathEnum<Path> {
    fn into(self) -> OwnedObjAtPath<Expression, Path> { match self {
        Self::Atomic(inner) => OwnedObjAtPath { obj: Expression::Atomic(inner.obj), path: inner.path },
        Self::Compound(inner) => OwnedObjAtPath { obj: Expression::Compound(inner.obj), path: inner.path },
    }}
}
