use tbl_structures::{atoms::BuiltInAtom, proof::InferenceInProof};
use tbl_textualization::structures::expressions::ExpressionStyle;

use crate::{assertions::*, errors::specification_error::ProofStepSpecificationError, inference_rules::StandardInferenceRule};

use super::resolve_verbatim;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Not (Verbatim(a) = Verbatim(b))" for any two different atoms a and b)
pub fn verify_atom_differentiation<'a>(inference: &InferenceInProof<StandardInferenceRule>, style: ExpressionStyle<'a>) -> Result<(),ProofStepSpecificationError<'a>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(&inference)?;
    // Throw ane error if the rule has any assumptions (this rule requires none)
    let [] = *explicit_conclusions_as_sized_slice(&inference)?;
    
    // Throw an error if there are not two expressions in the conclusion
    let [negation_head, identity] = *proposition_as_sized_slice(&conclusion)?;
    // Throw an error if the head of the conclusion is incorrect
    assert_expression_value(negation_head, BuiltInAtom::Negation.into(), style.clone())?;
    
    // Throw an error if there are not three expressions in the identity
    let [identity_head, identity_left, identity_right] = *expression_as_sized_slice(&identity)?;
    // Throw an error if the head of the identity is incorrect
    assert_expression_value(identity_head, BuiltInAtom::Identity.into(), style.clone())?;

    // Throw an error if either of the verbatim expressions do not resolve as Verbatim
    let left_verbatim_atom = resolve_verbatim(identity_left, style.clone())?;
    let right_verbatim_atom = resolve_verbatim(identity_right, style.clone())?;

    // Throw an error if either of the verbatim atoms is not actually an atom
    assert_expression_atomicity(left_verbatim_atom.clone(), true)?;
    assert_expression_atomicity(right_verbatim_atom.clone(), true)?;
    
    // Throw an error if the atoms aree actually identical
    assert_expression_value_inequality([left_verbatim_atom, right_verbatim_atom], style)?;

    Ok(())
}
