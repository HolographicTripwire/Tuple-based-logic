use tbl_structures::{atoms::BuiltInAtom, propositions::{Proposition, Expression}};

use crate::{inference_rules::TUPLE_OR_ERROR, ProofValidationError};

/// Verify that the assumptions and the conclusion form a valid instance of universal substitution ("for all x, P(x)" entails "P(y)" for any y)
pub fn verify_universal_substitution(assumptions: &[Proposition], conclusions: &[Proposition]) -> Result<(), ProofValidationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = conclusions else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw an error if there is not one assumptions
    let [substitution] = assumptions else { return Err(ProofValidationError::InvalidStepSpecification) };

    // Throw an error if there are not three expressions in the conclusion
    let [substitution_head, expr_to_replace, expr_to_replace_within] = TUPLE_OR_ERROR.as_slice(substitution)? else { return Err(ProofValidationError::InvalidStepSpecification) };

    // Throw an error if the head of the substitution is incorrect
    if substitution_head != &BuiltInAtom::UniversalQuantifier.into() { return Err(ProofValidationError::InvalidStepSpecification) }
    // Check that remainder of the substitution is correct
    substitution_comparison(expr_to_replace_within, expr_to_replace, &conclusion)?;
    
    // If none of the errors were triggered, then this step was successfully verified
    return Ok(())
}


/// Ensure that the verification_expr is what the find_expr would be, if all instances of the replace_expr were substituted for some value.
/// # Returns
/// - The value that the replace_expr was replaced with, if one can be found
/// - An error if such a replacement could not be verified to have taken place.
fn substitution_comparison(find_expr: &Expression, replace_expr: &Expression, verify_expr: &Expression) -> Result<Option<Expression>,ProofValidationError> {
    // If the find expression is the replace expression, then it must have been replaced with the verify expression so return that
    if find_expr == replace_expr { return Ok(Some(verify_expr.clone())) }
    
    // Throw an error if find_expr or verify_expr is not a tuple
    let find_exprs = TUPLE_OR_ERROR.as_tuple(find_expr)?;
    let verify_exprs = TUPLE_OR_ERROR.as_tuple(verify_expr)?;
    // Throw an error if the find expression and verify expressions are of different lengths (a substitution would not resolve this)
    if find_exprs.len() != verify_exprs.len() { return Err(ProofValidationError::InvalidStepSpecification) }
    
    // Recurse, performing substitution comparison on each expression within the sets of tuples
    let results = find_exprs.iter().zip(verify_exprs.iter())
        .map(|(expr1, expr2)| -> Result<Option<Expression>,ProofValidationError> { substitution_comparison(expr1, replace_expr, expr2) });
    
    // Filter out unwanted results
    let mut filtered_results = Vec::new();
    for result in results { match result {
        Ok(Some(expr)) => filtered_results.push(expr),
        Ok(None) => {}, // Ignore all None values - as these indicate that there were no replacements made, nor errors encountered
        Err(err) => { return Err(err) }, // Throw an error if any of the substitution comparisons returned an error
    }}

    if let Some(first_result) = filtered_results.get(0).cloned() {
        // Throw an error if the same replacement was not made on each expression within the set of tuples
        for result in filtered_results { if result != first_result { return Err(ProofValidationError::InvalidStepSpecification) } }
        // If all of the replacements are the same, then return what they were all replaced with
        return Ok(Some(first_result))
    // If no replacements were made, then return None; the find and verify expression are the same
    } else { return Ok(None) }
}
