use shared::{atom::BuiltInAtom, proposition::Proposition};

use crate::{production_rules::tuple_or_error, ProofVerificationError};

/// Verify that the assumptions and the conclusion form a valid instance of implication elimination ("a" and "a implies b" entails "b")
pub fn verify_implication_elimination(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(), ProofVerificationError> {
    // Throw an error if there are not two assumptions
    let [prior, implication] = assumptions.as_slice() else { return Err(ProofVerificationError::InvalidStepSpecification) };
    
    // Throw an error if the implication does not contain three terms
    let [implication_head, antecedent, consequent] = tuple_or_error::prop_as_slice(implication)? else { return Err(ProofVerificationError::InvalidStepSpecification) };

    // Throw an error if the head of the implication is incorrect
    if implication_head != &BuiltInAtom::Implication.into() { return Err(ProofVerificationError::InvalidStepSpecification) }
    // Throw an error if the left half of the implication is incorrect
    if antecedent != &prior.0 { return Err(ProofVerificationError::InvalidStepSpecification) }
    // Throw an error if the right half of the implication is incorrect
    if consequent != &conclusion.0 { return Err(ProofVerificationError::InvalidStepSpecification) }

    return Ok(());
}
