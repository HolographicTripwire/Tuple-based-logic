use path_lib::obj_at_path::{ObjAtPath, OwnedObjAtPath};

use crate::{expressions::types::unassigned::{OwnedUnassignedTblExpressionAtPath, UnassignedTblExpression, UnassignedTblExpressionAtPath, compound::{OwnedUnassignedCompoundTblExpressionAtPath, UnassignedCompoundTblExpression, UnassignedCompoundTblExpressionAtPath}, variable::{OwnedTblExpressionVariableAtPath, TblExpressionVariableAtPath}}, expressions::types::assigned::{atomic::{AtomicTblExpressionAtPath, OwnedAtomicTblExpressionAtPath}, compound::{CompoundTblExpressionAtPath, OwnedCompoundTblExpressionAtPath}}};

mod with_path_specified;
pub use with_path_specified::*;

pub enum UnassignedTblExpressionAtPathEnum<'a,C: UnassignedCompoundTblExpression, Path> {
    Atomic(AtomicTblExpressionAtPath<'a,Path>),
    Compound(UnassignedCompoundTblExpressionAtPath<'a,C,Path>),
    Variable(TblExpressionVariableAtPath<'a,Path>)
}

impl <'a,C:UnassignedCompoundTblExpression,Path> UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    pub fn is_atom(&self) -> bool { if let UnassignedTblExpressionAtPathEnum::Atomic(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let UnassignedTblExpressionAtPathEnum::Compound(_) = self { true } else { false } }
    pub fn is_variable(&self) -> bool { if let UnassignedTblExpressionAtPathEnum::Variable(_) = self { true } else { false } }
}
impl <'a,C:UnassignedCompoundTblExpression,Path> TryInto<AtomicTblExpressionAtPath<'a,Path>> for UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    type Error = ();
    fn try_into(self) -> Result<AtomicTblExpressionAtPath<'a,Path>, Self::Error> { match self {
        UnassignedTblExpressionAtPathEnum::Atomic(atom) => Ok(atom),
        _ => Err(()),
    }}
}
impl <'a,C:UnassignedCompoundTblExpression,Path> TryInto<CompoundTblExpressionAtPath<'a,C,Path>> for UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    type Error = ();
    fn try_into(self) -> Result<CompoundTblExpressionAtPath<'a,C,Path>, Self::Error> { match self {
        UnassignedTblExpressionAtPathEnum::Compound(compound) => Ok(compound),
        _ => Err(())
    }}
}
impl <'a,C:UnassignedCompoundTblExpression,Path> TryInto<TblExpressionVariableAtPath<'a,Path>> for UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    type Error = ();
    fn try_into(self) -> Result<TblExpressionVariableAtPath<'a,Path>, Self::Error> { match self {
        UnassignedTblExpressionAtPathEnum::Variable(variable) => Ok(variable),
        _ => Err(())
    }}
}

impl <'a,C: UnassignedCompoundTblExpression, Path> From<UnassignedTblExpressionAtPath<'a,C,Path>> for UnassignedTblExpressionAtPathEnum<'a,C,Path> {
    fn from(value: ObjAtPath<'a,UnassignedTblExpression<C>,Path>) -> Self { match value.obj {
        UnassignedTblExpression::Atomic(atom) => Self::Atomic(ObjAtPath { obj: atom, path: value.path }),
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


pub enum OwnedUnassignedTblExpressionAtPathEnum<C: UnassignedCompoundTblExpression, Path> {
    Atomic(OwnedAtomicTblExpressionAtPath<Path>),
    Compound(OwnedUnassignedCompoundTblExpressionAtPath<C,Path>),
    Variable(OwnedTblExpressionVariableAtPath<Path>)

}
impl <C:UnassignedCompoundTblExpression,Path> OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    pub fn is_atom(&self) -> bool { if let OwnedUnassignedTblExpressionAtPathEnum::Atomic(_) = self { true } else { false } }
    pub fn is_compound(&self) -> bool { if let OwnedUnassignedTblExpressionAtPathEnum::Atomic(_) = self { true } else { false } }
}
impl <C:UnassignedCompoundTblExpression,Path> TryInto<OwnedAtomicTblExpressionAtPath<Path>> for OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    type Error = ();
    fn try_into(self) -> Result<OwnedAtomicTblExpressionAtPath<Path>, Self::Error> { match self {
        OwnedUnassignedTblExpressionAtPathEnum::Atomic(atom) => Ok(atom),
        _ => Err(())
    }}
}
impl <C:UnassignedCompoundTblExpression,Path> TryInto<OwnedCompoundTblExpressionAtPath<C,Path>> for OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    type Error = ();
    fn try_into(self) -> Result<OwnedCompoundTblExpressionAtPath<C,Path>, Self::Error> { match self {
        OwnedUnassignedTblExpressionAtPathEnum::Compound(compound) => Ok(compound),
        _ => Err(())
    }}
}

impl <C: UnassignedCompoundTblExpression, Path> From<OwnedObjAtPath<UnassignedTblExpression<C>,Path>> for OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    fn from(value: OwnedObjAtPath<UnassignedTblExpression<C>,Path>) -> Self { match value.obj {
        UnassignedTblExpression::Atomic(atom) => Self::Atomic(OwnedObjAtPath { obj: atom, path: value.path }),
        UnassignedTblExpression::Compound(compound) => Self::Compound(OwnedObjAtPath { obj: compound, path: value.path }),
        UnassignedTblExpression::Variable(variable) => Self::Variable(OwnedObjAtPath { obj: variable, path: value.path }),
    }}
}
impl <C: UnassignedCompoundTblExpression,Path> Into<OwnedUnassignedTblExpressionAtPath<C,Path>> for OwnedUnassignedTblExpressionAtPathEnum<C,Path> {
    fn into(self) -> OwnedObjAtPath<UnassignedTblExpression<C>, Path> { match self {
        Self::Atomic(inner) => OwnedObjAtPath { obj: UnassignedTblExpression::Atomic(inner.obj), path: inner.path },
        Self::Compound(inner) => OwnedObjAtPath { obj: UnassignedTblExpression::Compound(inner.obj), path: inner.path },
        Self::Variable(inner) => OwnedObjAtPath { obj: UnassignedTblExpression::Variable(inner.obj), path: inner.path },
    }}
}
