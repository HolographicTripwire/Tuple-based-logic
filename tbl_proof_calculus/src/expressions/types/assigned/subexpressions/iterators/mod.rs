use crate::expressions::types::assigned::{
    TblExpression, TblExpressionAtPath,
    atom::{TblExpressionAtom, TblExpressionAtomAtPath},
    compound::TblExpressionCompound,
};

pub mod depth_first;

trait TblExpressionIterator<'a, C: 'a + TblExpressionCompound>:
    Sized + Iterator<Item = &'a TblExpression<C>>
{
    fn filter_atoms(self) -> impl Iterator<Item = TblExpressionAtom> {
        self.filter_map(|expr| match expr {
            TblExpression::Atom(atom) => Some(*atom),
            TblExpression::Compound(_) => None,
        })
    }
}
impl<'a, C: 'a + TblExpressionCompound, I: Iterator<Item = &'a TblExpression<C>>>
    TblExpressionIterator<'a, C> for I
{
}

trait TblExpressionAtPathIterator<'a, C: 'a + TblExpressionCompound, Path>:
    Sized + Iterator<Item = TblExpressionAtPath<'a, C, Path>>
{
    fn filter_atoms(self) -> impl Iterator<Item = TblExpressionAtomAtPath<'a, Path>> {
        self.filter_map(|expr| match expr.obj {
            TblExpression::Atom(atom) => Some(TblExpressionAtomAtPath {
                obj: atom,
                path: expr.path,
            }),
            TblExpression::Compound(_) => None,
        })
    }
}
impl<'a, C: 'a + TblExpressionCompound, Path, I: Iterator<Item = TblExpressionAtPath<'a, C, Path>>>
    TblExpressionAtPathIterator<'a, C, Path> for I
{
}
