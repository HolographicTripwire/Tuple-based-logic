use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::types::unassigned::{OwnedUnassignedTblExpressionAtPath, UnassignedTblExpression, UnassignedTblExpressionAtPath, compound::{OwnedUnassignedTblExpressionCompoundAtPath, UnassignedTblExpressionCompound, UnassignedTblExpressionCompoundAtPath}, variable::{OwnedTblExpressionVariableAtPath, TblExpressionVariableAtPath}}, expressions::types::assigned::{atom::{TblExpressionAtomAtPath, OwnedTblExpressionAtomAtPath}, compound::{TblExpressionCompoundAtPath, OwnedTblExpressionCompoundAtPath}}};

mod with_path_specified;
pub use with_path_specified::*;

pub enum UnassignedTblExpressionAtPathEnum<'a,C: UnassignedTblExpressionCompound, Path> {
    Atom(TblExpressionAtomAtPath<'a,Path>),
    Compound(UnassignedTblExpressionCompoundAtPath<'a,C,Path>),
    Variable(TblExpressionVariableAtPath<'a,Path>)
}

impl <'a,C:UnassignedTblExpressionCompound,Path> UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    pub fn is_atom(&self) -> bool { if let UnassignedTblExpressionAtPathEnum::Atom(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let UnassignedTblExpressionAtPathEnum::Compound(_) = self { true } else { false } }
    pub fn is_variable(&self) -> bool { if let UnassignedTblExpressionAtPathEnum::Variable(_) = self { true } else { false } }
}
impl <'a,C:UnassignedTblExpressionCompound,Path> TryInto<TblExpressionAtomAtPath<'a,Path>> for UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    type Error = ();
    fn try_into(self) -> Result<TblExpressionAtomAtPath<'a,Path>, Self::Error> { match self {
        UnassignedTblExpressionAtPathEnum::Atom(atom) => Ok(atom),
        _ => Err(()),
    }}
}
impl <'a,C:UnassignedTblExpressionCompound,Path> TryInto<TblExpressionCompoundAtPath<'a,C,Path>> for UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    type Error = ();
    fn try_into(self) -> Result<TblExpressionCompoundAtPath<'a,C,Path>, Self::Error> { match self {
        UnassignedTblExpressionAtPathEnum::Compound(compound) => Ok(compound),
        _ => Err(())
    }}
}
impl <'a,C:UnassignedTblExpressionCompound,Path> TryInto<TblExpressionVariableAtPath<'a,Path>> for UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    type Error = ();
    fn try_into(self) -> Result<TblExpressionVariableAtPath<'a,Path>, Self::Error> { match self {
        UnassignedTblExpressionAtPathEnum::Variable(variable) => Ok(variable),
        _ => Err(())
    }}
}

impl <'a,C: UnassignedTblExpressionCompound, Path> From<UnassignedTblExpressionAtPath<'a,C,Path>> for UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    fn from(value: ObjAtPath<'a,UnassignedTblExpression<C>,Path>) -> Self { match value.obj {
        UnassignedTblExpression::Atom(atom) => Self::Atom(ObjAtPath { obj: atom, path: value.path }),
        UnassignedTblExpression::Compound(compound) => Self::Compound(ObjAtPath { obj: compound, path: value.path }),
        UnassignedTblExpression::Variable(variable) => Self::Variable(ObjAtPath { obj: variable, path: value.path })
    }}
}
// impl <'a,Path> Into<OwnedObjAtPath<Expression,Path>> for ExpressionAtPathEnum<'a,Path> {
//     fn into(self) -> ObjAtPath<Expression, Path> { match self {
//         Self::Atomic(inner) => ObjAtPath { obj: Expression::Atomic(inner.obj), path: inner.path },
//         Self::Compound(inner) => ObjAtPath { obj: Expression::Compound(inner.obj), path: inner.path },
//     }}
// }


pub enum OwnedUnassignedTblExpressionAtPathEnum<C: UnassignedTblExpressionCompound, Path> {
    Atomic(OwnedTblExpressionAtomAtPath<Path>),
    Compound(OwnedUnassignedTblExpressionCompoundAtPath<C,Path>),
    Variable(OwnedTblExpressionVariableAtPath<Path>)

}
impl <C:UnassignedTblExpressionCompound,Path> OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    pub fn is_atom(&self) -> bool { if let OwnedUnassignedTblExpressionAtPathEnum::Atomic(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let OwnedUnassignedTblExpressionAtPathEnum::Atomic(_) = self { true } else { false } }
}
impl <C:UnassignedTblExpressionCompound,Path> TryInto<OwnedTblExpressionAtomAtPath<Path>> for OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    type Error = ();
    fn try_into(self) -> Result<OwnedTblExpressionAtomAtPath<Path>, Self::Error> { match self {
        OwnedUnassignedTblExpressionAtPathEnum::Atomic(atom) => Ok(atom),
        _ => Err(())
    }}
}
impl <C:UnassignedTblExpressionCompound,Path> TryInto<OwnedTblExpressionCompoundAtPath<C,Path>> for OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    type Error = ();
    fn try_into(self) -> Result<OwnedTblExpressionCompoundAtPath<C,Path>, Self::Error> { match self {
        OwnedUnassignedTblExpressionAtPathEnum::Compound(compound) => Ok(compound),
        _ => Err(())
    }}
}

impl <C: UnassignedTblExpressionCompound, Path> From<OwnedObjAtPath<UnassignedTblExpression<C>,Path>> for OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    fn from(value: OwnedObjAtPath<UnassignedTblExpression<C>,Path>) -> Self { match value.obj {
        UnassignedTblExpression::Atom(atom) => Self::Atomic(OwnedObjAtPath { obj: atom, path: value.path }),
        UnassignedTblExpression::Compound(compound) => Self::Compound(OwnedObjAtPath { obj: compound, path: value.path }),
        UnassignedTblExpression::Variable(variable) => Self::Variable(OwnedObjAtPath { obj: variable, path: value.path }),
    }}
}
impl <C: UnassignedTblExpressionCompound,Path> Into<OwnedUnassignedTblExpressionAtPath<C,Path>> for OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    fn into(self) -> OwnedObjAtPath<UnassignedTblExpression<C>, Path> { match self {
        Self::Atomic(inner) => OwnedObjAtPath { obj: UnassignedTblExpression::Atom(inner.obj), path: inner.path },
        Self::Compound(inner) => OwnedObjAtPath { obj: UnassignedTblExpression::Compound(inner.obj), path: inner.path },
        Self::Variable(inner) => OwnedObjAtPath { obj: UnassignedTblExpression::Variable(inner.obj), path: inner.path },
    }}
}
