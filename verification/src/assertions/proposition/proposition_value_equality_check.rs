use tbl_structures::{path_composites::OwnedPropositionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

use crate::errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate};

/// Get a [Predicate](NaryPredicate) which takes n [Propositions](OwnedPropositionInProof) and checks if their values are equal
fn proposition_value_equality_predicate<'a,const N: usize>() -> impl NaryPredicate<'a,[OwnedPropositionInProof;N]> {
    move |os: [OwnedPropositionInProof; N]| { 
        let mut iter = os.iter().map(|o| o.0.obj() );
        let first_value = iter.next().expect("Cannot check value equality for zero propositions");
        for nth_value in iter {
            if nth_value != first_value { return false }
        }
        true
    }
}

/// Get a [Stringifier](NaryStringifier) which takes n [Propositions](OwnedPropositionInProof) and returns an error message saying that these proposition's value aren't equal
fn proposition_value_equality_stringifier<'a,const N:usize>(style: ExpressionStyle<'a>) -> impl NaryStringifier<'a,[OwnedPropositionInProof;N]> {
    move |os: [OwnedPropositionInProof; N]| format!(
        "Proposition values expected to be equal, but weren't; {values}",
        values = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Propositions](OwnedPropositionInProof) and returns an error message if these propositions values are not equal
pub fn proposition_value_equality_check<'a,const N: usize>(style: ExpressionStyle<'a>) -> StringifiablePredicate<'a,[OwnedPropositionInProof;N]> { StringifiablePredicate::new(
    proposition_value_equality_predicate(),
    proposition_value_equality_stringifier(style),
)}

/// Check that the provided [Propositions](OwnedPropositionInProof) have equal values, returning an error otherwise
pub fn assert_proposition_value_equality<'a,const N: usize>(expr: OwnedPropositionInProof, style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_value_equality_check(style).evaluate([expr])
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
