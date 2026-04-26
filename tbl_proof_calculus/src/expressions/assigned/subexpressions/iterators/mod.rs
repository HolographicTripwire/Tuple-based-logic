use crate::expressions::assigned::{TblExpression, TblExpressionAtPath, atomic::{AtomicTblExpression, AtomicTblExpressionAtPath}, compound::CompoundTblExpression};

pub mod depth_first;

trait TblExpressionIterator<'a,C: 'a + CompoundTblExpression>: Sized + Iterator<Item = &'a TblExpression<C>> {
    fn filter_atoms(self) -> impl Iterator<Item = AtomicTblExpression> { self.filter_map(|expr| match expr {
        TblExpression::Atomic(atom) => Some(*atom),
        TblExpression::Compound(_) => None,
    })}
}
impl <'a,C: 'a + CompoundTblExpression,I: Iterator<Item = &'a TblExpression<C>>> TblExpressionIterator<'a,C> for I {}

trait TblExpressionAtPathIterator<'a,C: 'a + CompoundTblExpression,Path>: Sized + Iterator<Item = TblExpressionAtPath<'a,C,Path>> {
    fn filter_atoms(self) -> impl Iterator<Item = AtomicTblExpressionAtPath<'a,Path>> { self.filter_map(|expr| match expr.obj {
        TblExpression::Atomic(atom) => Some(AtomicTblExpressionAtPath { obj: atom, path: expr.path }),
        TblExpression::Compound(_) => None,
    })}
}
impl <'a,C: 'a + CompoundTblExpression,Path,I: Iterator<Item = TblExpressionAtPath<'a,C,Path>>> TblExpressionAtPathIterator<'a,C,Path> for I {}
