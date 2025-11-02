use tbl_structures::path_composites::OwnedExpressionInProof;
use tbl_structures::{atoms::BuiltInAtom, expressions::Expression, proof::OwnedInferenceInProof};
use tbl_textualization::structures::expressions::ExpressionStyle;

use crate::inference_rules::{ProofStepSpecificationError, StandardInferenceRule};
use crate::assertions::*;

/// Verify that the assumptions and the conclusion form a valid instance of universal substitution ("for all x, P(x)" entails "P(y)" for any y)
pub fn verify_universal_substitution<'a>(inference: &'a OwnedInferenceInProof<StandardInferenceRule>, style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(inference)?;
    // Throw an error if there is not exactly one assumption
    let [substitution] = &*assumptions_as_sized_slice(inference)?;
    // Throw an error if there are not three expressions in the conclusion
    let [substitution_head, expr_to_replace, expr_to_replace_within] = *proposition_as_sized_slice(substitution)?;

    // Throw an error if the head of the substitution is incorrect
    assert_expression_value(substitution_head, BuiltInAtom::UniversalQuantifier.into(), style.clone())?;
    // Check that remainder of the substitution is correct
    substitution_comparison(expr_to_replace_within, expr_to_replace.0.obj(), OwnedExpressionInProof(conclusion.0.replace_path(|p| p.into())), style);
    
    // If none of the errors were triggered, then this step was successfully verified
    return Ok(())
}


/// Ensure that the verification_expr is what the find_expr would be, if all instances of the replace_expr were substituted for some value.
/// # Returns
/// - The value that the replace_expr was replaced with, if one can be found
/// - An error if such a replacement could not be verified to have taken place.
fn substitution_comparison<'a>(find_expr: OwnedExpressionInProof, replace_expr: &Expression, verify_expr: OwnedExpressionInProof, style: ExpressionStyle<'a>) -> Result<Option<Expression>,ProofStepSpecificationError<'a>> {
    let mut ivm_paths = substitution_comparison_inner(find_expr, replace_expr, verify_expr)?.into_iter();
    if let Some(head) = ivm_paths.next() {
        for tail_expr in ivm_paths { assert_expression_value_equality([head.clone(), tail_expr], style.clone())? };
        Ok(Some(head.0.obj().clone()))
    } else { Ok(None) }
}

fn substitution_comparison_inner<'a>(find_expr: OwnedExpressionInProof, replace_expr: &Expression, verify_expr: OwnedExpressionInProof) -> Result<Vec<OwnedExpressionInProof>,ProofStepSpecificationError<'a>> {
    // If the find expression is the replace expression, then it must have been replaced with the verify expression so return that
    if find_expr.0.obj() == replace_expr { return Ok(vec![verify_expr.clone()]) }
    if find_expr.0.obj() == verify_expr.0.obj() { return Ok(vec![]) }
    
    // Throw an error if find_expr or verify_expr is not a tuple
    let find_exprs = expression_as_slice(&find_expr)?;
    let verify_exprs = expression_as_slice(&find_expr)?;
    // Throw an error if the find expression and verify expressions are of different lengths (a substitution would not resolve this)
    assert_expression_length_equality([find_expr, verify_expr])?;
    
    // Recurse, performing substitution comparison on each expression within the sets of tuples
    let results = find_exprs.into_iter().zip(verify_exprs.into_iter())
        .map(|(expr1, expr2)| -> Result<Vec<OwnedExpressionInProof>,ProofStepSpecificationError> { substitution_comparison_inner(expr1, replace_expr, expr2) })
        .collect::<Result<Vec<_>,ProofStepSpecificationError>>()? // Throw an error if any of these contained errors
        .into_iter().flatten().collect(); // Join together all of the 

    Ok(results)
}
