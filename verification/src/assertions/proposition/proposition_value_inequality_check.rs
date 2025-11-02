use std::collections::HashSet;

use tbl_structures::{path_composites::OwnedPropositionInProof};
use tbl_textualization::{helpers::styles::Style, structures::expressions::ExpressionStyle};

use crate::errors::specification_error::{NaryPredicate, NaryStringifier, ProofStepSpecificationError, StringifiablePredicate};

/// Get a [Predicate](NaryPredicate) which takes n [Propositions](OwnedPropositionInProof) and checks if their values are inequal
pub fn proposition_value_inequality_predicate<'a,const n: usize>() -> impl NaryPredicate<'a,n,OwnedPropositionInProof> {
    move |os: [OwnedPropositionInProof; n]| { 
        let mut values = HashSet::new();
        for value in os.iter().map(|o| o.0.obj() )
            { if !values.insert(value) { return false; } }
        true
    }
}

/// Get a [Stringifier](NaryStringifier) which takes n [Propositions](OwnedPropositionInProof) and returns an error message saying that these proposition's value aren't inequal
pub fn proposition_value_inequality_stringifier<'a,const n:usize>(style: ExpressionStyle<'a>) -> impl NaryStringifier<'a,n,OwnedPropositionInProof> {
    move |os: [OwnedPropositionInProof; n]| format!(
        "Proposition values expected to be inequal, but weren't; {values}",
        values = os.map(|o| 
            o.0.path().to_string()
            + " -> " +
            &style.stringify(o.0.obj())
        ).join(", ")
    )
}
/// Get a [Checker](StringifiablePredicate) which takes n [Propositions](OwnedPropositionInProof) and returns an error message if these propositions values aren't inequal
pub fn proposition_value_inequality_check<'a,const n: usize>(style: ExpressionStyle<'a>) -> StringifiablePredicate<'a,n,OwnedPropositionInProof> { StringifiablePredicate::new(
    proposition_value_inequality_predicate(),
    proposition_value_inequality_stringifier(style),
)}

/// Check that the provided [Propositions](OwnedPropositionInProof) have inequal values, returning an error otherwise
pub fn assert_proposition_value_inequality<'a,const n: usize>(exprs: [OwnedPropositionInProof; n], style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    proposition_value_inequality_check(style).evaluate(exprs)
        .map_err(|assertion| ProofStepSpecificationError::from_inner(assertion))
}
