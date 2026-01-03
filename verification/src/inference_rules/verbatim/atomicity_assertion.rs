use tbl_structures::{atoms::BuiltInAtom, expressions::Expression, inference::{Inference, InferenceRule}};

use crate::{assertions::*, inference_rules::verbatim::FunctionUnwrapError};

use super::resolve_verbatim;

#[derive(Clone)]
pub enum AtomicityAssertionError {
    WrongExplicitConclusionCount(usize),
    WrongAssumptionCount(usize),
    AtomicityWrongSize(Option<usize>),
    AtomicityWrongHead(Expression),
    AtomicityParamNotVerbatim(FunctionUnwrapError),
    VerbatimComponentNotAtomic(Expression)
}

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Atomic(Verbatim(a))" for any atom a)
pub fn verify_atomicity_assertion<Rule: InferenceRule> (inference: &Inference<Rule>) -> Result<(),AtomicityAssertionError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(&inference)
        .map_err(|e| AtomicityAssertionError::WrongExplicitConclusionCount(e.get_actual_count()))?;
    // Throw ane error if the rule has any assumptions (this rule requires none)
    let [] = *explicit_conclusions_as_sized_slice(&inference)
        .map_err(|e| AtomicityAssertionError::WrongAssumptionCount(e.get_actual_count()))?;
    
    // Throw an error if there are not three expressions in the conclusion
    let [atomicity_head, verbatim_expr] = *proposition_as_sized_slice(&conclusion)
        .map_err(|e| AtomicityAssertionError::AtomicityWrongSize(e.get_actual_length()))?;
    // Throw an error if the head of the conclusion is incorrect
    assert_expression_value(&atomicity_head, &BuiltInAtom::Atomic.into())
        .map_err(|e| AtomicityAssertionError::AtomicityWrongHead(e.into_expression()))?;
    // Throw an error if the verbatim expression does not resolve to as Verbatim
    let verbatim_atom = resolve_verbatim(verbatim_expr)
        .map_err(|e| AtomicityAssertionError::AtomicityParamNotVerbatim(e))?;
    // Throw an error if the verbatim atom is not actually an atom
    assert_expression_atomicity(&verbatim_atom, true)
        .map_err(|e| AtomicityAssertionError::VerbatimComponentNotAtomic(e.expression.0.into_obj_and_path().0))?;
    Ok(())
}
t6gy7