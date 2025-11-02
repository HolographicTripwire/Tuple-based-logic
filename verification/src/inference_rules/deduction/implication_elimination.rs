use tbl_structures::path_composites::OwnedExpressionInProof;
use tbl_structures::proof::OwnedInferenceInProof;
use tbl_structures::atoms::BuiltInAtom;
use tbl_textualization::structures::expressions::ExpressionStyle;

use crate::errors::specification_error::ProofStepSpecificationError;
use crate::assertions::*;
use crate::inference_rules::StandardInferenceRule;


/// Verify that the assumptions and the conclusion form a valid instance of implication elimination ("a" and "a implies b" entails "b")
pub fn verify_implication_elimination<'a>(inference: &'a OwnedInferenceInProof<StandardInferenceRule>, style: ExpressionStyle<'a>) -> Result<(), ProofStepSpecificationError<'a>> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = *explicit_conclusions_as_sized_slice(inference)?;
    // Throw an error if there are not exactly two assumptions
    let [prior, implication] = *assumptions_as_sized_slice(inference)?;
    // Throw an error if the implication does not contain three expressions
    let [implication_head, antecedent, consequent] = *proposition_as_sized_slice(&implication)?;
    // Throw errors if the values of the inference components are incorrect
    assert_expression_value(implication_head, BuiltInAtom::Implication.into(), style.clone())?;
    assert_expression_value_equality([antecedent, OwnedExpressionInProof(prior.0.replace_path(|p| p.into()))], style.clone())?;
    assert_expression_value_equality([consequent, OwnedExpressionInProof(conclusion.0.replace_path(|p| p.into()))], style)?;
    Ok(())
}
