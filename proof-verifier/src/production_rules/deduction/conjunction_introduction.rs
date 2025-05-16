use shared::{atom::BuiltInAtom, proposition::Proposition};

use crate::VerificationError;

/// Verify that the propositions and the conclusion form a valid instance of conjunction introduction
pub fn verify_conjunction_introduction(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    // Throw an error if there are not two assumptions
    let [assumption_left, assumption_right] = assumptions.as_slice() else { return Err(VerificationError::InvalidStepSpecification) };

    // Throw an error if the conclusion is not a tuple
    let conclusion_terms = conclusion.0.as_tuple().or(Err(VerificationError::InvalidStepSpecification))?;
    // Throw an error if there are not three terms in the conclusion
    let [conjunction_head, conjunction_left, conjunction_right] = conclusion_terms.as_slice() else { return Err(VerificationError::InvalidStepSpecification) };

    // Throw an error if the head of the conjunction is incorrect
    if conjunction_head != &BuiltInAtom::Conjunction.into() { return Err(VerificationError::InvalidStepSpecification) }
    // Throw an error if the left half of the conjunction is incorrect
    if conjunction_left != &assumption_left.0 { return Err(VerificationError::InvalidStepSpecification) }
    // Throw an error if the right half of the conjunction is incorrect
    if conjunction_right != &assumption_right.0 { return Err(VerificationError::InvalidStepSpecification) }
    
    // If none of the errors were triggered, then this step was successfully verified
    return Ok(())
}
