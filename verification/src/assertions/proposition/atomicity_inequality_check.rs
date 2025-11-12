use std::fmt::Display;

use tbl_structures::path_composites::{PropositionInProof, OwnedPropositionInProof};


pub struct PropositionAtomicityInequalityError {
    expr1: OwnedPropositionInProof,
    expr2: OwnedPropositionInProof,
}
impl PropositionAtomicityInequalityError {
    pub fn new(expr1: OwnedPropositionInProof, expr2: OwnedPropositionInProof) -> Self
        { Self { expr1, expr2 } }
}
impl Display for PropositionAtomicityInequalityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Atomicity of propositions {expr1} and {expr2} expected to be inequal, but both were {value}",
            expr1 = self.expr1.0.path(),
            expr2 = self.expr2.0.path(),
            value = self.expr1.0.obj().as_atom().is_ok()
        )
    }
}

/// Check that the provided [Propositions](OwnedPropositionInProof) have inequal atomicity, returning an error otherwise
pub fn assert_proposition_atomicity_inequality<'a>(expr1: &PropositionInProof, expr2: &PropositionInProof) -> Result<(), PropositionAtomicityInequalityError> {
    let first_atomicity = expr1.0.obj().as_atom().is_ok();
    let second_atomicity = expr2.0.obj().as_atom().is_ok();
    if first_atomicity == second_atomicity { Ok(()) }
    else { Err(PropositionAtomicityInequalityError::new(expr1.into_owned(), expr2.into_owned()).into()) }
}
