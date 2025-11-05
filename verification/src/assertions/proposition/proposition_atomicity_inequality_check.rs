use tbl_structures::{inference::InferenceRule, path_composites::OwnedPropositionInProof};

use crate::{assertions::expression::stringify_atomicity, errors::specification_error::{Assessor, AssessedStringifier, ProofStepSpecificationError, StringifiablePredicate}};

/// Get a [Predicate](NaryPredicate) which takes n [Propositions](OwnedPropositionInProof) and checks if their atomicities are equal
pub fn proposition_atomicity_inequality_predicate<'a>() -> impl Assessor<'a,[OwnedPropositionInProof; 2],()> {
    move |os: [OwnedPropositionInProof; 2]| { 
        if os[0].0.obj().as_atom().is_ok() != os[1].0.obj().as_atom().is_ok() { Ok(()) } else { Err(()) }
    }
}
/// Get a [Stringifier](NaryStringifier) which takes n [Propositions](OwnedPropositionInProof) and returns an error message saying that their atomicities aren't equal
pub fn proposition_atomicity_inequality_stringifier<'a>() -> impl AssessedStringifier<'a,[OwnedPropositionInProof;2],()> {
    move |os: [OwnedPropositionInProof; 2],_| format!(
        "Proposition atomicities expected to be inequal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            stringify_atomicity(o.0.obj().as_atom().is_ok())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Proposition](OwnedPropositionInProof) and returns an error message if their atomicities aren't equal
pub fn proposition_atomicity_inequality_check<'a>() -> StringifiablePredicate<'a,[OwnedPropositionInProof;2],()> { StringifiablePredicate::new(
    proposition_atomicity_inequality_predicate(),
    proposition_atomicity_inequality_stringifier(),
)}

/// Check that the provided [Propositions](OwnedPropositionInProof) have inequal atomicity, returning an error otherwise
pub fn assert_proposition_atomicity_inequality<'a,Rule:InferenceRule>(exprs: [OwnedPropositionInProof; 2]) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_atomicity_inequality_check::<'a>()
        .evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
