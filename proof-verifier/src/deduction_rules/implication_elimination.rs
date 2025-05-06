use shared::{entity::BuiltinEntity, proposition::{Proposition, PropositionTerm}};

use crate::VerificationError;

pub fn verify_implication_elimination(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    
}