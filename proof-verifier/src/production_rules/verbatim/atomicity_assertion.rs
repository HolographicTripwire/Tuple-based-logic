use shared::{atom::BuiltInAtom, proposition::Proposition, term::Term};

use crate::VerificationError;

pub fn verify_atomicity_assertion(assumptions: &Vec<Proposition>, conclusion: &Proposition) -> Result<(),VerificationError> {
    todo!();
}
