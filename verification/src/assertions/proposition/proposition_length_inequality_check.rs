use std::collections::HashSet;

use tbl_structures::{path_composites::OwnedPropositionInProof};

use crate::{assertions::expression::stringify_length, errors::{specification_error::{Assessor, AssessedErrorStringifier, ErrorStringifiableAssessor}, ProofStepSpecificationError}};

/// Get a [Predicate](NaryPredicate) which takes n [Propositions](OwnedPropositionInProof) and checks if their lengths aren't equal
pub fn proposition_length_inequality_predicate<'a,const N: usize>() -> impl Assessor<'a,[OwnedPropositionInProof;N],()> {
    move |os: [OwnedPropositionInProof; N]| { 
        let mut values = HashSet::new();
        for value in os.iter().map(|o| o.0.obj().as_slice() )
            { if !values.insert(value) { return Err(()); } }
        Ok(())
    }
}
/// Get a [Stringifier](NaryStringifier) which takes an [Propositions](OwnedPropositionInProof) and returns an error message saying that their lengths aren't inequal
pub fn proposition_length_inequality_stringifier<'a, const N: usize>() -> impl AssessedErrorStringifier<'a,[OwnedPropositionInProof;N],()> {
    move |os: [OwnedPropositionInProof; N],_| format!(
        "Proposition lengths expected to be inequal, but weren't; {atomicities}",
        atomicities = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            &stringify_length(o.0.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Propositions](OwnedPropositionInProof) and returns an error message if their lengths aren't inequal
pub fn proposition_length_inequality_check<'a, const N: usize>() -> ErrorStringifiableAssessor<'a,[OwnedPropositionInProof;N],()> { ErrorStringifiableAssessor::new(
    proposition_length_inequality_predicate(),
    proposition_length_inequality_stringifier(),
)}

/// Check that the provided [Propositions](OwnedPropositionInProof) have inequal length, returning an error otherwise
pub fn assert_proposition_length_inequality<'a,const N: usize>(exprs: [OwnedPropositionInProof; N]) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_length_inequality_check().evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
