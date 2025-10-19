use std::fmt::Display;

use path_lib::Path;

use crate::{proof::ProofInProofPath, DisplayExt};

#[derive(Clone,PartialEq,Eq)]
pub struct InferenceInProofPath(pub ProofInProofPath);
impl Path for InferenceInProofPath {}
impl Display for InferenceInProofPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0.display())
    }
}
