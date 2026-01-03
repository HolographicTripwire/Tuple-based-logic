use tbl_structures::expressions::Expression;
use tbl_structures::inference::{Inference, InferenceRule};
use tbl_structures::atoms::BuiltInAtom;

use crate::assertions::*;

#[derive(Clone)]
pub enum ImplicationEliminationError {
    WrongExplicitConclusionCount(usize),
    WrongAssumptionCount(usize),
    ImplicationWrongSize(Option<usize>),
    ImplicationWrongHead(Expression),
    AntecedentInequal(Expression,Expression),
    ConsequentInequal(Expression,Expression)
}

/// Verify that the assumptions and the conclusion form a valid instance of implication elimination ("a" and "a implies b" entails "b")
pub fn verify_implication_elimination<'a,Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<(),ImplicationEliminationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(inference)
        .map_err(|e| ImplicationEliminationError::WrongExplicitConclusionCount(e.get_actual_count()))?;
    // Throw an error if there are not exactly two assumptions
    let [assumption_left, assumption_right] = *assumptions_as_sized_slice(inference)
        .map_err(|e| ImplicationEliminationError::WrongAssumptionCount(e.get_actual_count()))?;
    // Throw an error if the implication does not contain three expressions
    let [implication_head, antecedent, consequent] = *proposition_as_sized_slice(&assumption_right)
        .map_err(|e| ImplicationEliminationError::ImplicationWrongSize(e.get_actual_length()))?;
    // Throw errors if the values of the inference components are incorrect
    assert_expression_value(&implication_head, &BuiltInAtom::Implication.into())
        .map_err(|e| ImplicationEliminationError::ImplicationWrongHead(e.into_expression()))?;
    assert_expression_value_equality(&[antecedent, assumption_left.into()])
        .map_err(|e| ImplicationEliminationError::AntecedentInequal(e.expressions[0].0.obj().clone(), e.expressions[1].0.obj().clone()))?;
    assert_expression_value_equality(&[consequent, conclusion.into()])
        .map_err(|e| ImplicationEliminationError::ConsequentInequal(e.expressions[0].0.obj().clone(), e.expressions[1].0.obj().clone()))?;
    Ok(())
}
