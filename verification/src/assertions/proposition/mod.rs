mod proposition_atomicity_check;
mod proposition_length_check;
mod proposition_value_check;

use path_lib::obj_at_path::ObjAtPathWithChildren;
pub use proposition_atomicity_check::assert_proposition_atomicity;
pub use proposition_length_check::{assert_proposition_length, proposition_length_stringifier};
pub use proposition_value_check::assert_proposition_value;
use tbl_structures::path_composites::{OwnedExpressionInProof, OwnedPropositionInProof};

use crate::errors::{specification_error::NaryStringifier, ProofStepSpecificationError};

pub fn proposition_as_slice(proposition: &OwnedPropositionInProof) -> Vec<OwnedExpressionInProof> {
    proposition.get_located_children_owned()
        .into_iter()
        .map(|obj| obj.replace_path(|p| p.into()))
        .collect::<Vec<OwnedExpressionInProof>>()
}

pub fn proposition_as_sized_slice<'a,const expected_size: usize>(proposition: &OwnedPropositionInProof) -> Result<Box<[OwnedExpressionInProof; expected_size]>,ProofStepSpecificationError<'a>> {
    match proposition_as_slice(proposition)
        .try_into() {
            Ok(a) => Ok(a),
            Err(_) => Err(ProofStepSpecificationError::from_inner(proposition_length_stringifier(expected_size).assign([proposition.to_owned()]))),
        }
}
