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
use tbl_structures::{DisplayExt, expressions::{Expression, ExpressionInExpressionPath}, path_composites::{ExpressionInInference, ExpressionInInferencePath, OwnedExpressionInInference}};

pub struct ExpressionSubpathError {
    pub subpath: ExpressionInExpressionPath,
    pub expression: OwnedExpressionInInference
}

pub fn format_expression_subpath_error(err: ExpressionSubpathError) -> String {
    format!("Expression at {path} has no subexpression at subpath {subpath}",
        path=err.expression.0.path(),
        subpath=err.subpath.display()
    )
}

pub fn expression_subexpression<'a>(expression: &'a ExpressionInInference<'a>, subpath: ExpressionInExpressionPath) -> Result<ExpressionInInference<'a>,ExpressionSubpathError> {
    return match expression.0.get_located_descendant(subpath.clone()) {
        Ok(c) => Ok(ExpressionInInference(c.replace_path(
            |p: PathPair<ExpressionInInferencePath,ExpressionInExpressionPath>| p.into()
        ))), Err(_) => { Err(ExpressionSubpathError {
            subpath,
            expression: expression.clone().into_owned()
        }) }
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

pub fn expression_as_slice<'a>(expression: &'a ExpressionInInference) -> Result<Vec<ExpressionInInference<'a>>,ExpressionAtomicityCheckError> {
    if let Expression::Atomic(_) = expression.0.obj() { return Err(ExpressionAtomicityCheckError {
        expected_atomicity: false,
        expression: expression.clone().into_owned()
    }) };
    Ok(expression.0.get_located_children()
        .into_iter()
        .map(|obj| ExpressionInInference(obj.replace_path(|p| p.into())))
        .collect::<Vec<ExpressionInInference>>())
}

pub fn expression_as_sized_slice<'a,const EXPECTED_SIZE: usize>(expression: &'a ExpressionInInference) -> Result<Result<Box<[ExpressionInInference<'a>; EXPECTED_SIZE]>,ExpressionLengthCheckError>,ExpressionAtomicityCheckError> {
    match expression_as_slice(expression)?
        .try_into() {
        Ok(a) => Ok(Ok(a)),
        Err(_) => Ok(Err(ExpressionLengthCheckError{
            expected_length: EXPECTED_SIZE, 
            expression: expression.clone().into_owned()
        })),
    }
}
