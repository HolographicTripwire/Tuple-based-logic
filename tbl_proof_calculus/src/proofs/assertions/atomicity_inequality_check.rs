use crate::expressions::types::assigned::{OwnedTblExpressionAtPath, TblExpressionAtPath, at_path_enum::TblExpressionAtPathEnum, atom::TblExpressionAtomAtPath, compound::{TblExpressionCompound, TblExpressionCompoundAtPath}};

pub enum ExpressionAtomicityInequalityError<C: TblExpressionCompound, Path> {
    BothAtoms(OwnedAtomicTblExpressionAtPath<C,Path>, OwnedAtomicTblExpressionAtPath<C,Path>),
    BothCompounds(OwnedCompoundTblExpressionAtPath<C,Path>, OwnedCompoundTblExpressionAtPath<C,Path>)
}

/// Check that the provided [Expressions](ExpressionInInference) have inequal atomicity, returning an error otherwise
pub fn assert_expression_atomicity_inequality<'a,C: TblExpressionCompound,Path:Clone>(expr1: &TblExpressionAtPath<C, Path>, expr2: &TblExpressionAtPath<C,Path>) -> Result<(), ExpressionAtomicityInequalityError<C,Path>> {
    match (expr1.into(), expr2.into()) {
        (TblExpressionAtPathEnum::Atom(atom1), TblExpressionAtPathEnum::Atom(atom2))
            => { Err(ExpressionAtomicityInequalityError::BothAtoms(atom1.into(), atom2.into())) },
        (TblExpressionAtPathEnum::Compound(compound1), TblExpressionAtPathEnum::Compound(compound2))
            => { Err(ExpressionAtomicityInequalityError::BothCompounds(compound1.into(), compound2.into())) },
        _ => Ok(())
    }
}
