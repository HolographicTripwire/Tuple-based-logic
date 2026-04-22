use proof_calculus::{structures::propositions::{ParentOfAssumptions, ParentOfExplicitConclusions}, verification::validity::assertions::as_sized_slice};
use tbl_proof_calculus::{structures::{expressions::{TblExpression, compound::CompoundTblExpression}, proof_calculus_derived::aliases::inferences::{TblInference, TblInferenceRule}}, proofs::assertions::{assert_expression_value, assert_fixed_length_expression_value_equality, expression_as_sized_slice_in_inference}};

use crate::structures::atoms::PhilosophicaInferenceAtoms;

#[derive(Clone)]
pub enum ConjunctionIntroductionError<C: CompoundTblExpression> {
    WrongAssumptionCount(usize),
    ConjunctionWrongSize(Option<usize>),
    ConjunctionWrongHead(TblExpression<C>),
    LeftSideInequal(TblExpression<C>,TblExpression<C>),
    RightSideInequal(TblExpression<C>,TblExpression<C>)
}

/// Verify that the assumptions and the conclusion form a valid instance of conjunction introduction ("a" and "b" entails "a and b")
pub fn validate_conjunction_introduction<'a,C: CompoundTblExpression, Rule: TblInferenceRule<C>>(inference: &TblInference<C,Rule>) -> Result<(),ConjunctionIntroductionError<C>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = as_sized_slice(inference.get_located_explicit_conclusions())
        .expect("Inference objects must always return exactly one conclusion");
    // Throw an error if there are not/ exactly two assumptions
    let [assumption_left, assumption_right] = as_sized_slice(inference.get_located_assumptions())
        .map_err(|e| ConjunctionIntroductionError::WrongAssumptionCount(e.len()))?;

    // Throw an error if there are not three expressions in the conclusion
    let [conjunction_head, conjunction_left, conjunction_right] = *expression_as_sized_slice_in_inference(&conclusion)
        .map_err(|e| ConjunctionIntroductionError::ConjunctionWrongSize(e.get_actual_length()))?;
    // Throw errors if the values of the inference components are incorrect
    assert_expression_value(&conjunction_head, &PhilosophicaInferenceAtoms::Conjunction.into())
        .map_err(|e| ConjunctionIntroductionError::ConjunctionWrongHead(e.into_expression()))?;
    assert_fixed_length_expression_value_equality(&[&assumption_left.transform_path(), &conjunction_left])
        .map_err(|e| ConjunctionIntroductionError::LeftSideInequal(e.expressions[0].obj.clone(), e.expressions[1].obj.clone()))?;
    assert_fixed_length_expression_value_equality(&[&assumption_right.transform_path(), &conjunction_right])
        .map_err(|e| ConjunctionIntroductionError::RightSideInequal(e.expressions[0].obj.clone(), e.expressions[1].obj.clone()))?;
    Ok(())
}
