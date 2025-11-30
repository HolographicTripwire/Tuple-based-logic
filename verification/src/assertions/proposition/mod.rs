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
use tbl_structures::{DisplayExt, expressions::{ExpressionInExpressionPath, Proposition}, path_composites::{ExpressionInProof, OwnedExpressionInProof, OwnedPropositionInProof, PropositionInProof, PropositionInProofPath}};

pub struct PropositionSubpathError {
    subpath: ExpressionInExpressionPath,
    proposition: OwnedPropositionInProof
}
impl PropositionSubpathError {
    fn new(subpath: ExpressionInExpressionPath, proposition: OwnedPropositionInProof ) -> Self
        { Self { subpath, proposition } }
}

pub fn format_proposition_subpath_error(err: PropositionSubpathError) -> String {
    format!("Proposition at {path} has no subexpression at subpath {subpath}",
        path=err.proposition.0.path(),
        subpath=err.subpath.display()
    )
}


pub fn proposition_subexpression<'a>(proposition: &'a PropositionInProof<'a>, subpath: ExpressionInExpressionPath) -> Result<ExpressionInProof<'a>,PropositionSubpathError> {
    return match proposition.0.get_located_descendant(subpath.clone()) {
        Ok(c) => Ok(ExpressionInProof(c.replace_path(
            |p: PathPair<PropositionInProofPath,ExpressionInExpressionPath>| p.into()
        ))), Err(_) => { Err(PropositionSubpathError::new(subpath, proposition.clone().into_owned())) }
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

pub fn proposition_as_slice<'a>(proposition: &OwnedPropositionInProof) -> Result<Vec<OwnedExpressionInProof>,PropositionAtomicityCheckError> {
    if let Proposition::Atomic(_) = proposition.0.obj() { return Err(PropositionAtomicityCheckError::new(false,proposition.to_owned())) };
    Ok(proposition.0.get_located_children_owned()
        .into_iter()
        .map(|obj| OwnedExpressionInProof(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedExpressionInProof>>())
}

pub fn proposition_as_sized_slice<'a,const EXPECTED_SIZE: usize>(proposition: &OwnedPropositionInProof) -> Result<Result<Box<[OwnedExpressionInProof; EXPECTED_SIZE]>,PropositionLengthCheckError>,PropositionAtomicityCheckError> {
    match proposition_as_slice(proposition)?
        .try_into() {
        Ok(a) => Ok(Ok(a)),
        Err(_) => Ok(Err(PropositionLengthCheckError::new(EXPECTED_SIZE, proposition.to_owned()))),
    }
}
