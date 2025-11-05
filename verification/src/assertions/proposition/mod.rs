mod proposition_atomicity_check;
mod proposition_atomicity_equality_check;
mod proposition_atomicity_inequality_check;
mod proposition_length_check;
mod proposition_length_equality_check;
mod proposition_length_inequality_check;
mod proposition_value_check;
mod proposition_value_equality_check;
mod proposition_value_inequality_check;

pub use proposition_atomicity_check::*;
pub use proposition_atomicity_equality_check::*;
pub use proposition_atomicity_inequality_check::*;
pub use proposition_length_check::*;
pub use proposition_length_equality_check::*;
pub use proposition_length_inequality_check::*;
pub use proposition_value_check::*;
pub use proposition_value_equality_check::*;
pub use proposition_value_inequality_check::*;

use path_lib::obj_at_path::ObjAtPathWithChildren;
use tbl_structures::path_composites::{OwnedExpressionInProof, OwnedPropositionInProof};

use crate::errors::{specification_error::NaryStringifier, ProofStepSpecificationError};

pub fn proposition_as_slice(proposition: &OwnedPropositionInProof) -> Vec<OwnedExpressionInProof> {
    proposition.0.get_located_children_owned()
        .into_iter()
        .map(|obj| OwnedExpressionInProof(obj.replace_path(|p| p.into())))
        .collect::<Vec<OwnedExpressionInProof>>()
}

pub fn proposition_as_sized_slice<'a,const EXPECTED_SIZE: usize>(proposition: &OwnedPropositionInProof) -> Result<Box<[OwnedExpressionInProof; EXPECTED_SIZE]>,ProofStepSpecificationError<'a>> {
    match proposition_as_slice(proposition)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => Err(ProofStepSpecificationError::from_inner(proposition_length_stringifier(EXPECTED_SIZE).assign(proposition.to_owned()))),
        }
}
