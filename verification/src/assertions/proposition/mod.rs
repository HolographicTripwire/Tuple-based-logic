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

use path_lib::{obj_at_path::{ObjAtPathWithChildren, ObjAtPathWithDescendants}, paths::PathPair};
use tbl_structures::{DisplayExt, expressions::{ExpressionInExpressionPath, ExpressionInPropositionPath, Proposition}, path_composites::ExpressionInInference, proof::{OwnedPropositionInInference, PropositionInInference, PropositionInInferencePath}};

pub struct PropositionSubpathError {
    pub subpath: ExpressionInExpressionPath,
    pub proposition: OwnedPropositionInInference
}


pub fn format_proposition_subpath_error(err: PropositionSubpathError) -> String {
    format!("Proposition at {path} has no subproposition at subpath {subpath}",
        path=err.proposition.0.path(),
        subpath=err.subpath.display()
    )
}

pub fn proposition_subproposition<'a>(proposition: &'a PropositionInInference<'a>, subpath: ExpressionInPropositionPath) -> Result<ExpressionInInference<'a>,PropositionSubpathError> {
    return match proposition.0.get_located_descendant(subpath.clone()) {
        Ok(c) => Ok(ExpressionInInference(c.replace_path(
            |p: PathPair<PropositionInInferencePath,ExpressionInPropositionPath>| p.into()
        ))), Err(_) => { Err(PropositionSubpathError {
            subpath,
            proposition: proposition.clone().into_owned()
        }) }
    };
    /* 
    TODO: Fix or delete
    let child: Result<PropositionInProof, ()> = proposition
        .get_located_child_owned(subpath)
        .map(|e| e.replace_path(|p: PathPair<ProofSubpropositionPath,AtomicSubpropositionPath>| p.into()) );

    for atom in subpath.paths() {
        let child: Result<ObjAtPath<'a, Proposition, ProofSubpropositionPath>, ()> = proposition
            .get_located_child( *atom)
            .map(|e| e.replace_path(|p: PathPair<ProofSubpropositionPath,AtomicSubpropositionPath>| p.into()) );
        match child {
                Ok(e) => { proposition = e; }
                Err(_) => { return Err(ProofStepSpecificationError::from_inner(proposition_subpath_stringifier(subpath).assign([proposition.into_owned()]))) }
        };
    }
    Ok(proposition)
     */
}

pub fn proposition_as_slice<'a>(proposition: &'a PropositionInInference) -> Result<Vec<ExpressionInInference<'a>>,PropositionAtomicityCheckError> {
    if let Proposition::Atomic(_) = proposition.0.obj() { return Err(PropositionAtomicityCheckError {
        expected_atomicity: false,
        proposition: proposition.clone().into_owned()
    }) };
    Ok(proposition.0.get_located_children()
        .into_iter()
        .map(|obj| ExpressionInInference(obj.replace_path(|p| p.into())))
        .collect::<Vec<ExpressionInInference>>())
}
pub fn proposition_into_slice<'a>(proposition: PropositionInInference<'a>) -> Result<Vec<ExpressionInInference<'a>>,PropositionAtomicityCheckError> {
    if let Proposition::Atomic(_) = proposition.0.obj() { return Err(PropositionAtomicityCheckError {
        expected_atomicity: false,
        proposition: proposition.clone().into_owned()
    }) };
    Ok(proposition.0.into_located_children()
        .into_iter()
        .map(|expr| ExpressionInInference(expr.replace_path(|path| path.into())))
        .collect::<Vec<ExpressionInInference>>())
}

pub fn proposition_as_sized_slice<'a,const EXPECTED_SIZE: usize>(proposition: &'a PropositionInInference) -> Result<Box<[ExpressionInInference<'a>; EXPECTED_SIZE]>,PropositionLengthCheckError> {
    Ok(proposition_as_slice(proposition)
        .map_err(|_| PropositionLengthCheckError{
            expected_length: EXPECTED_SIZE, 
            proposition: proposition.clone().into_owned()
        })?
        .try_into()
        .map_err(|_| PropositionLengthCheckError{
            expected_length: EXPECTED_SIZE, 
            proposition: proposition.clone().into_owned()
        })?)
}
pub fn proposition_into_sized_slice<'a,const EXPECTED_SIZE: usize>(proposition: PropositionInInference<'a>) -> Result<Box<[ExpressionInInference<'a>; EXPECTED_SIZE]>,PropositionLengthCheckError> {
    Ok(proposition_into_slice(proposition.clone())
        .map_err(|_| PropositionLengthCheckError{
            expected_length: EXPECTED_SIZE, 
            proposition: proposition.clone().into_owned()
        })?
        .try_into()
        .map_err(|_| PropositionLengthCheckError{
            expected_length: EXPECTED_SIZE, 
            proposition: proposition.clone().into_owned()
        })?)
}