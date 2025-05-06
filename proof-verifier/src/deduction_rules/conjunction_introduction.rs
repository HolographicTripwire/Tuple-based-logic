use shared::{entity::BuiltinEntity, proposition::{Proposition, PropositionTerm}};

use crate::VerificationError;

/// Verify that the propositions and the conclusion form a valid instance of conjunction
pub fn verify_conjunction_introduction(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    // Throw an error if there are not two assumptions
    if assumptions.len() != 2 { Err(VerificationError::InvalidStepSpecification) }
    // Throw an error if there are not three entities in the conclusion
    else if conclusion.len() != 3 { Err(VerificationError::InvalidStepSpecification) }

    // Panic if the conclusion does not have a first term
    else if let PropositionTerm::Entity(entity_id) = conclusion.get_term(0).expect("Proposition had no first term") {
        // Throw an error if the head is not [BuiltInEntity::Conjunction]
        if entity_id != BuiltinEntity::Conjunction.into() { Err(VerificationError::InvalidStepSpecification) }
        else {
            // Throw an error if a proposition cannot be constructed by the first term
            if let Ok(conjunction_left) = conclusion.proposition_from_term(1) { 
                // Throw an error if the assumptions do not contain the left side of the conjunction
                if assumptions.get(0).expect("No first assumption found") == &conjunction_left { return Err(VerificationError::InvalidStepSpecification) }
            } else { return Err(VerificationError::InvalidStepSpecification) }

            // Throw an error if a proposition cannot be constructed by the second term
            if let Ok(conjunction_right) = conclusion.proposition_from_term(2) { 
                // Throw an error if the assumptions do not contain the right side of the conjunction
                if assumptions.get(1).expect("No second assumption found") == &conjunction_right { return Err(VerificationError::InvalidStepSpecification) }
            } else { return Err(VerificationError::InvalidStepSpecification) }

            // If none of the errors were triggered, then this step was successfully verified
            Ok(())
        }
    // Throw an error if the head is not a [PropositionTerm::Entity]
    } else { Err(VerificationError::InvalidStepSpecification) }
}
