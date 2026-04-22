use proof_calculus::{structures::propositions::{ParentOfAssumptions as _, ParentOfExplicitConclusions}, verification::validity::assertions::as_sized_slice};
use tbl_proof_calculus::{structures::{expressions::{TblExpression, compound::CompoundTblExpression}, proof_calculus_derived::{aliases::inferences::{TblInference, TblInferenceRule}, path_composites::{OwnedTblExpressionInInference, TblExpressionInInference}}}, proofs::assertions::{assert_expression_value, assert_fixed_length_expression_length_equality, assert_fixed_length_expression_value_equality, expression_as_sized_slice_in_inference, expression_into_slice}};

use crate::structures::atoms::PhilosophicaInferenceAtoms;

#[derive(Clone)]
pub enum UniversalSubstitutionError<C: CompoundTblExpression> {
    WrongAssumptionCount(usize),
    SubstitutionWrongSize(Option<usize>),
    SubtitutionWrongHead(TblExpression<C>),
    SubstitutionComparisonError(SubstitutionComparisonError<C>),
}

/// Verify that the assumptions and the conclusion form a valid instance of universal substitution ("for all x, P(x)" entails "P(y)" for any y)
pub fn verify_universal_substitution<'a,C: CompoundTblExpression, Rule: TblInferenceRule<C>>(inference: &TblInference<C,Rule>) -> Result<(), UniversalSubstitutionError<C>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = as_sized_slice(inference.get_located_explicit_conclusions())
        .expect("Inference objects must always return exactly one conclusion");
    // Throw an error if there is not exactly one assumption
    let [substitution] = as_sized_slice(inference.get_located_assumptions())
        .map_err(|e| UniversalSubstitutionError::WrongAssumptionCount(e.len()))?;

    // Throw an error if there are not three expressions in the conclusion
    let [substitution_head, expr_to_replace, expr_to_replace_within] = *expression_as_sized_slice_in_inference(&substitution)
        .map_err(|e| UniversalSubstitutionError::SubstitutionWrongSize(e.get_actual_length()))?;
    // Throw an error if the head of the substitution is incorrect
    assert_expression_value(&substitution_head, &PhilosophicaInferenceAtoms::UniversalQuantifier.into())
        .map_err(|e| UniversalSubstitutionError::SubtitutionWrongHead(e.into_expression()))?;
    // Check that remainder of the substitution is correct
    assert_substitution_comparison_validity(expr_to_replace_within, expr_to_replace.obj, conclusion.transform_path())
        .map_err(|e| UniversalSubstitutionError::SubstitutionComparisonError(e))?;
    
    // If none of the errors were triggered, then this step was successfully verified
    return Ok(())
}


#[derive(Clone)]
pub enum SubstitutionComparisonError<C: CompoundTblExpression> {
    NotATuple(OwnedTblExpressionInInference<C>),
    InequalComponentLength(OwnedTblExpressionInInference<C>, OwnedTblExpressionInInference<C>),
    InequalComponentValues(OwnedTblExpressionInInference<C>,OwnedTblExpressionInInference<C>)
}

/// Ensure that the verification_expr is what the find_expr would be, if all instances of the replace_expr were substituted for some value.
/// # Returns
/// - The value that the replace_expr was replaced with, if one can be found
/// - An error if such a replacement could not be verified to have taken place.
fn assert_substitution_comparison_validity<'a,C:CompoundTblExpression>(find_expr: TblExpressionInInference<C>, replace_expr: &TblExpression<C>, verify_expr: TblExpressionInInference<'_,C>) -> Result<Option<TblExpression<C>>,SubstitutionComparisonError<C>> {
    let mut ivm_paths = substitution_comparison_inner(find_expr, replace_expr, verify_expr)?.into_iter();
    if let Some(head_expr) = ivm_paths.next() {
        for tail_expr in ivm_paths {
            if let Err(e) = assert_fixed_length_expression_value_equality(&[&head_expr, &tail_expr])
                { return Err(SubstitutionComparisonError::InequalComponentValues(e.expressions[0].clone(), e.expressions[1].clone()))  }
        };
        Ok(Some(head_expr.obj.clone()))
    } else { Ok(None) }
}

fn substitution_comparison_inner<'a,C:CompoundTblExpression>(find_expr: TblExpressionInInference<'a,C>, replace_expr: &TblExpression<C>, verify_expr: TblExpressionInInference<'a,C>) -> Result<Box<[TblExpressionInInference<'a,C>]>, SubstitutionComparisonError<C>> {
    // If the find expression is the replace expression, then it must have been replaced with the verify expression so return that
    if find_expr.obj == replace_expr { return Ok(Box::new([verify_expr])) }
    if find_expr.obj == verify_expr.obj { return Ok(Box::new([])) }
    
    // Throw an error if the find expression and verify expressions are of different lengths (a substitution would not resolve this)
    if let Err(e) = assert_fixed_length_expression_length_equality(&[&find_expr, &verify_expr]) 
        { return Err(SubstitutionComparisonError::InequalComponentLength(e.expressions[0].clone(), e.expressions[1].clone())) }
    // Throw an error if find_expr or verify_expr is not a tuple
    let find_exprs = match expression_into_slice(find_expr) {
        Ok(exprs) => exprs,
        Err(e) => return Err(SubstitutionComparisonError::NotATuple(e.expression)),
    };
    let verify_exprs: Box<[TblExpressionInInference<'_,C>]> = match expression_into_slice(verify_expr) {
        Ok(exprs) => exprs,
        Err(e) => return Err(SubstitutionComparisonError::NotATuple(e.expression)),
    };
    
    // Recurse, performing substitution comparison on each expression within the sets of tuples
    let results = find_exprs.into_iter().zip(verify_exprs.into_iter())
        .map(|(expr1, expr2)| -> Result<Box<[TblExpressionInInference<'_,C>]>,SubstitutionComparisonError<C>> { substitution_comparison_inner(expr1, replace_expr, expr2) })
        .collect::<Result<Box<[_]>,SubstitutionComparisonError<C>>>()? // Throw an error if any of these contained errors
        .into_iter().flatten()
        .collect(); // Join together all of the 

    Ok(results)
}
