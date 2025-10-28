use tbl_structures::{inference::InferenceRule, path_composites::OwnedPropositionInProof};

use crate::{assertions::expression::stringify_length, errors::{specification_error::{NaryPredicate, NaryStringifier, StringifiablePredicate}, ProofStepSpecificationError}};

/// Get a [Predicate](NaryPredicate) which takes n [Propositions](OwnedPropositionInProof) and checks if their lengths are equal
fn proposition_length_equality_predicate<'a,const n: usize>() -> impl NaryPredicate<'a,n,OwnedPropositionInProof> {
    move |os: [OwnedPropositionInProof; n]| { 
        let mut iter = os.iter().map(|o| o.obj().as_slice() );
        let first_length = iter.next().expect("Cannot check length equality for zero propositions");
        for nth_length in iter {
            if nth_length != first_length { return false }
        }
        true
    }
}
/// Get a [Stringifier](NaryStringifier) which takes an [Propositions](OwnedPropositionInProof) and returns an error message saying that their lengths aren't equal
pub fn proposition_length_equality_stringifier<'a, const n: usize>() -> impl NaryStringifier<'a,n,OwnedPropositionInProof> {
    move |os: [OwnedPropositionInProof; n]| format!(
        "Proposition lengths expected to be equal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.path().to_string()
            + " -> " +
            &stringify_length(o.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Propositions](OwnedPropositionInProof) and returns an error message if their lengths aren't equal
pub fn proposition_length_equality_check<'a, const n: usize>() -> StringifiablePredicate<'a,n,OwnedPropositionInProof> { StringifiablePredicate::new(
    proposition_length_equality_predicate(),
    proposition_length_equality_stringifier(),
)}

/// Check that the provided [Propositions](OwnedPropositionInProof) have equal length, returning an error otherwise
pub fn assert_proposition_length_equality<'a,const n: usize, Rule:InferenceRule>(exprs: [OwnedPropositionInProof; n]) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_length_equality_check().evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
