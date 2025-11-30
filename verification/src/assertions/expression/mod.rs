mod atomicity_check;
mod atomicity_equality_check;
mod atomicity_inequality_check;
mod length_check;
mod length_equality_check;
mod length_inequality_check;
mod value_check;
mod value_equality_check;
mod value_inequality_check;

pub use atomicity_check::*;
pub use atomicity_equality_check::*;
pub use atomicity_inequality_check::*;
pub use length_check::*;
pub use length_equality_check::*;
pub use length_inequality_check::*;
pub use value_check::*;
pub use value_equality_check::*;
pub use value_inequality_check::*;

use path_lib::{obj_at_path::{ObjAtPathWithChildren, ObjAtPathWithDescendants}, paths::PathPair};
use tbl_structures::{expressions::{Expression, ExpressionInExpressionPath}, path_composites::{ExpressionInProof, ExpressionInProofPath, OwnedExpressionInProof}, DisplayExt};

pub struct ExpressionSubpathError {
    subpath: ExpressionInExpressionPath,
    expression: OwnedExpressionInProof
}
impl ExpressionSubpathError {
    fn new(subpath: ExpressionInExpressionPath, expression: OwnedExpressionInProof ) -> Self
        { Self { subpath, expression } }
}

pub fn format_expression_subpath_error(err: ExpressionSubpathError) -> String {
    format!("Expression at {path} has no subexpression at subpath {subpath}",
        path=err.expression.0.path(),
        subpath=err.subpath.display()
    )
}

pub fn expression_subexpression<'a>(expression: &'a ExpressionInProof<'a>, subpath: ExpressionInExpressionPath) -> Result<ExpressionInProof<'a>,ExpressionSubpathError> {
    return match expression.0.get_located_descendant(subpath.clone()) {
        Ok(c) => Ok(ExpressionInProof(c.replace_path(
            |p: PathPair<ExpressionInProofPath,ExpressionInExpressionPath>| p.into()
        ))), Err(_) => { Err(ExpressionSubpathError::new(subpath, expression.clone().into_owned())) }
    };
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

pub fn expression_as_slice<'a>(expression: &OwnedExpressionInProof) -> Result<Vec<OwnedExpressionInProof>,ExpressionAtomicityCheckError> {
    if let Expression::Atomic(_) = expression.0.obj() { return Err(ExpressionAtomicityCheckError::new(false,expression.to_owned())) };
    Ok(expression.0.get_located_children_owned()
        .into_iter()
        .map(|obj| OwnedExpressionInProof(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedExpressionInProof>>())
}

pub fn expression_as_sized_slice<'a,const EXPECTED_SIZE: usize>(expression: &OwnedExpressionInProof) -> Result<Result<Box<[OwnedExpressionInProof; EXPECTED_SIZE]>,ExpressionLengthCheckError>,ExpressionAtomicityCheckError> {
    match expression_as_slice(expression)?
        .try_into() {
        Ok(a) => Ok(Ok(a)),
        Err(_) => Ok(Err(ExpressionLengthCheckError::new(EXPECTED_SIZE, expression.to_owned()))),
    }
}
