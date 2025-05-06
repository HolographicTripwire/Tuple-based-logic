use shared::{entity::BuiltinEntity, proposition::{Proposition, PropositionTerm}};

use crate::VerificationError;

pub fn verify_tuple_appendation(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    Ok(())
}