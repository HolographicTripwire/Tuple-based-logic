use tbl_structures::{atoms::BuiltInAtom, inference::Inference};

use crate::inference_rules::{error::ProofStepSpecificationError, StandardInferenceRule};
use crate::assertions::*;

/// Verify that the assumptions and the conclusion form a valid instance of conjunction introduction ("a" and "b" entails "a and b")
pub fn verify_conjunction_introduction(inference: &Inference<StandardInferenceRule>) -> Result<(),ProofStepSpecificationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = &*conclusions_as_sized_slice(inference)?;
    // Throw an error if there are not exactly two assumptions
    let [assumption_left, assumption_right] = &*assumptions_as_sized_slice(inference)?;
    // Throw an error if there are not three expressions in the conclusion
    let [conjunction_head, conjunction_left, conjunction_right] = &*expression_as_sized_slice(conclusion)?; 
    // Throw errors if the values of the inference components are incorrect
    assert_expression_value(conjunction_head, &BuiltInAtom::Conjunction.into())?;
    assert_value_match(assumption_left,conjunction_left)?;
    assert_value_match(assumption_right,conjunction_right)?;
    Ok(())
}
