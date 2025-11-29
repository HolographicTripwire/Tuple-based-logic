use tbl_structures::{inference::InferenceRule, path_composites::OwnedPropositionInProof};

use crate::{assertions::expression::stringify_atomicity, errors::specification_error::{Assessor, AssessedErrorStringifier, ProofStepSpecificationError, ErrorStringifiableAssessor}};

/// Get a [Predicate](NaryPredicate) which takes n [Propositions](OwnedPropositionInProof) and checks if their atomicities are equal
fn proposition_atomicity_equality_predicate<'a,const N: usize>() -> impl Assessor<'a,[OwnedPropositionInProof;N],()> {
    move |os: [OwnedPropositionInProof; N]| { 
        let mut iter = os.iter().map(|o| o.0.obj().as_atom().is_ok());
        let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero propositions");
        for nth_atomicity in iter {
            if nth_atomicity != first_atomicity { return Err(()) }
        }
        Ok(())
    }
}
/// Get a [Stringifier](NaryStringifier) which takes n [Propositions](OwnedPropositionInProof) and returns an error message saying that their atomicities aren't equal
fn proposition_atomicity_equality_stringifier<'a,const N: usize>() -> impl AssessedErrorStringifier<'a,[OwnedPropositionInProof;N],()> {
    move |os: [OwnedPropositionInProof; N],_| format!(
        "Proposition atomicities expected to be equal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            stringify_atomicity(o.0.obj().as_atom().is_ok())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Proposition](OwnedPropositionInProof) and returns an error message if their atomicities aren't equal
pub fn proposition_atomicity_equality_check<'a,const N: usize>() -> ErrorStringifiableAssessor<'a,[OwnedPropositionInProof;N],()> { ErrorStringifiableAssessor::new(
    proposition_atomicity_equality_predicate(),
    proposition_atomicity_equality_stringifier(),
)}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an atomicity equal to atomicty_expected, returning an error otherwise
pub fn assert_proposition_atomicity_equality<'a,const N: usize, Rule:InferenceRule>(exprs: [OwnedPropositionInProof; N]) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_atomicity_equality_check::<'a>()
        .evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
