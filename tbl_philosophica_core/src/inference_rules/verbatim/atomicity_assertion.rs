use proof_calculus::{structures::propositions::{ParentOfAssumptions, ParentOfExplicitConclusions}, verification::validity::assertions::as_sized_slice};
use tbl_proof_calculus::{structures::{expressions::{TblExpression, compound::CompoundTblExpression}, proof_calculus_derived::aliases::inferences::{TblInference, TblInferenceRule}}, proofs::assertions::{assert_expression_atomicity, assert_expression_value, expression_as_sized_slice_in_inference}};

use crate::{inference_rules::verbatim::unwrap_verbatim_expression, structures::atoms::PhilosophicaInferenceAtoms};

#[derive(Clone)]
pub enum AtomicityAssertionError<C: CompoundTblExpression> {
    WrongAssumptionCount(usize),
    AtomicityWrongSize(Option<usize>),
    AtomicityWrongHead(TblExpression<C>),
    AtomicityParamNotVerbatim(TblExpression<C>),
    VerbatimComponentNotAtomic(TblExpression<C>)
}

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Atomic(Verbatim(a))" for any atom a)
pub fn verify_atomicity_assertion<C:CompoundTblExpression, Rule: TblInferenceRule<C>>(inference: &TblInference<C,Rule>) -> Result<(),AtomicityAssertionError<C>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = as_sized_slice(inference.get_located_explicit_conclusions())
        .expect("Inference objects must always return exactly one conclusion");
    // Throw ane error if the rule has any assumptions (this rule requires none)
    let [] = as_sized_slice(inference.get_located_assumptions())
        .map_err(|e| AtomicityAssertionError::WrongAssumptionCount(e.len()))?;
    
    // Throw an error if there are not three expressions in the conclusion
    let [atomicity_head, verbatim_expr] = *expression_as_sized_slice_in_inference(&conclusion)
        .map_err(|e| AtomicityAssertionError::AtomicityWrongSize(e.get_actual_length()))?;
    // Throw an error if the head of the conclusion is incorrect
    assert_expression_value(&atomicity_head, &PhilosophicaInferenceAtoms::Atomic.into())
        .map_err(|e| AtomicityAssertionError::AtomicityWrongHead(e.into_expression()))?;
    // Throw an error if the verbatim expression does not resolve to as Verbatim
    let verbatim_atom = unwrap_verbatim_expression(&verbatim_expr)
        .map_err(|e| AtomicityAssertionError::AtomicityParamNotVerbatim(e.expression().obj))?;
    // Throw an error if the verbatim atom is not actually an atom
    assert_expression_atomicity(&verbatim_atom, true)
        .map_err(|e| AtomicityAssertionError::VerbatimComponentNotAtomic(e.expression.obj))?;
    Ok(())
}