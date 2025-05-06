use shared::{entity::BuiltinEntity, proposition::{Proposition, PropositionTerm}};

use crate::VerificationError;

pub fn verify_universal_instantiation(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    Ok(())
}