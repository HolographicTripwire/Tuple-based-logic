mod atomicity_check;
mod atomicity_equality_check;
mod atomicity_inequality_check;
mod length_check;
mod length_equality_check;
mod length_inequality_check;
mod value_check;
mod value_equality_check;
mod value_inequality_check;
mod functional;

use std::fmt::Display;

pub use atomicity_check::*;
pub use atomicity_equality_check::*;
pub use atomicity_inequality_check::*;
pub use length_check::*;
pub use length_equality_check::*;
pub use length_inequality_check::*;
pub use value_check::*;
pub use value_equality_check::*;
pub use value_inequality_check::*;
pub use functional::*;

use crate::structures::expressions::{TblExpression, compound::CompoundTblExpression, located::{OwnedTblExpressionAtPath, TblExpressionAtPath}, subexpressions::{SubexpressionInExpressionPath, immediate::{ImmediateSubexpressionInExpressionPath, LocatedParentOfImmediateSubexpressions as _}}};

pub struct ExpressionSubpathError<C: CompoundTblExpression, ParentPath> {
    pub subpath: SubexpressionInExpressionPath,
    pub expression: OwnedTblExpressionAtPath<C,ParentPath>
}

pub fn format_expression_subpath_error<C:CompoundTblExpression,Path:Display>(err: ExpressionSubpathError<C,Path>) -> String {
    format!("Expression at {path} has no subexpression at subpath {subpath}",
        path=err.expression.path,
        subpath=err.subpath
    )
}

pub fn expression_subexpression<'a,C:CompoundTblExpression,ParentPath:Clone,JoinedPath: From<(ParentPath,SubexpressionInExpressionPath)>>(expression: &'a TblExpressionAtPath<'a,C,ParentPath>, subpath: SubexpressionInExpressionPath) -> Result<TblExpressionAtPath<'a,C,JoinedPath>,ExpressionSubpathError<C,ParentPath>> {
    let result = match expression.obj {
        TblExpression::Atomic(_) => None,
        TblExpression::Compound(compound) => Some(compound.get_located_subexpression(subpath.clone())),
    };
    match result {
        Some(Ok(subexpression)) => Ok(subexpression.prepend_path_to_self(expression.path.clone())),
        None | Some(Err(_)) => Err(ExpressionSubpathError {
            subpath,
            expression: expression.clone().into()
        })
    }
    /* 
    TODO: Fix or delete
    let child: Result<ExpressionInProof, ()> = expression
        .get_located_child_owned(subpath)
        .map(|e| e.replace_path(|p: PathPair<ProofSubexpressionPath,AtomicSubexpressionPath>| p.into()) );

    for atom in subpath.paths() {
        let child: Result<ObjAtPath<'a, Expression, ProofSubexpressionPath>, ()> = expression
            .get_located_child( *atom)
            .map(|e| e.replace_path(|p: PathPair<ProofSubexpressionPath,AtomicSubexpressionPath>| p.into()) );
        match child {
                Ok(e) => { expression = e; }
                Err(_) => { return Err(ProofStepSpecificationError::from_inner(expression_subpath_stringifier(subpath).assign([expression.into_owned()]))) }
        };
    }
    Ok(expression)
     */
}

pub fn expression_as_slice<'a,C:CompoundTblExpression,Path:'a + Clone, JoinedPath: From<(Path,ImmediateSubexpressionInExpressionPath)>>(expression: &'a TblExpressionAtPath<'a,C,Path>) -> Result<Box<[TblExpressionAtPath<'a,C,JoinedPath>]>,ExpressionAtomicityCheckError<C,Path>> {
    if let TblExpression::Atomic(_) = expression.obj { return Err(ExpressionAtomicityCheckError {
        expected_atomicity: false,
        expression: expression.clone().into()
    }) };
    Ok(expression.get_located_immediate_subexpressions()
        .into_iter()
        .map(|obj| obj.into())
        .collect())
}
// pub fn expression_into_slice<'a,C:CompoundTblExpression,Path:'a + Clone,JoinedPath: From<(Path,ImmediateSubexpressionInExpressionPath)>>(expression: &TblExpressionAtPath<'a,C,Path>) -> Result<Box<[TblExpressionAtPath<'a,C,JoinedPath>]>,ExpressionAtomicityCheckError<C,Path>> {
//     if let TblExpression::Atomic(_) = expression.obj { return Err(ExpressionAtomicityCheckError {
//         expected_atomicity: false,
//         expression: expression.clone().into()
//     }) };
//     Ok(expression.into_located_immediate_subexpressions()
//         .into_iter()
//         .collect()
//     )
// }

pub fn expression_as_sized_slice<'a,const EXPECTED_SIZE: usize,C:CompoundTblExpression,Path:Clone + From<(Path,ImmediateSubexpressionInExpressionPath)>>(expression: &'a TblExpressionAtPath<'a,C,Path>) -> Result<Box<[TblExpressionAtPath<'a,C,Path>; EXPECTED_SIZE]>,ExpressionLengthCheckError<C,Path>> {
    Ok(expression_as_slice(expression)
        .map_err(|_| ExpressionLengthCheckError{
            expected_length: EXPECTED_SIZE, 
            expression: expression.clone().into()
        })?
        .try_into()
        .map_err(|_| ExpressionLengthCheckError{
            expected_length: EXPECTED_SIZE, 
            expression: expression.clone().into()
        })?)
}
// pub fn expression_into_sized_slice<'a,const EXPECTED_SIZE: usize,C:CompoundTblExpression,Path:Clone + From<(Path,ImmediateSubexpressionInExpressionPath)>>(expression: TblExpressionAtPath<'a,C,Path>) -> Result<Box<[TblExpressionAtPath<'a,C,Path>; EXPECTED_SIZE]>,ExpressionLengthCheckError<C,Path>> {
//     Ok(expression_into_slice(expression.clone())
//         .map_err(|_| ExpressionLengthCheckError{
//             expected_length: EXPECTED_SIZE, 
//             expression: expression.clone().into()
//         })?
//         .try_into()
//         .map_err(|_| ExpressionLengthCheckError{
//             expected_length: EXPECTED_SIZE, 
//             expression: expression.clone().into()
//         })?)
// }
