use tbl_structures::{atoms::BuiltInAtom, expressions::Expression, inference::{Inference, InferenceRule}};

use crate::assertions::*;

#[derive(Clone)]
pub enum AtomDifferentiationError {
    WrongExplicitConclusionCount(usize),
    WrongAssumptionCount(usize),
    NegationWrongSize(Option<usize>),
    NegationWrongHead(Expression),
    IdentityWrongSize(Option<usize>),
    IdentityWrongHead(Expression),
    IdentityLeftNotVerbatim(Expression),
    IdentityRightNotVerbatim(Expression),
    VerbatimLeftInatomic(Expression),
    VerbatimRightInatomic(Expression),
    LeftAndRightEqual(Expression),
}

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Not (Verbatim(a) = Verbatim(b))" for any two different atoms a and b)
pub fn verify_atom_differentiation<Rule: InferenceRule>(inference: &Inference<Rule>) -> Result<(),AtomDifferentiationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(&inference)
        .map_err(|e| AtomDifferentiationError::WrongExplicitConclusionCount(e.get_actual_count()))?;
    // Throw ane error if the rule has any assumptions (this rule requires none)
    let [] = *explicit_conclusions_as_sized_slice(&inference)
        .map_err(|e| AtomDifferentiationError::WrongAssumptionCount(e.get_actual_count()))?;
    
    // Throw an error if there are not two expressions in the conclusion
    let [negation_head, identity] = *proposition_as_sized_slice(&conclusion)
        .map_err(|e| AtomDifferentiationError::NegationWrongSize(e.get_actual_length()))?;
    // Throw an error if the head of the conclusion is incorrect
    assert_expression_value(&negation_head, &BuiltInAtom::Negation.into())
        .map_err(|e| AtomDifferentiationError::NegationWrongHead(e.into_expression()))?;
    
    // Throw an error if there are not three expressions in the identity
    let [identity_head, identity_left, identity_right] = *expression_as_sized_slice(&identity)
        .map_err(|e| AtomDifferentiationError::IdentityWrongSize(e.get_actual_length()))?;
    // Throw an error if the head of the identity is incorrect
    assert_expression_value(&identity_head, &BuiltInAtom::Identity.into())
        .map_err(|e| AtomDifferentiationError::IdentityWrongHead(e.into_expression()))?;

    // Throw an error if either of the verbatim expressions do not resolve as Verbatim
    let left_verbatim_atom = resolve_verbatim(identity_left)
        .map_err(|e| AtomDifferentiationError::IdentityLeftNotVerbatim(e.value))?;
    let right_verbatim_atom = resolve_verbatim(identity_right)
        .map_err(|e| AtomDifferentiationError::IdentityRightNotVerbatim(e.value))?;

    // Throw an error if either of the verbatim atoms is not actually an atom
    assert_expression_atomicity(&left_verbatim_atom, true)
        .map_err(|e| AtomDifferentiationError::VerbatimLeftInatomic(e.expression.0.into_obj_and_path().0))?;
    assert_expression_atomicity(&right_verbatim_atom, true)
        .map_err(|e| AtomDifferentiationError::VerbatimRightInatomic(e.expression.0.into_obj_and_path().0))?;
    
    // Throw an error if the atoms aree actually identical
    assert_fixed_length_expression_value_inequality(&[left_verbatim_atom, right_verbatim_atom])
        .map_err(|e| AtomDifferentiationError::LeftAndRightEqual(e.expressions[0].0.clone().into_obj_and_path().0))?;

    Ok(())
}
