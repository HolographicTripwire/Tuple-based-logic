use tbl_structures::{atoms::BuiltInAtom, propositions::{Proposition, Term}};

use crate::{inference_rules::TUPLE_OR_ERROR, ProofValidationError};

/// Verify that the assumptions and the conclusion form a valid instance of universal substitution ("for all x, P(x)" entails "P(y)" for any y)
pub fn verify_universal_substitution(assumptions: &Vec<Proposition>, conclusions: &Vec<Proposition>) -> Result<(), ProofValidationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = conclusions.as_slice() else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw an error if there is not one assumptions
    let [substitution] = assumptions.as_slice() else { return Err(ProofValidationError::InvalidStepSpecification) };

    // Throw an error if there are not three terms in the conclusion
    let [substitution_head, term_to_replace, term_to_replace_within] = TUPLE_OR_ERROR.prop_as_slice(substitution)? else { return Err(ProofValidationError::InvalidStepSpecification) };

    // Throw an error if the head of the substitution is incorrect
    if substitution_head != &BuiltInAtom::UniversalQuantifier.into() { return Err(ProofValidationError::InvalidStepSpecification) }
    // Check that remainder of the substitution is correct
    substitution_comparison(term_to_replace_within, term_to_replace, &conclusion.0)?;
    
    // If none of the errors were triggered, then this step was successfully verified
    return Ok(())
}


/// Ensure that the verification_term is what the find_term would be, if all instances of the replace_term were substituted for some value.
/// # Returns
/// - The value that the replace_term was replaced with, if one can be found
/// - An error if such a replacement could not be verified to have taken place.
fn substitution_comparison(find_term: &Term, replace_term: &Term, verify_term: &Term) -> Result<Option<Term>,ProofValidationError> {
    // If the find term is the replace term, then it must have been replaced with the verify term so return that
    if find_term == replace_term { return Ok(Some(verify_term.clone())) }
    
    // Throw an error if find_term or verify_term is not a tuple
    let find_terms = TUPLE_OR_ERROR.term_as_tuple(find_term)?;
    let verify_terms = TUPLE_OR_ERROR.term_as_tuple(verify_term)?;
    // Throw an error if the find term and verify terms are of different lengths (a substitution would not resolve this)
    if find_terms.len() != verify_terms.len() { return Err(ProofValidationError::InvalidStepSpecification) }
    
    // Recurse, performing substitution comparison on each term within the sets of tuples
    let results = find_terms.iter().zip(verify_terms.iter())
        .map(|(term1, term2)| -> Result<Option<Term>,ProofValidationError> { substitution_comparison(term1, replace_term, term2) });
    
    // Filter out unwanted results
    let mut filtered_results = Vec::new();
    for result in results { match result {
        Ok(Some(term)) => filtered_results.push(term),
        Ok(None) => {}, // Ignore all None values - as these indicate that there were no replacements made, nor errors encountered
        Err(err) => { return Err(err) }, // Throw an error if any of the substitution comparisons returned an error
    }}

    if let Some(first_result) = filtered_results.get(0).cloned() {
        // Throw an error if the same replacement was not made on each term within the set of tuples
        for result in filtered_results { if result != first_result { return Err(ProofValidationError::InvalidStepSpecification) } }
        // If all of the replacements are the same, then return what they were all replaced with
        return Ok(Some(first_result))
    // If no replacements were made, then return None; the find and verify term are the same
    } else { return Ok(None) }
}
