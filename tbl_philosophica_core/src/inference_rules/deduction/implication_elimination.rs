use proof_calculus::{structures::propositions::{ParentOfAssumptions as _, ParentOfExplicitConclusions}, verification::validity::assertions::as_sized_slice};
use tbl_proof_calculus::{structures::{expressions::{TblExpression, compound::CompoundTblExpression}, proof_calculus_derived::aliases::inferences::{TblInference, TblInferenceRule}}, proofs::assertions::{assert_expression_value, assert_expression_value_equality, expression_as_sized_slice_in_inference}};

use crate::structures::atoms::PhilosophicaInferenceAtoms;

#[derive(Clone)]
pub enum ImplicationEliminationError<C: CompoundTblExpression> {
    WrongAssumptionCount(usize),
    ImplicationWrongSize(Option<usize>),
    ImplicationWrongHead(TblExpression<C>),
    AntecedentInequal(TblExpression<C>,TblExpression<C>),
    ConsequentInequal(TblExpression<C>,TblExpression<C>)
}

/// Verify that the assumptions and the conclusion form a valid instance of implication elimination ("a" and "a implies b" entails "b")
pub fn verify_implication_elimination<'a,C: CompoundTblExpression, Rule: TblInferenceRule<C>>(inference: &TblInference<C,Rule>) -> Result<(),ImplicationEliminationError<C>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = as_sized_slice(inference.get_located_explicit_conclusions())
        .expect("Inference objects must always return exactly one conclusion");
    // Throw an error if there are not/ exactly two assumptions
    let [assumption_left, assumption_right] = as_sized_slice(inference.get_located_assumptions())
        .map_err(|e| ImplicationEliminationError::WrongAssumptionCount(e.len()))?;

    // Throw an error if the implication does not contain three expressions
    let [implication_head, antecedent, consequent] = *expression_as_sized_slice_in_inference(&assumption_right)
        .map_err(|e| ImplicationEliminationError::ImplicationWrongSize(e.get_actual_length()))?;
    // Throw errors if the values of the inference components are incorrect
    assert_expression_value(&implication_head, &PhilosophicaInferenceAtoms::Implication.into())
        .map_err(|e| ImplicationEliminationError::ImplicationWrongHead(e.into_expression()))?;
    assert_expression_value_equality(&[&antecedent, &assumption_left.transform_path()])
        .map_err(|e| ImplicationEliminationError::AntecedentInequal(e.expressions[0].obj.clone(), e.expressions[1].obj.clone()))?;
    assert_expression_value_equality(&[&consequent, &conclusion.transform_path()])
        .map_err(|e| ImplicationEliminationError::ConsequentInequal(e.expressions[0].obj.clone(), e.expressions[1].obj.clone()))?;
    Ok(())
}
