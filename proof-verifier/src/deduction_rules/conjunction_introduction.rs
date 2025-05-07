use shared::{entity::BuiltinEntity, proposition::{Proposition, PropositionTerm}};

use crate::VerificationError;

/// Verify that the propositions and the conclusion form a valid instance of conjunction
pub fn verify_conjunction_introduction(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    // Throw an error if there are not two assumptions
    if assumptions.len() != 2 { return Err(VerificationError::InvalidStepSpecification) }
    // Throw an error if there are not three entities in the conclusion
    if conclusion.len() != 3 { return Err(VerificationError::InvalidStepSpecification) }

    verify_conjunction_head()?;
    verify_conjunction_left()?;
    verify_conjunction_right()?;

    // If none of the errors were triggered, then this step was successfully verified
    return Ok(())
}

fn verify_conjunction_head() -> Result<(),VerificationError> {
    // Panic if the conclusion does not have a first term
    if let PropositionTerm::Entity(entity_id) = conclusion.get_term(0).expect("Conclusion had no first term") {
        // Throw an error if the head is not [BuiltInEntity::Conjunction]
        if entity_id != BuiltinEntity::Conjunction.into() { return Err(VerificationError::InvalidStepSpecification) }
    // Throw an error if the head is not a [PropositionTerm::Entity]
    } else { return Err(VerificationError::InvalidStepSpecification) }

    return Ok(());
}

fn verify_conjunction_left() -> Result<(),VerificationError> {
    // Throw an error if a proposition cannot be constructed by the first term
    if let Ok(conjunction_left) = conclusion.proposition_from_term(1) { 
        // Panic if there is no first assumption
        if assumptions.get(0).expect("No first assumption found") == &conjunction_left
        // Throw an error if the assumptions do not contain the left side of the conjunction
            { return Err(VerificationError::InvalidStepSpecification) }
    } else { return Err(VerificationError::InvalidStepSpecification) }

    return Ok(());
}

fn verify_conjunction_right() -> Result<(),VerificationError> {
    // Throw an error if a proposition cannot be constructed by the second term
    if let Ok(conjunction_right) = conclusion.proposition_from_term(2) { 
        // Panic if there is no second assumption
        if assumptions.get(1).expect("No second assumption found") == &conjunction_right
        // Throw an error if the assumptions do not contain the right side of the conjunction
            { return Err(VerificationError::InvalidStepSpecification) }
    } else { return Err(VerificationError::InvalidStepSpecification) }

    Ok(())
}