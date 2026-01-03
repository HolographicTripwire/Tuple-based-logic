use tbl_structures::expressions::Expression;
use tbl_structures::inference::{Inference, InferenceRule};
use tbl_structures::path_composites::ExpressionInInference;
use tbl_structures::atoms::BuiltInAtom;

use crate::assertions::*;

#[derive(Clone)]
pub enum ConjunctionIntroductionError {
    WrongExplicitConclusionCount(usize),
    WrongAssumptionCount(usize),
    ConjunctionWrongSize(Option<usize>),
    ConjunctionWrongHead(Expression),
    LeftSideInequal(Expression,Expression),
    RightSideInequal(Expression,Expression)
}

/// Verify that the assumptions and the conclusion form a valid instance of conjunction introduction ("a" and "b" entails "a and b")
pub fn verify_conjunction_introduction<'a,Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<(),ConjunctionIntroductionError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(inference)
        .map_err(|e| ConjunctionIntroductionError::WrongExplicitConclusionCount(e.get_actual_count()))?;
    // Throw an error if there are not/ exactly two assumptions
    let [assumption_left, assumption_right] = *assumptions_as_sized_slice(&inference)
        .map_err(|e| ConjunctionIntroductionError::WrongAssumptionCount(e.get_actual_count()))?;
    // Throw an error if there are not three expressions in the conclusion
    let [conjunction_head, conjunction_left, conjunction_right] = *proposition_as_sized_slice(&conclusion)
        .map_err(|e| ConjunctionIntroductionError::ConjunctionWrongSize(e.get_actual_length()))?;
    // Throw errors if the values of the inference components are incorrect
    assert_expression_value(&conjunction_head, &BuiltInAtom::Conjunction.into())
        .map_err(|e| ConjunctionIntroductionError::ConjunctionWrongHead(e.into_expression()))?;
    assert_fixed_length_expression_value_equality(&[ExpressionInInference(assumption_left.0.replace_path(|p| p.into())),conjunction_left])
        .map_err(|e| ConjunctionIntroductionError::LeftSideInequal(e.expressions[0].0.obj().clone(), e.expressions[1].0.obj().clone()))?;
    assert_fixed_length_expression_value_equality(&[ExpressionInInference(assumption_right.0.replace_path(|p| p.into())),conjunction_right])
        .map_err(|e| ConjunctionIntroductionError::RightSideInequal(e.expressions[0].0.obj().clone(), e.expressions[1].0.obj().clone()))?;
    Ok(())
}
