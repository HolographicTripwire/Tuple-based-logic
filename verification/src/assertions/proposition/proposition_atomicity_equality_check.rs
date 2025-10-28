use tbl_structures::{inference::InferenceRule, path_composites::OwnedPropositionInProof};

use crate::{assertions::expression::stringify_atomicity, errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate}};

/// Get a [Predicate](NaryPredicate) which takes n [Propositions](OwnedPropositionInProof) and checks if their atomicities are equal
fn proposition_atomicity_equality_predicate<'a,const n: usize>() -> impl NaryPredicate<'a,n,OwnedPropositionInProof> {
    move |os: [OwnedPropositionInProof; n]| { 
        let mut iter = os.iter().map(|o| o.obj().as_atom().is_ok());
        let first_atomicity = iter.next().expect("Cannot check atomicity equality for zero propositions");
        for nth_atomicity in iter {
            if nth_atomicity != first_atomicity { return false }
        }
        true
    }
}
/// Get a [Stringifier](NaryStringifier) which takes n [Propositions](OwnedPropositionInProof) and returns an error message saying that their atomicities aren't equal
fn proposition_atomicity_equality_stringifier<'a,const n: usize>() -> impl NaryStringifier<'a,n,OwnedPropositionInProof> {
    move |os: [OwnedPropositionInProof; n]| format!(
        "Proposition atomicities expected to be equal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.path().to_string()
            + " -> " +
            stringify_atomicity(o.obj().as_atom().is_ok())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Proposition](OwnedPropositionInProof) and returns an error message if their atomicities aren't equal
pub fn proposition_atomicity_equality_check<'a,const n: usize>() -> StringifiablePredicate<'a,n,OwnedPropositionInProof> { StringifiablePredicate::new(
    proposition_atomicity_equality_predicate(),
    proposition_atomicity_equality_stringifier(),
)}

/// Check that the provided [Proposition](OwnedPropositionInProof) has an atomicity equal to atomicty_expected, returning an error otherwise
pub fn assert_proposition_atomicity_equality<'a,const n: usize, Rule:InferenceRule>(exprs: [OwnedPropositionInProof; n]) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_atomicity_equality_check::<'a>()
        .evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
