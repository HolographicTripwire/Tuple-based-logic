use path_lib::obj_at_path::ObjAtPath;

use crate::expressions::types::assigned::{OwnedTblExpressionAtPath, TblExpression, TblExpressionAtPath, at_path_enum::TblExpressionAtPathEnum, atom::{TblExpressionAtomAtPath, OwnedTblExpressionAtomAtPath}, compound::{TblExpressionCompound, OwnedTblExpressionCompoundAtPath}};

#[derive(Clone)]
pub enum ExpressionAtomicityCheckError<C: TblExpressionCompound, Path> {
    ExpectedCompoundFoundAtom(OwnedTblExpressionAtomAtPath<Path>),
    ExpectedAtomFoundCompound(OwnedTblExpressionCompoundAtPath<C,Path>)
}
impl <C: TblExpressionCompound,Path> ExpressionAtomicityCheckError<C,Path> {
    pub fn get_expected_atomicity(&self) -> bool { match self {
        ExpressionAtomicityCheckError::ExpectedCompoundFoundAtom(_) => true,
        ExpressionAtomicityCheckError::ExpectedAtomFoundCompound(_) => false,
    } }
    pub fn get_actual_atomicity(&self) -> bool { !self.get_expected_atomicity() }
    pub fn into_expression(self) -> TblExpression<C> { self.expression.obj }
}

/// Check that the provided [Expression](ExpressionInInference) has an atomicty equal to expected_atomicity, returning an error otherwise
pub fn assert_expression_atomicity<'a,C,Path>(expr: &TblExpressionAtPath<'a,C,Path>, expected_atomicity: bool) -> Result<(), ExpressionAtomicityCheckError<C,Path>> where
C: TblExpressionCompound, Path: Clone {
    match expr.clone().into() {
        TblExpressionAtPathEnum::Atom(atom) => {
            if expected_atomicity == true { Ok(()) }
            else { Err(ExpressionAtomicityCheckError::ExpectedAtomFoundCompound(atom.into())) }
        }, TblExpressionAtPathEnum::Compound(compound) => {
            if expected_atomicity == false { Ok(()) }
            else { Err(ExpressionAtomicityCheckError::ExpectedCompoundFoundAtom(compound.into())) }
        }
    }
}
