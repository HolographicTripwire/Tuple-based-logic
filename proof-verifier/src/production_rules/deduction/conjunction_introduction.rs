use shared::{atom::BuiltInAtom, proposition::Proposition};

use crate::{production_rules::tuple_or_error, ProofValidationError};

/// Verify that the assumptions and the conclusion form a valid instance of conjunction introduction ("a" and "b" entails "a and b")
pub fn verify_conjunction_introduction(assumptions: &Vec<Proposition>, conclusions: &Vec<Proposition>) -> Result<(),ProofValidationError> {
    // Throw an error if there is not exactly one conclusion
    let [conclusion] = conclusions.as_slice() else { return Err(ProofValidationError::InvalidStepSpecification) };
    // Throw an error if there are not two assumptions
    let [assumption_left, assumption_right] = assumptions.as_slice() else { return Err(ProofValidationError::InvalidStepSpecification) };

    // Throw an error if there are not three terms in the conclusion
    let [conjunction_head, conjunction_left, conjunction_right] = tuple_or_error::prop_as_slice(conclusion)? else { return Err(ProofValidationError::InvalidStepSpecification) };

    // Throw an error if the head of the conjunction is incorrect
    if conjunction_head != &BuiltInAtom::Conjunction.into() { return Err(ProofValidationError::InvalidStepSpecification) }
    // Throw an error if the left half of the conjunction is incorrect
    if conjunction_left != &assumption_left.0 { return Err(ProofValidationError::InvalidStepSpecification) }
    // Throw an error if the right half of the conjunction is incorrect
    if conjunction_right != &assumption_right.0 { return Err(ProofValidationError::InvalidStepSpecification) }
    
    // If none of the errors were triggered, then this step was successfully verified
    return Ok(())
}
