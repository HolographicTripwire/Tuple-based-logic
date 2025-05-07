use shared::{entity::BuiltinEntity, proposition::{Proposition, PropositionTerm}};

use crate::VerificationError;

pub fn verify_implication_elimination(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(), VerificationError> {
    // Throw an error if there are not two assumptions
    if assumptions.len() != 2 { return Err(VerificationError::InvalidStepSpecification) }
    
    // Panic if there is no second assumption
    let implication = assumptions.get(1).expect("No second assumption found");
    
    verify_implication_head(implication)?;
    verify_implication_antecedent(implication, assumptions)?;
    verify_implication_consequent(implication, conclusion)?;

    return Ok(());
}

fn verify_implication_head(implication: &Proposition) -> Result<(),VerificationError> {
    // Panic if implication has no first term
    if let PropositionTerm::Entity(entity_id) = implication.get_term(0).expect("Conclusion had no first term") {
        // Throw an error if the head is not [BuiltInEntity::Conjunction]
        if entity_id != BuiltinEntity::Implication.into() { return Err(VerificationError::InvalidStepSpecification) }
    // Throw an error if the head is not a [PropositionTerm::Entity]
    } else { return Err(VerificationError::InvalidStepSpecification) }

    return Ok(())
}

fn verify_implication_antecedent(implication: &Proposition, assumptions: &Vec<Proposition>) -> Result<(),VerificationError> {
    if let Ok(expected_antecedent) = implication.proposition_from_term(1) {
        // Panic if there is no first assumption
        let antecedent = assumptions.get(0).expect("No first assumption found");
        // Throw an error if the antecedent and expected antecedent are not equal
        if antecedent != &expected_antecedent { return Err(VerificationError::InvalidStepSpecification) }
    // Throw an error if the antecedent is not a valid proposition
    } else { return Err(VerificationError::InvalidStepSpecification) }

    return Ok(())
}

fn verify_implication_consequent(implication: &Proposition, conclusion: &Proposition) -> Result<(),VerificationError> {
    if let Ok(expected_consequent) = implication.proposition_from_term(2) {
        // Throw an error if the antecedent and expected antecedent are not equal
        if conclusion != &expected_consequent { return Err(VerificationError::InvalidStepSpecification) }
    // Throw an error if the consequent is not a valid proposition
    } else { return Err(VerificationError::InvalidStepSpecification) }

    return Ok(())
}