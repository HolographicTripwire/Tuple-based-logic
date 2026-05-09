use path_lib::obj_at_path::OwnedObjAtPath;

use crate::expressions::types::assigned::{
    TblExpression, TblExpressionAtPath,
    atom::OwnedTblExpressionAtomAtPath,
    compound::{OwnedTblExpressionCompoundAtPath, TblExpressionCompound},
};

pub enum ExpressionAtomicityInequalityError<C: TblExpressionCompound, Path> {
    BothAtoms(
        OwnedTblExpressionAtomAtPath<Path>,
        OwnedTblExpressionAtomAtPath<Path>,
    ),
    BothCompounds(
        OwnedTblExpressionCompoundAtPath<C, Path>,
        OwnedTblExpressionCompoundAtPath<C, Path>,
    ),
}

/// Check that the provided [Expressions](ExpressionInInference) have inequal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_inequality<'a, C: TblExpressionCompound, Path: Clone>(
    expr1: &TblExpressionAtPath<C, Path>,
    expr2: &TblExpressionAtPath<C, Path>,
) -> Result<(), ExpressionAtomicityInequalityError<C, Path>> {
    match (expr1.obj, expr2.obj) {
        (TblExpression::Atom(atom1), TblExpression::Atom(atom2)) => {
            Err(ExpressionAtomicityInequalityError::BothAtoms(
                OwnedObjAtPath {
                    obj: *atom1,
                    path: expr1.path.clone(),
                },
                OwnedObjAtPath {
                    obj: *atom2,
                    path: expr2.path.clone(),
                },
            ))
        }
        (TblExpression::Compound(compound1), TblExpression::Compound(compound2)) => {
            Err(ExpressionAtomicityInequalityError::BothCompounds(
                OwnedObjAtPath {
                    obj: compound1.clone(),
                    path: expr1.path.clone(),
                },
                OwnedObjAtPath {
                    obj: compound2.clone(),
                    path: expr2.path.clone(),
                },
            ))
        }
        _ => Ok(()),
    }
}
