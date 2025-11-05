use tbl_structures::{inference::InferenceRule, path_composites::OwnedPropositionInProof};

use crate::{assertions::expression::stringify_atomicity, errors::specification_error::{Assessor, AssessedStringifier, ProofStepSpecificationError, StringifiablePredicate}};

/// Get a [Predicate](NaryPredicate) which takes an [Proposition](OwnedPropositionInProof) and checks if its atomicity is the expected value
fn proposition_atomicity_predicate<'a>(atomicity_expected: bool) -> impl Assessor<'a,OwnedPropositionInProof,()> {
    move |o: OwnedPropositionInProof| 
    if o.0.obj().as_atom().is_ok() == atomicity_expected { Ok(()) } else { Err(()) }
}

/// Get a [Stringifier](NaryStringifier) which takes an [Proposition](OwnedPropositionInProof) and returns an error message saying that this proposition's atomicity is not the expected value
fn proposition_atomicity_stringifier<'a>(atomicity_expected: bool) -> impl AssessedStringifier<'a,OwnedPropositionInProof,()> {
    move |o: OwnedPropositionInProof,_| format!(
        "Proposition at {path} has wrong atomicity (expected {atomicity_expected}; found {atomicity_actual})",
        path=o.0.path().to_string(),
        atomicity_expected=stringify_atomicity(atomicity_expected),
        atomicity_actual=stringify_atomicity(o.0.obj().as_atom().is_ok())
    )
}
/// Get a [Checker](StringifiablePredicate) which takes an [Proposition](OwnedPropositionInProof) and returns an error message if this proposition's atomicity is not the expected value
pub fn proposition_atomicity_check<'a>(atomicity_expected: bool) -> StringifiablePredicate<'a,OwnedPropositionInProof,()> { StringifiablePredicate::new(
    proposition_atomicity_predicate(atomicity_expected),
    proposition_atomicity_stringifier(atomicity_expected),
)}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an atomicity equal to atomicty_expected, returning an error otherwise
pub fn assert_proposition_atomicity<'a,Rule:InferenceRule>(expr: OwnedPropositionInProof, atomicity_expected: bool) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_atomicity_check::<'a>(atomicity_expected)
        .evaluate(expr)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
