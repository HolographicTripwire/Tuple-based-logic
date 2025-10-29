mod expression_atomicity_check;
mod expression_atomicity_equality_check;
mod expression_atomicity_inequality_check;
mod expression_length_check;
mod expression_length_equality_check;
mod expression_length_inequality_check;
mod expression_value_check;
mod expression_value_equality_check;
mod expression_value_inequality_check;

pub use expression_atomicity_check::*;
pub use expression_atomicity_equality_check::*;
pub use expression_atomicity_inequality_check::*;
pub use expression_length_check::*;
pub use expression_length_equality_check::*;
pub use expression_length_inequality_check::*;
pub use expression_value_check::*;
pub use expression_value_equality_check::*;
pub use expression_value_inequality_check::*;

use path_lib::{obj_at_path::{ObjAtPathWithChildren, ObjAtPathWithDescendants}, paths::PathPair};
use tbl_structures::{expressions::{Expression, ExpressionInExpressionPath}, path_composites::{ExpressionInProof, ExpressionInProofPath, OwnedExpressionInProof}, DisplayExt};

use crate::errors::{specification_error::NaryStringifier, ProofStepSpecificationError};

/// Convert atomicity to string
pub(crate) fn stringify_atomicity(is_atomic: bool) -> &'static str {
    if is_atomic { "atomic" } else { "not-atomic" }
}
/// Convert length of an expression to string
pub(crate) fn stringify_length(expr: &Expression) -> String {
    match expr.as_slice() {
        Ok(tuple) => tuple.len().to_string(),
        Err(()) => stringify_atomicity(true).to_string()
    }
}

pub fn expression_subpath_stringifier<'a>(subpath: ExpressionInExpressionPath) -> impl NaryStringifier<'a,1,OwnedExpressionInProof> {
    move |o: [OwnedExpressionInProof; 1]| format!(
        "Expression at {path} has no subexpression at subpath {subpath}",
        path=o[0].path().to_string(),
        subpath=subpath.display()
    )
}

pub fn subexpression<'a>(expression: &'a ExpressionInProof<'a>, subpath: ExpressionInExpressionPath) -> Result<ExpressionInProof<'a>,ProofStepSpecificationError<'a>> {
    return match expression.get_located_descendant(subpath.clone()) {
        Ok(c) => Ok(c.replace_path(
            |p: PathPair<ExpressionInProofPath,ExpressionInExpressionPath>| p.into()
        )), Err(_) => {
            let inner = expression_subpath_stringifier(subpath);
            let inner2 = inner.assign([expression.clone().into_owned()]);
            Err(ProofStepSpecificationError::from_inner(inner2))
        }
    };
    /* TODO: Delete or fix
    let child: Result<ObjAtPath<'a, Expression, ProofSubexpressionPath>, ()> = expression
        .get_located_child_owned(x())
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

pub fn expression_as_slice<'a>(expression: &OwnedExpressionInProof) -> Result<Vec<OwnedExpressionInProof>,ProofStepSpecificationError<'a>> {
    if let Expression::Atomic(_) = expression.obj() { return Err(ProofStepSpecificationError::from_inner(expression_atomicity_stringifier(false).assign([expression.clone()]))) };
    Ok(expression.get_located_children_owned()
        .into_iter()
        .map(|obj| obj.replace_path(|p| p.into()))
        .collect::<Vec<OwnedExpressionInProof>>())
}

pub fn expression_as_sized_slice<'a,const expected_size: usize>(expression: &OwnedExpressionInProof) -> Result<Box<[OwnedExpressionInProof; expected_size]>,ProofStepSpecificationError<'a>> {
    match expression_as_slice(expression)?
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => Err(ProofStepSpecificationError::from_inner(expression_length_stringifier(expected_size).assign([expression.to_owned()]))),
        }
}
