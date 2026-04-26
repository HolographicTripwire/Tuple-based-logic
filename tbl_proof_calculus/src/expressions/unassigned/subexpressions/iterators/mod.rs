use crate::expressions::{assigned::atomic::{AtomicTblExpression, AtomicTblExpressionAtPath}, unassigned::{UnassignedTblExpression, UnassignedTblExpressionAtPath, compound::UnassignedCompoundTblExpression, variable::{TblExpressionVariable, TblExpressionVariableAtPath}}};

pub mod depth_first;

trait UnassignedTblExpressionIterator<'a,C: 'a + UnassignedCompoundTblExpression>: Sized + Iterator<Item = &'a UnassignedTblExpression<C>> {
    fn filter_atoms(self) -> impl Iterator<Item = AtomicTblExpression> { self.filter_map(|expr| match expr {
        UnassignedTblExpression::Atomic(atom) => Some(*atom),
        UnassignedTblExpression::Variable(_) => None,
        UnassignedTblExpression::Compound(_) => None,
    })}
    fn filter_variables(self) -> impl Iterator<Item = TblExpressionVariable> { self.filter_map(|expr| match expr {
        UnassignedTblExpression::Atomic(_) => None,
        UnassignedTblExpression::Variable(variable) => Some(*variable),
        UnassignedTblExpression::Compound(_) => None,
    })}
    // fn filter_atoms_and_variables(self) -> impl Iterator<Item = AtomicTblExpression> { self.filter_map(|expr| match expr {
    //     UnassignedTblExpression::Atomic(atom) => Some(*atom),
    //     UnassignedTblExpression::Variable(_) => None,
    //     UnassignedTblExpression::Compound(_) => None,
    // })}
}
impl <'a,C: 'a + UnassignedCompoundTblExpression,I: Iterator<Item = &'a UnassignedTblExpression<C>>> UnassignedTblExpressionIterator<'a,C> for I {}

trait UnassignedTblExpressionAtPathIterator<'a,C: 'a + UnassignedCompoundTblExpression,Path>: Sized + Iterator<Item = UnassignedTblExpressionAtPath<'a,C,Path>> {
    fn filter_atoms(self) -> impl Iterator<Item = AtomicTblExpressionAtPath<'a,Path>> { self.filter_map(|expr| match expr.obj {
        UnassignedTblExpression::Atomic(atom) => Some(AtomicTblExpressionAtPath { obj: atom, path: expr.path }),
        UnassignedTblExpression::Variable(_) => None,
        UnassignedTblExpression::Compound(_) => None,
    })}
    fn filter_variables(self) -> impl Iterator<Item = TblExpressionVariableAtPath<'a,Path>> { self.filter_map(|expr| match expr.obj {
        UnassignedTblExpression::Atomic(atom) => None,
        UnassignedTblExpression::Variable(variable) => Some(TblExpressionVariableAtPath { obj: variable, path: expr.path }),
        UnassignedTblExpression::Compound(_) => None,
    })}
}
impl <'a,C: 'a + UnassignedCompoundTblExpression,Path,I: Iterator<Item = UnassignedTblExpressionAtPath<'a,C,Path>>> UnassignedTblExpressionAtPathIterator<'a,C,Path> for I {}
