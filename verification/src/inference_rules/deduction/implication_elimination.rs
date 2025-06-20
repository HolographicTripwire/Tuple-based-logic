use tbl_structures::{atoms::BuiltInAtom, propositions::Proposition};

use crate::{inference_rules::TUPLE_OR_ERROR, ProofValidationError};

/// Verify that the assumptions and the conclusion form a valid instance of implication elimination ("a" and "a implies b" entails "b")
pub fn verify_implication_elimination(assumptions: &[Proposition], conclusions: &[Proposition]) -> Result<(), ProofValidationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = conclusions else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw an error if there are not two assumptions
    let [prior, implication] = assumptions else { return Err(ProofValidationError::InvalidStepSpecification) };
    
    // Throw an error if the implication does not contain three expressions
    let [implication_head, antecedent, consequent] = TUPLE_OR_ERROR.as_slice(implication)? else { return Err(ProofValidationError::InvalidStepSpecification) };

    // Throw an error if the head of the implication is incorrect
    if implication_head != &BuiltInAtom::Implication.into() { return Err(ProofValidationError::InvalidStepSpecification) }
    // Throw an error if the left half of the implication is incorrect
    if antecedent != prior { return Err(ProofValidationError::InvalidStepSpecification) }
    // Throw an error if the right half of the implication is incorrect
    if consequent != conclusion { return Err(ProofValidationError::InvalidStepSpecification) }

    return Ok(());
}
