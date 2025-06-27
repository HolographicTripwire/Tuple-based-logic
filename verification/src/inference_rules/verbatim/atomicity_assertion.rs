use tbl_structures::{atoms::BuiltInAtom, inference::Inference};

use crate::inference_rules::{assertions::{assert_expression_atomicity, assert_expression_value, assumptions_as_sized_slice, conclusions_as_sized_slice, expression_as_sized_slice}, ProofStepSpecificationError, StandardInferenceRule};

use super::resolve_verbatim;

/// Verify that the assumptions and the conclusion form a valid instance of atomicity assertion ("Atomic(Verbatim(a))" for any atom a)
pub fn verify_atomicity_assertion(inference: &Inference<StandardInferenceRule>) -> Result<(),ProofStepSpecificationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = &*conclusions_as_sized_slice(inference)?;
    // Throw ane error if the rule has any assumptions (this rule requires none)
    let [] = &*assumptions_as_sized_slice(inference)?;
    
    // Throw an error if there are not three expressions in the conclusion
    let [atomicity_head, verbatim_expr] = &*expression_as_sized_slice(conclusion)?;
    // Throw an error if the head of the conclusion is incorrect
    assert_expression_value(atomicity_head, &BuiltInAtom::Atomic.into())?;
    // Throw an error if the verbatim expression does not resolve to as Verbatim
    let verbatim_atom = resolve_verbatim(verbatim_expr)?;
    // Throw an error if the verbatim atom is not actually an atom
    assert_expression_atomicity(&verbatim_atom, true)?;
    Ok(())
}
