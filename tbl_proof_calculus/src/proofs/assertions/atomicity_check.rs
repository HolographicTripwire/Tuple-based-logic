use path_lib::obj_at_path::OwnedObjAtPath;

use crate::expressions::types::assigned::{
    TblExpression, TblExpressionAtPath,
    atom::OwnedTblExpressionAtomAtPath,
    compound::{OwnedTblExpressionCompoundAtPath, TblExpressionCompound},
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExpressionCheckAtomicError<C: TblExpressionCompound, Path>(
    pub OwnedTblExpressionCompoundAtPath<C, Path>,
);
impl<C: TblExpressionCompound, Path> ExpressionCheckAtomicError<C, Path> {
    pub fn into_expression(self) -> TblExpression<C> {
        TblExpression::Compound(self.0.obj)
    }
}

pub fn assert_expression_atomic<'a, C: TblExpressionCompound, Path: Clone>(
    expr: &TblExpressionAtPath<'a, C, Path>,
) -> Result<(), ExpressionCheckAtomicError<C, Path>> {
    if let TblExpression::Compound(c) = expr.obj {
        Err(ExpressionCheckAtomicError(OwnedObjAtPath {
            obj: c.clone(),
            path: expr.path.clone(),
        }))
    } else {
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExpressionCheckInatomicError<Path>(pub OwnedTblExpressionAtomAtPath<Path>);
impl<Path> ExpressionCheckInatomicError<Path> {
    pub fn into_expression<C: TblExpressionCompound>(self) -> TblExpression<C> {
        TblExpression::Atom(self.0.obj)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExpressionAtomicityCheckError<C: TblExpressionCompound, Path> {
    ExpectedCompoundFoundAtom(OwnedTblExpressionAtomAtPath<Path>),
    ExpectedAtomFoundCompound(OwnedTblExpressionCompoundAtPath<C, Path>),
}
impl<C: TblExpressionCompound, Path> ExpressionAtomicityCheckError<C, Path> {
    pub fn get_expected_atomicity(&self) -> bool {
        match self {
            ExpressionAtomicityCheckError::ExpectedCompoundFoundAtom(_) => true,
            ExpressionAtomicityCheckError::ExpectedAtomFoundCompound(_) => false,
        }
    }
    pub fn get_actual_atomicity(&self) -> bool {
        !self.get_expected_atomicity()
    }
    pub fn into_expression(self) -> TblExpression<C> {
        match self {
            ExpressionAtomicityCheckError::ExpectedCompoundFoundAtom(atom) => {
                TblExpression::Atom(atom.obj)
            }
            ExpressionAtomicityCheckError::ExpectedAtomFoundCompound(compound) => {
                TblExpression::Compound(compound.obj)
            }
        }
    }
}

/// Check that the provided [Expression](ExpressionInInference) has an atomicty equal to expected_atomicity, returning an error otherwise
pub fn assert_expression_atomicity<'a, C, Path>(
    expr: &TblExpressionAtPath<'a, C, Path>,
    expected_atomicity: bool,
) -> Result<(), ExpressionAtomicityCheckError<C, Path>>
where
    C: TblExpressionCompound,
    Path: Clone,
{
    match expr.obj {
        TblExpression::Atom(atom) => {
            if expected_atomicity == true {
                Ok(())
            } else {
                Err(ExpressionAtomicityCheckError::ExpectedCompoundFoundAtom(
                    OwnedObjAtPath {
                        obj: *atom,
                        path: expr.path.clone(),
                    },
                ))
            }
        }
        TblExpression::Compound(compound) => {
            if expected_atomicity == false {
                Ok(())
            } else {
                Err(ExpressionAtomicityCheckError::ExpectedAtomFoundCompound(
                    OwnedObjAtPath {
                        obj: compound.clone(),
                        path: expr.path.clone(),
                    },
                ))
            }
        }
    }
}
