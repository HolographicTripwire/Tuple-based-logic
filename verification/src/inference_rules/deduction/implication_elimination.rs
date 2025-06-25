use tbl_structures::{atoms::BuiltInAtom, inference::Inference};

use crate::inference_rules::{error::ProofStepSpecificationError, StandardInferenceRule};
use crate::assertions::*;


/// Verify that the assumptions and the conclusion form a valid instance of implication elimination ("a" and "a implies b" entails "b")
pub fn verify_implication_elimination(inference: &Inference<StandardInferenceRule>) -> Result<(), ProofStepSpecificationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = &*conclusions_as_sized_slice(inference)?;
    // Throw an error if there are not two assumptions
    let [prior, implication] = &*assumptions_as_sized_slice(inference)?;
    // Throw an error if the implication does not contain three expressions
    let [implication_head, antecedent, consequent] = &*expression_as_sized_slice(implication)?;
    // Throw errors if the values of the inference components are incorrect
    assert_expression_value(implication_head, &BuiltInAtom::Implication.into())?;
    assert_value_match(antecedent, prior)?;
    assert_value_match(consequent, conclusion)?;
    Ok(())
}
