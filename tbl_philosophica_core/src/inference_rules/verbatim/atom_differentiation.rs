use proof_calculus::{structures::propositions::{ParentOfAssumptions, ParentOfExplicitConclusions}, verification::validity::assertions::as_sized_slice};
use tbl_proof_calculus::{expressions::TblExpressionLength, proofs::assertions::{assert_expression_atomicity, assert_expression_value, assert_fixed_length_expression_value_inequality, expression_as_sized_slice_in_inference}, structures::{expressions::{TblExpression, compound::CompoundTblExpression}, proof_calculus_derived::aliases::inferences::{TblInference, TblInferenceRule}}};

use crate::{inference_rules::verbatim::unwrap_verbatim_expression, structures::atoms::PhilosophicaInferenceAtoms};


#[derive(Clone)]
pub enum AtomDifferentiationError<C: CompoundTblExpression> {
    WrongAssumptionCount(usize),
    NegationWrongLength(TblExpressionLength),
    NegationWrongHead(TblExpression<C>),
    IdentityWrongLength(TblExpressionLength),
    IdentityWrongHead(TblExpression<C>),
    IdentityLeftNotVerbatim(TblExpression<C>),
    IdentityRightNotVerbatim(TblExpression<C>),
    VerbatimLeftNonunitary(TblExpression<C>),
    VerbatimRightNonunitary(TblExpression<C>),
    LeftAndRightEqual(TblExpression<C>),
}

/// Verify that the assumptions and the conclusion form a valid instance of atom idfferentiation assertion ("Not (Verbatim(a) = Verbatim(b))" for any two different atoms a and b)
pub fn verify_atom_differentiation<C:CompoundTblExpression, Rule: TblInferenceRule<C>>(inference: &TblInference<C,Rule>) -> Result<(),AtomDifferentiationError<C>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = as_sized_slice(inference.get_located_explicit_conclusions())
        .expect("Inference objects must always return exactly one conclusion");
    // Throw ane error if the rule has any assumptions (this rule requires none)
    let [] = as_sized_slice(inference.get_located_assumptions())
        .map_err(|e| AtomDifferentiationError::WrongAssumptionCount(e.len()))?;
    
    // Throw an error if there are not two expressions in the conclusion
    let [negation_head, identity] = *expression_as_sized_slice_in_inference(&conclusion)
        .map_err(|e| AtomDifferentiationError::NegationWrongLength(e.get_actual_length()))?;
    // Throw an error if the head of the conclusion is incorrect
    assert_expression_value(&negation_head, &PhilosophicaInferenceAtoms::Negation.into())
        .map_err(|e| AtomDifferentiationError::NegationWrongHead(e.into_expression()))?;
    
    // Throw an error if there are not three expressions in the identity
    let [identity_head, identity_left, identity_right] = *expression_as_sized_slice_in_inference(&identity)
        .map_err(|e| AtomDifferentiationError::IdentityWrongLength(e.get_actual_length()))?;
    // Throw an error if the head of the identity is incorrect
    assert_expression_value(&identity_head, &PhilosophicaInferenceAtoms::Identity.into())
        .map_err(|e| AtomDifferentiationError::IdentityWrongHead(e.into_expression()))?;

    // Throw an error if either of the verbatim expressions do not resolve as Verbatim
    let left_verbatim_atom = unwrap_verbatim_expression(&identity_left)
        .map_err(|e| AtomDifferentiationError::IdentityLeftNotVerbatim(e.expression().obj))?;
    let right_verbatim_atom = unwrap_verbatim_expression(&identity_right)
        .map_err(|e| AtomDifferentiationError::IdentityRightNotVerbatim(e.expression().obj))?;

    // Throw an error if either of the verbatim atoms is not actually an atom
    assert_expression_atomicity(&left_verbatim_atom, true)
        .map_err(|e| AtomDifferentiationError::VerbatimLeftNonunitary(e.expression.obj))?;
    assert_expression_atomicity(&right_verbatim_atom, true)
        .map_err(|e| AtomDifferentiationError::VerbatimRightNonunitary(e.expression.obj))?;
    
    // Throw an error if the atoms aree actually identical
    assert_fixed_length_expression_value_inequality(&[&left_verbatim_atom, &right_verbatim_atom])
        .map_err(|e| AtomDifferentiationError::LeftAndRightEqual(e.expressions[0].clone().obj))?;

    Ok(())
}
