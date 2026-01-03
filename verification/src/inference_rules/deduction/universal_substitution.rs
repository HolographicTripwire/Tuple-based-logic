use tbl_structures::inference::{Inference, InferenceRule};
use tbl_structures::path_composites::{ExpressionInInference, OwnedExpressionInInference};
use tbl_structures::{atoms::BuiltInAtom, expressions::Expression};

use crate::assertions::*;

#[derive(Clone)]
pub enum UniversalSubstitutionError {
    WrongExplicitConclusionCount(usize),
    WrongAssumptionCount(usize),
    SubstitutionWrongSize(Option<usize>),
    SubtitutionWrongHead(Expression),
    SubstitutionComparisonError(SubstitutionComparisonError),
}

/// Verify that the assumptions and the conclusion form a valid instance of universal substitution ("for all x, P(x)" entails "P(y)" for any y)
pub fn verify_universal_substitution<'a,Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<(), UniversalSubstitutionError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(inference)
        .map_err(|e| UniversalSubstitutionError::WrongExplicitConclusionCount(e.get_actual_count()))?;
    // Throw an error if there is not exactly one assumption
    let [substitution] = *assumptions_as_sized_slice(&inference)
        .map_err(|e| UniversalSubstitutionError::WrongAssumptionCount(e.get_actual_count()))?;
    // Throw an error if there are not three expressions in the conclusion
    let [substitution_head, expr_to_replace, expr_to_replace_within] = *proposition_as_sized_slice(&substitution)
        .map_err(|e| UniversalSubstitutionError::SubstitutionWrongSize(e.get_actual_length()))?;
    // Throw an error if the head of the substitution is incorrect
    assert_expression_value(&substitution_head, &BuiltInAtom::UniversalQuantifier.into())
        .map_err(|e| UniversalSubstitutionError::SubtitutionWrongHead(e.into_expression()))?;
    // Check that remainder of the substitution is correct
    assert_substitution_comparison_validity(expr_to_replace_within, expr_to_replace.0.obj(), conclusion.into())
        .map_err(|e| UniversalSubstitutionError::SubstitutionComparisonError(e))?;
    
    // If none of the errors were triggered, then this step was successfully verified
    return Ok(())
}



pub enum SubstitutionComparisonError {
    NotATuple(OwnedExpressionInInference),
    InequalComponentLength(OwnedExpressionInInference, OwnedExpressionInInference),
    InequalComponentValues(OwnedExpressionInInference,OwnedExpressionInInference)
}

/// Ensure that the verification_expr is what the find_expr would be, if all instances of the replace_expr were substituted for some value.
/// # Returns
/// - The value that the replace_expr was replaced with, if one can be found
/// - An error if such a replacement could not be verified to have taken place.
fn assert_substitution_comparison_validity<'a>(find_expr: ExpressionInInference, replace_expr: &Expression, verify_expr: ExpressionInInference) -> Result<Option<Expression>,SubstitutionComparisonError> {
    let mut ivm_paths = substitution_comparison_inner(find_expr, replace_expr, verify_expr)?.into_iter();
    if let Some(head_expr) = ivm_paths.next() {
        for tail_expr in ivm_paths {
            if let Err(e) = assert_fixed_length_expression_value_equality(&[head_expr.clone(), tail_expr])
                { return Err(SubstitutionComparisonError::InequalComponentValues(e.expressions[0].clone(), e.expressions[1].clone()))  }
        };
        Ok(Some(head_expr.0.obj().clone()))
    } else { Ok(None) }
}

fn substitution_comparison_inner<'a>(find_expr: ExpressionInInference<'a>, replace_expr: &Expression, verify_expr: ExpressionInInference<'a>) -> Result<Vec<ExpressionInInference<'a>>, SubstitutionComparisonError> {
    // If the find expression is the replace expression, then it must have been replaced with the verify expression so return that
    if find_expr.0.obj() == replace_expr { return Ok(vec![verify_expr]) }
    if find_expr.0.obj() == verify_expr.0.obj() { return Ok(vec![]) }
    
    // Throw an error if the find expression and verify expressions are of different lengths (a substitution would not resolve this)
    if let Err(e) = assert_fixed_length_expression_length_equality(&[find_expr.clone(), verify_expr.clone()]) 
        { return Err(SubstitutionComparisonError::InequalComponentLength(e.expressions[0].clone(), e.expressions[1].clone())) }
    // Throw an error if find_expr or verify_expr is not a tuple
    let find_exprs = match expression_into_slice(find_expr) {
        Ok(exprs) => exprs,
        Err(e) => return Err(SubstitutionComparisonError::NotATuple(e.expression)),
    };
    let verify_exprs: Vec<ExpressionInInference<'_>> = match expression_into_slice(verify_expr) {
        Ok(exprs) => exprs,
        Err(e) => return Err(SubstitutionComparisonError::NotATuple(e.expression)),
    };
    
    // Recurse, performing substitution comparison on each expression within the sets of tuples
    let results = find_exprs.into_iter().zip(verify_exprs.into_iter())
        .map(|(expr1, expr2)| -> Result<Vec<ExpressionInInference>,SubstitutionComparisonError> { substitution_comparison_inner(expr1, replace_expr, expr2) })
        .collect::<Result<Vec<_>,SubstitutionComparisonError>>()? // Throw an error if any of these contained errors
        .into_iter().flatten()
        .collect(); // Join together all of the 

    Ok(results)
}
