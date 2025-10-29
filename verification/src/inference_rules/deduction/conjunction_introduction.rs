use tbl_structures::proof::OwnedInferenceInProof;
use tbl_structures::atoms::BuiltInAtom;
use tbl_textualization::structures::expressions::ExpressionStyle;

use crate::errors::specification_error::ProofStepSpecificationError;
use crate::assertions::*;
use crate::inference_rules::StandardInferenceRule;

/// Verify that the assumptions and the conclusion form a valid instance of conjunction introduction ("a" and "b" entails "a and b")
pub fn verify_conjunction_introduction<'a>(inference: &'a OwnedInferenceInProof<StandardInferenceRule>, style: ExpressionStyle<'a>) -> Result<(),ProofStepSpecificationError<'a>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = &*explicit_conclusions_as_sized_slice(inference)?;
    // Throw an error if there are not exactly two assumptions
    let [assumption_left, assumption_right] = *assumptions_as_sized_slice(inference)?;
    // Throw an error if there are not three expressions in the conclusion
    let [conjunction_head, conjunction_left, conjunction_right] = *proposition_as_sized_slice(conclusion)?; 
    // Throw errors if the values of the inference components are incorrect
    assert_expression_value(conjunction_head, BuiltInAtom::Conjunction.into(), style.clone())?;
    assert_expression_value_equality([assumption_left.replace_path(|p| p.into()),conjunction_left], style.clone())?;
    assert_expression_value_equality([assumption_right.replace_path(|p| p.into()),conjunction_right], style)?;
    Ok(())
}
